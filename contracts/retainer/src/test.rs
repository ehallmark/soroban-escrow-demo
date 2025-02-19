#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::Address as _, // AuthorizedFunction, AuthorizedInvocation},
    token, Address, Env, vec
};

use token::Client as TokenClient;
use token::StellarAssetClient as TokenAdminClient;

fn create_token_contract<'a>(e: &Env, admin: &Address) -> (TokenClient<'a>, TokenAdminClient<'a>) {
    let sac = e.register_stellar_asset_contract_v2(admin.clone());
    (   
        token::Client::new(e, &sac.address()),
        token::StellarAssetClient::new(e, &sac.address()),
    )
}

fn create_retainer_contract<'a>(e: &Env) -> ContractClient<'a> {
    ContractClient::new(e, &e.register(Contract, ()))
}

fn str<'a>(e: &'a Env, s: &'a str) -> String {
    String::from_str(e, s)
}

struct RetainerTest<'a> {
    env: Env,
    retainor: Address,
    retainee: Address,
    token: TokenClient<'a>,
    contract: ContractClient<'a>,
}


impl<'a> RetainerTest<'a> {
    fn setup() -> Self {
        let env = Env::default();
        env.mock_all_auths();

        let retainor = Address::generate(&env);
        let retainee = Address::generate(&env);
        let token_admin = Address::generate(&env);
        
        let (token, token_admin_client) = create_token_contract(&env, &token_admin);
        token_admin_client.mint(&retainor, &10_000);

        let contract = create_retainer_contract(&env);
        
        // Set retainee info
        contract.set_retainee_info(&retainee, 
                                    &str(&env, "Alice"), 
                                    &vec![&env, retainor.clone()]);
        
        // Set retainor info
        contract.set_retainor_info(&retainor, 
                                    &str(&env, "Bob"), 
                                    &vec![&env, retainee.clone()]);

        RetainerTest {
            env,
            retainor,
            retainee,
            token,
            contract,
        }
    }
}

#[test]
fn test_setup() {
    let RetainerTest { env, retainor, retainee, token, contract } = RetainerTest::setup();

    // Verify retainor info
    assert_eq!(
        contract.retainee_info(&retainee),
        RetaineeInfo {
            name: str(&env, "Alice"),
            retainors: vec![&env, retainor.clone()],
        }
    );

    // Verify retainee info
    assert_eq!(
        contract.retainor_info(&retainor),
        RetainorInfo {
            name: str(&env, "Bob"),
            retainees: vec![&env, retainee.clone()],
        }
    );

    // Check initial token balances
    assert_eq!(token.balance(&retainor), 10_000);
    assert_eq!(token.balance(&retainee), 0);
}


#[test]
#[should_panic(expected = "No retained balance")]
fn test_submit_bill_without_retained_balance() {
    let RetainerTest { env, retainor, retainee, contract, .. } = RetainerTest::setup();

    contract.submit_bill(&retainor, 
                            &retainee, 
                            &100, 
                            &str(&env, "Bill 1"), 
                            &str(&env, "2021-01-01T00:00:00Z"));
}

#[test]
#[should_panic(expected = "Token mismatch")]
fn test_add_retainer_balance_token_mismatch() {
    let RetainerTest { env, retainor, retainee, contract, token } = RetainerTest::setup();

    contract.add_retainer_balance(&retainor, &retainee, &99, &token.address);
    contract.add_retainer_balance(&retainor, &retainee, &99, &Address::generate(&env));
}

#[test]
fn test_add_retainer_balance_twice() {
    let RetainerTest { retainor, retainee, contract, token, .. } = RetainerTest::setup();

    contract.add_retainer_balance(&retainor, &retainee, &99, &token.address);
    contract.add_retainer_balance(&retainor, &retainee, &100, &token.address);

    assert_eq!(
        contract.retainer_balance(&retainor, &retainee),
        Some(RetainerBalance {
            amount: 199,
            token: token.address.clone(),
        })
    );
}

