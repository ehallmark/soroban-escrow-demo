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
        contract.set_retainee_info(&retainee, &str(&env, "Alice"), &vec![&env, retainor.clone()]);
        
        // Set retainor info
        contract.set_retainor_info(&retainor, &str(&env, "Bob"), &vec![&env, retainee.clone()]);

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
    assert_eq!(
        token.balance(&retainor),
        10_000
    );
    assert_eq!(
        token.balance(&retainee),
        0
    );
}


#[test]
#[should_panic(expected = "No retained balance")]
fn test_submit_bill_without_retained_balance() {
    let RetainerTest { env, retainor, retainee, contract, .. } = RetainerTest::setup();

    contract.submit_bill(&retainor, &retainee, &100, &str(&env, "Bill 1"), &str(&env, "2021-01-01T00:00:00Z"));
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

    contract.submit_bill(&retainor, &retainee, &100, &str(&env, "Bill 1"), &str(&env, "2021-01-01T00:00:00Z"));
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

    // this should panic
    contract.submit_bill(&retainor, &retainee, &49, &str(&env, "Bill 2"), &str(&env, "2021-01-01T00:00:00Z"));
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
    assert_eq!(
        contract.view_bill(&retainor, &retainee),
        None
    );

    contract.submit_bill(&retainor, &retainee, &49, &str(&env, "Bill 2"), &str(&env, "2021-01-01T00:00:00Z"));
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