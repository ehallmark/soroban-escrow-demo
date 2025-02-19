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
fn test() {
    let test = RetainerTest::setup();
    let RetainerTest { env, retainor, retainee, token, contract } = test;

    // Check initial token balances
    assert_eq!(
        token.balance(&retainor),
        10_000
    );
    assert_eq!(
        token.balance(&retainee),
        0
    );

    // Set retainee info
    contract.set_retainee_info(&retainee, &String::from_str(&env, "Alice"), &vec![&env, retainor.clone()]);
    assert_eq!(
        contract.retainee_info(&retainee),
        RetaineeInfo {
            name: String::from_str(&env, "Alice"),
            retainors: vec![&env, retainor.clone()],
        }
    );
    // Set retainor info
    contract.set_retainor_info(&retainor, &String::from_str(&env, "Bob"), &vec![&env, retainee.clone()]);
    assert_eq!(
        contract.retainor_info(&retainor),
        RetainorInfo {
            name: String::from_str(&env, "Bob"),
            retainees: vec![&env, retainee.clone()],
        }
    );

}