#[test]
#[should_panic(expected = "Insufficient retained balance")]
fn test_submit_bill_insufficient_retained_balance() {
    let RetainerTest { env, retainor, retainee, contract, token } = RetainerTest::setup();

    contract.add_retainer_balance(&retainor, &retainee, &99, &token.address);
    assert_eq!(
        contract.retainer_balance(&retainor, &retainee),
        Some(RetainerBalance {
            amount: 99,
            token: token.address.clone(),
        })
    );

    contract.submit_bill(&retainor, 
                            &retainee, 
                            &100, 
                            &str(&env, "Bill 1"), 
                            &str(&env, "2021-01-01T00:00:00Z"));
}

#[test]
#[should_panic(expected = "Pending payment already exists")]
fn test_submit_bill_pending_payment_exists() {
    let RetainerTest { env, retainor, retainee, contract, token } = RetainerTest::setup();

    contract.add_retainer_balance(&retainor, &retainee, &100, &token.address);
    assert_eq!(
        contract.retainer_balance(&retainor, &retainee),
        Some(RetainerBalance {
            amount: 100,
            token: token.address.clone(),
        })
    );

    contract.submit_bill(&retainor, 
                            &retainee, 
                            &50, 
                            &str(&env, "Bill 1"), 
                            &str(&env, "2021-01-01T00:00:00Z"));
    assert_eq!(
        contract.view_bill(&retainor, &retainee),
        Some(Bill {
            amount: 50,
            notes: str(&env, "Bill 1"),
            date: str(&env, "2021-01-01T00:00:00Z"),
            token: token.address.clone(),
        })
    );

    // this should panic
    contract.submit_bill(&retainor, 
                            &retainee, 
                            &49, 
                            &str(&env, "Bill 2"),
                            &str(&env, "2021-01-01T00:00:00Z"));
}

#[test]
fn test_resubmit_bill() {
    let RetainerTest { env, retainor, retainee, contract, token } = RetainerTest::setup();

    contract.add_retainer_balance(&retainor, &retainee, &100, &token.address);
    assert_eq!(
        contract.retainer_balance(&retainor, &retainee),
        Some(RetainerBalance {
            amount: 100,
            token: token.address.clone(),
        })
    );

    contract.submit_bill(&retainor, &retainee, &50, &str(&env, "Bill 1"), &str(&env, "2021-01-01T00:00:00Z"));
    assert_eq!(
        contract.view_bill(&retainor, &retainee),
        Some(Bill {
            amount: 50,
            notes: str(&env, "Bill 1"),
            date: str(&env, "2021-01-01T00:00:00Z"),
            token: token.address.clone(),
        })
    );

    contract.unsubmit_bill(&retainor, &retainee);
    assert_eq!(contract.view_bill(&retainor, &retainee), None);

    contract.submit_bill(&retainor, 
                            &retainee, 
                            &49, 
                            &str(&env, "Bill 2"), 
                            &str(&env, "2021-01-01T00:00:00Z"));
    assert_eq!(
        contract.view_bill(&retainor, &retainee),
        Some(Bill {
            amount: 49,
            notes: str(&env, "Bill 2"),
            date: str(&env, "2021-01-01T00:00:00Z"),
            token: token.address.clone(),
        })
    );
}


#[test]
#[should_panic(expected = "No pending payment")]
fn test_resolve_bill_without_pending_payment() {
    let RetainerTest { env, retainor, retainee, contract, token } = RetainerTest::setup();

    contract.add_retainer_balance(&retainor, &retainee, &100, &token.address);
    assert_eq!(
        contract.retainer_balance(&retainor, &retainee),
        Some(RetainerBalance {
            amount: 100,
            token: token.address.clone(),
        })
    );

    contract.resolve_bill(&retainor, 
                            &retainee, 
                            &ApprovalStatus::Approved,
                            &str(&env, "Bill 1 resolved"),
                            &str(&env, "2021-01-01T00:00:00Z"));
    
}

#[test]
fn test_resolve_bill_approved() {
    let RetainerTest { env, retainor, retainee, contract, token } = RetainerTest::setup();

    contract.add_retainer_balance(&retainor, &retainee, &100, &token.address);
    assert_eq!(
        contract.retainer_balance(&retainor, &retainee),
        Some(RetainerBalance {
            amount: 100,
            token: token.address.clone(),
        })
    );

    assert_eq!(token.balance(&retainor), 9_900);
    assert_eq!(token.balance(&contract.address), 100);
    assert_eq!(token.balance(&retainee), 0);

    contract.submit_bill(&retainor, 
                            &retainee, 
                            &49, 
                            &str(&env, "Bill 1"), 
                            &str(&env, "2021-01-01T00:00:00Z"));

    contract.resolve_bill(&retainor, 
                            &retainee, 
                            &ApprovalStatus::Approved,
                            &str(&env, "Bill 1 resolved"),
                            &str(&env, "2021-01-01T00:00:00Z"));
    
    // verify balances
    assert_eq!(token.balance(&retainor), 9_900);
    assert_eq!(token.balance(&retainee), 49);
    assert_eq!(token.balance(&contract.address), 51);
    // check final state
    assert_eq!(contract.view_bill(&retainor, &retainee), None);
    assert_eq!(
        contract.retainer_balance(&retainor, &retainee),
        Some(RetainerBalance {
            amount: 51,
            token: token.address.clone(),
        })
    );
    assert_eq!(
        contract.view_receipt_history(&retainor, 
                                        &retainee, 
                                        &0),
        vec![&env, Receipt {
            bill: Bill {
                amount: 49,
                notes: str(&env, "Bill 1"),
                date: str(&env, "2021-01-01T00:00:00Z"),
                token: token.address.clone(),
            },
            notes: str(&env, "Bill 1 resolved"),
            date: str(&env, "2021-01-01T00:00:00Z"),
            status: ApprovalStatus::Approved,
        }]
    );
    assert_eq!(contract.history_index(&retainor, &retainee), 1);
    assert_eq!(
        contract.view_receipt(&retainor, &retainee, &1),
        Some(Receipt {
            bill: Bill {
                amount: 49,
                notes: str(&env, "Bill 1"),
                date: str(&env, "2021-01-01T00:00:00Z"),
                token: token.address.clone(),
            },
            notes: str(&env, "Bill 1 resolved"),
            date: str(&env, "2021-01-01T00:00:00Z"),
            status: ApprovalStatus::Approved,
        })
    );
}


#[test]
fn test_resolve_bill_denied() {
    let RetainerTest { env, retainor, retainee, contract, token } = RetainerTest::setup();

    contract.add_retainer_balance(&retainor, &retainee, &100, &token.address);
    assert_eq!(
        contract.retainer_balance(&retainor, &retainee),
        Some(RetainerBalance {
            amount: 100,
            token: token.address.clone(),
        })
    );

    assert_eq!(token.balance(&retainor), 9_900);
    assert_eq!(token.balance(&contract.address), 100);
    assert_eq!(token.balance(&retainee), 0);

    contract.submit_bill(&retainor, 
                            &retainee, 
                            &49, 
                            &str(&env, "Bill 1"), 
                            &str(&env, "2021-01-01T00:00:00Z"));

    contract.resolve_bill(&retainor, 
                            &retainee, 
                            &ApprovalStatus::Denied,
                            &str(&env, "Bill 1 resolved"),
                            &str(&env, "2021-01-01T00:00:00Z"));
    
    // verify balances
    assert_eq!(token.balance(&retainor), 9_900);
    assert_eq!(token.balance(&retainee), 0);
    assert_eq!(token.balance(&contract.address), 100);
    // check final state
    assert_eq!(contract.view_bill(&retainor, &retainee), None);
    assert_eq!(
        contract.retainer_balance(&retainor, &retainee),
        Some(RetainerBalance {
            amount: 100,
            token: token.address.clone(),
        })
    );
    assert_eq!(
        contract.view_receipt_history(&retainor, 
                                        &retainee, 
                                        &0),
        vec![&env, Receipt {
            bill: Bill {
                amount: 49,
                notes: str(&env, "Bill 1"),
                date: str(&env, "2021-01-01T00:00:00Z"),
                token: token.address.clone(),
            },
            notes: str(&env, "Bill 1 resolved"),
            date: str(&env, "2021-01-01T00:00:00Z"),
            status: ApprovalStatus::Denied,
        }]
    );
    assert_eq!(contract.history_index(&retainor, &retainee), 1);
    assert_eq!(
        contract.view_receipt(&retainor, &retainee, &1),
        Some(Receipt {
            bill: Bill {
                amount: 49,
                notes: str(&env, "Bill 1"),
                date: str(&env, "2021-01-01T00:00:00Z"),
                token: token.address.clone(),
            },
            notes: str(&env, "Bill 1 resolved"),
            date: str(&env, "2021-01-01T00:00:00Z"),
            status: ApprovalStatus::Denied,
        })
    );
}


#[test]
fn test_multiple_bills() {
    let RetainerTest { env, retainor, retainee, contract, token } = RetainerTest::setup();

    contract.add_retainer_balance(&retainor, &retainee, &100, &token.address);

    contract.submit_bill(&retainor, 
                            &retainee, 
                            &50, 
                            &str(&env, "Bill 1"), 
                            &str(&env, "2021-01-01T00:00:00Z"));

    contract.resolve_bill(&retainor, 
                            &retainee, 
                            &ApprovalStatus::Approved,
                            &str(&env, "Bill 1 resolved"),
                            &str(&env, "2021-01-01T00:00:00Z"));
    
    contract.submit_bill(&retainor, 
                            &retainee, 
                            &25, 
                            &str(&env, "Bill 2"), 
                            &str(&env, "2021-01-01T00:00:00Z"));

    contract.resolve_bill(&retainor, 
                            &retainee, 
                            &ApprovalStatus::Approved,
                            &str(&env, "Bill 2 resolved"),
                            &str(&env, "2021-01-01T00:00:00Z"));
    
    // verify balances
    assert_eq!(token.balance(&retainor), 9_900);
    assert_eq!(token.balance(&retainee), 75);
    assert_eq!(token.balance(&contract.address), 25);
    // check final state
        assert_eq!(
        contract.view_bill(&retainor, &retainee),
        None
    );
    assert_eq!(
        contract.retainer_balance(&retainor, &retainee),
        Some(RetainerBalance {
            amount: 25,
            token: token.address.clone(),
        })
    );
    assert_eq!(
        contract.view_receipt_history(&retainor, 
                                        &retainee, 
                                        &0),
        vec![&env, 
            Receipt {
                bill: Bill {
                    amount: 50,
                    notes: str(&env, "Bill 1"),
                    date: str(&env, "2021-01-01T00:00:00Z"),
                    token: token.address.clone(),
                },
                notes: str(&env, "Bill 1 resolved"),
                date: str(&env, "2021-01-01T00:00:00Z"),
                status: ApprovalStatus::Approved,
            }, 
            Receipt {
                bill: Bill {
                    amount: 25,
                    notes: str(&env, "Bill 2"),
                    date: str(&env, "2021-01-01T00:00:00Z"),
                    token: token.address.clone(),
                },
                notes: str(&env, "Bill 2 resolved"),
                date: str(&env, "2021-01-01T00:00:00Z"),
                status: ApprovalStatus::Approved,
            }]
    );
    assert_eq!(contract.history_index(&retainor, &retainee), 2);
    assert_eq!(
        contract.view_receipt(&retainor, &retainee, &2),
        Some(Receipt {
            bill: Bill {
                amount: 25,
                notes: str(&env, "Bill 2"),
                date: str(&env, "2021-01-01T00:00:00Z"),
                token: token.address.clone(),
            },
            notes: str(&env, "Bill 2 resolved"),
            date: str(&env, "2021-01-01T00:00:00Z"),
            status: ApprovalStatus::Approved,
        })
    );
}


#[test]
fn test_multiple_retainees() {
    let RetainerTest { env, retainor, retainee, contract, token } = RetainerTest::setup();
    let retainee2 = Address::generate(&env);

    contract.add_retainer_balance(&retainor, &retainee, &100, &token.address);
    contract.add_retainer_balance(&retainor, &retainee2, &200, &token.address);

    // verify balances
    assert_eq!(token.balance(&retainor), 9_700);
    assert_eq!(token.balance(&retainee), 0);
    assert_eq!(token.balance(&retainee2), 0);
    assert_eq!(token.balance(&contract.address), 300);

    contract.submit_bill(&retainor, 
                            &retainee, 
                            &50, 
                            &str(&env, "R1 Bill 1"), 
                            &str(&env, "2021-01-01T00:00:00Z"));

    contract.resolve_bill(&retainor, 
                            &retainee, 
                            &ApprovalStatus::Approved,
                            &str(&env, "R1 Bill 1 resolved"),
                            &str(&env, "2021-01-01T00:00:00Z"));
    
    contract.submit_bill(&retainor, 
                            &retainee2, 
                            &25, 
                            &str(&env, "R2 Bill 1"), 
                            &str(&env, "2021-01-01T00:00:00Z"));

    contract.resolve_bill(&retainor, 
                            &retainee2, 
                            &ApprovalStatus::Approved,
                            &str(&env, "R2 Bill 1 resolved"),
                            &str(&env, "2021-01-01T00:00:00Z"));
    
    // verify balances
    assert_eq!(token.balance(&retainor), 9_700);
    assert_eq!(token.balance(&retainee), 50);
    assert_eq!(token.balance(&retainee2), 25);
    assert_eq!(token.balance(&contract.address), 225);

    // check final state
    assert_eq!(contract.view_bill(&retainor, &retainee), None);
    assert_eq!(contract.view_bill(&retainor, &retainee2), None);
    assert_eq!(
        contract.retainer_balance(&retainor, &retainee),
        Some(RetainerBalance {
            amount: 50,
            token: token.address.clone(),
        })
    );
    assert_eq!(
        contract.retainer_balance(&retainor, &retainee2),
        Some(RetainerBalance {
            amount: 175,
            token: token.address.clone(),
        })
    );
    assert_eq!(
        contract.view_receipt_history(&retainor, 
                                        &retainee, 
                                        &0).len(),
        1
    );
    assert_eq!(
        contract.view_receipt_history(&retainor, 
                                        &retainee2, 
                                        &0).len(),
        1
    );
    assert_eq!(contract.history_index(&retainor, &retainee), 1);
    assert_eq!(contract.history_index(&retainor, &retainee2), 1);
    assert_eq!(
        contract.view_receipt(&retainor, &retainee, &1),
        Some(Receipt {
            bill: Bill {
                amount: 50,
                notes: str(&env, "R1 Bill 1"),
                date: str(&env, "2021-01-01T00:00:00Z"),
                token: token.address.clone(),
            },
            notes: str(&env, "R1 Bill 1 resolved"),
            date: str(&env, "2021-01-01T00:00:00Z"),
            status: ApprovalStatus::Approved,
        })
    );
    assert_eq!(
        contract.view_receipt(&retainor, &retainee2, &1),
        Some(Receipt {
            bill: Bill {
                amount: 25,
                notes: str(&env, "R2 Bill 1"),
                date: str(&env, "2021-01-01T00:00:00Z"),
                token: token.address.clone(),
            },
            notes: str(&env, "R2 Bill 1 resolved"),
            date: str(&env, "2021-01-01T00:00:00Z"),
            status: ApprovalStatus::Approved,
        })
    );
}


#[test]
fn test_multiple_retainors() {
    let RetainerTest { env, retainor, retainee, contract, token } = RetainerTest::setup();
    
    let retainor2 = Address::generate(&env);
    
    token.transfer(&retainor, &retainor2, &4_000);
    
    assert_eq!(token.balance(&retainor), 6_000);
    assert_eq!(token.balance(&retainor2), 4_000);

    contract.add_retainer_balance(&retainor, &retainee, &100, &token.address);
    contract.add_retainer_balance(&retainor2, &retainee, &200, &token.address);

    // verify balances
    assert_eq!(token.balance(&retainor), 5_900);
    assert_eq!(token.balance(&retainee), 0);
    assert_eq!(token.balance(&retainor2), 3_800);
    assert_eq!(token.balance(&contract.address), 300);

    contract.submit_bill(&retainor, 
                            &retainee, 
                            &50, 
                            &str(&env, "R1 Bill 1"), 
                            &str(&env, "2021-01-01T00:00:00Z"));

    contract.resolve_bill(&retainor, 
                            &retainee, 
                            &ApprovalStatus::Approved,
                            &str(&env, "R1 Bill 1 resolved"),
                            &str(&env, "2021-01-01T00:00:00Z"));
    
    contract.submit_bill(&retainor2, 
                            &retainee, 
                            &25, 
                            &str(&env, "R2 Bill 1"), 
                            &str(&env, "2021-01-01T00:00:00Z"));

    contract.resolve_bill(&retainor2, 
                            &retainee, 
                            &ApprovalStatus::Approved,
                            &str(&env, "R2 Bill 1 resolved"),
                            &str(&env, "2021-01-01T00:00:00Z"));
    
    // verify balances
    assert_eq!(token.balance(&retainor), 5_900);
    assert_eq!(token.balance(&retainor2), 3_800);
    assert_eq!(token.balance(&retainee), 75);
    assert_eq!(token.balance(&contract.address), 225);

    // check final state
    assert_eq!(contract.view_bill(&retainor, &retainee), None);
    assert_eq!(contract.view_bill(&retainor2, &retainee), None);
    assert_eq!(
        contract.retainer_balance(&retainor, &retainee),
        Some(RetainerBalance {
            amount: 50,
            token: token.address.clone(),
        })
    );
    assert_eq!(
        contract.retainer_balance(&retainor2, &retainee),
        Some(RetainerBalance {
            amount: 175,
            token: token.address.clone(),
        })
    );
    assert_eq!(
        contract.view_receipt_history(&retainor, 
                                        &retainee, 
                                        &0).len(),
        1
    );
    assert_eq!(
        contract.view_receipt_history(&retainor2, 
                                        &retainee, 
                                        &0).len(),
        1
    );
    assert_eq!(contract.history_index(&retainor, &retainee), 1);
    assert_eq!(contract.history_index(&retainor2, &retainee), 1);
    assert_eq!(
        contract.view_receipt(&retainor, &retainee, &1),
        Some(Receipt {
            bill: Bill {
                amount: 50,
                notes: str(&env, "R1 Bill 1"),
                date: str(&env, "2021-01-01T00:00:00Z"),
                token: token.address.clone(),
            },
            notes: str(&env, "R1 Bill 1 resolved"),
            date: str(&env, "2021-01-01T00:00:00Z"),
            status: ApprovalStatus::Approved,
        })
    );
    assert_eq!(
        contract.view_receipt(&retainor2, &retainee, &1),
        Some(Receipt {
            bill: Bill {
                amount: 25,
                notes: str(&env, "R2 Bill 1"),
                date: str(&env, "2021-01-01T00:00:00Z"),
                token: token.address.clone(),
            },
            notes: str(&env, "R2 Bill 1 resolved"),
            date: str(&env, "2021-01-01T00:00:00Z"),
            status: ApprovalStatus::Approved,
        })
    );
}