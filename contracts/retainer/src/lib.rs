#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, token, Address, Vec, String};

#[contracttype]
pub enum StorageKey {
    // Retainor, Retainee -> RetainerBalance
    Retainer(Address, Address),
    // Retainor, Retainee -> Bill
    PendingPayment(Address, Address),
    // Retainor, Retainee, Index -> Receipt
    History(Address, Address, u32),
    // Retainor, Retainee -> Index
    HistoryIndex(Address, Address),
    // Retainee -> RetaineeInfo
    Retainees(Address),
    // Retainor -> RetainorInfo
    Retainors(Address), 
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[contracttype]
pub enum ApprovalStatus {
    Approved,
    Denied,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[contracttype]
pub struct RetainerBalance {
    pub amount: i128,
    pub token: Address,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[contracttype]
pub struct Bill {
    pub amount: i128,
    pub token: Address,
    pub notes: String,
    pub date: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[contracttype]
pub struct Receipt {
    pub bill: Bill,
    pub notes: String,
    pub date: String,
    pub status: ApprovalStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[contracttype]
pub struct RetaineeInfo {
    pub name: String,
    pub retainors: Vec<Address>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[contracttype]
pub struct RetainorInfo {
    pub name: String,
    pub retainees: Vec<Address>,
}

fn check_positive_amount(amount: i128) {
    if amount <= 0 {
        panic!("Negative amount");
    }
}

fn get_retainer_balance(env: &Env, retainor: &Address, retainee: &Address) -> Option<RetainerBalance> {
    let key = StorageKey::Retainer(retainor.clone(), retainee.clone());
    let res = env.storage()
        .persistent()
        .get::<_, RetainerBalance>(&key);
    res
}

fn get_pending_payment(env: &Env, retainor: &Address, retainee: &Address) -> Option<Bill> {
    env.storage()
        .persistent()
        .get::<_, Bill>(&StorageKey::PendingPayment(retainor.clone(), retainee.clone()))
}

fn get_receipt(env: &Env, retainor: &Address, retainee: &Address, index: u32) -> Option<Receipt> {
    env.storage()
        .persistent()
        .get::<_, Receipt>(&StorageKey::History(retainor.clone(), retainee.clone(), index))
}

fn get_retainee_info(env: &Env, retainee: &Address) -> Option<RetaineeInfo> {
    env.storage()
        .persistent()
        .get::<_, RetaineeInfo>(&StorageKey::Retainees(retainee.clone()))
}

fn get_retainor_info(env: &Env, retainor: &Address) -> Option<RetainorInfo> {
    env.storage()
        .persistent()
        .get::<_, RetainorInfo>(&StorageKey::Retainors(retainor.clone()))
}

fn get_history_index(env: &Env, retainor: &Address, retainee: &Address) -> u32 {
    env.storage()
        .persistent()
        .get::<_, u32>(&StorageKey::HistoryIndex(retainor.clone(), retainee.clone()))
        .unwrap_or(0u32)
}

fn set_retainer_balance(env: &Env, retainor: &Address, retainee: &Address, config: RetainerBalance) {
    env.storage()
        .persistent()
        .set::<_, RetainerBalance>(&StorageKey::Retainer(retainor.clone(), retainee.clone()), &config);
}

fn set_pending_payment(env: &Env, retainor: &Address, retainee: &Address, bill: Bill) {
    env.storage()
        .persistent()
        .set::<_, Bill>(&StorageKey::PendingPayment(retainor.clone(), retainee.clone()), &bill);
}

fn set_receipt(env: &Env, retainor: &Address, retainee: &Address, index: u32, receipt: Receipt) {
    env.storage()
        .persistent()
        .set::<_, Receipt>(&StorageKey::History(retainor.clone(), retainee.clone(), index), &receipt);
}

fn set_retainee_info(env: &Env, retainee: &Address, info: RetaineeInfo) {
    env.storage()
        .persistent()
        .set::<_, RetaineeInfo>(&StorageKey::Retainees(retainee.clone()), &info);
}

fn set_retainor_info(env: &Env, retainor: &Address, info: RetainorInfo) {
    env.storage()
        .persistent()
        .set::<_, RetainorInfo>(&StorageKey::Retainors(retainor.clone()), &info);
}

fn set_history_index(env: &Env, retainor: &Address, retainee: &Address, index: u32) {
    env.storage()
        .persistent()
        .set::<_, u32>(&StorageKey::HistoryIndex(retainor.clone(), retainee.clone()), &index);
}

fn clear_pending_payment(env: &Env, retainor: &Address, retainee: &Address) {
    env.storage()
        .persistent()
        .remove(&StorageKey::PendingPayment(retainor.clone(), retainee.clone()));
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {

    pub fn submit_bill(env: Env, retainor: Address, retainee: Address, amount: i128, notes: String, date: String) {
        retainee.require_auth();
        check_positive_amount(amount);
        match get_pending_payment(&env, &retainor, &retainee) {
            Some(_) => panic!("Pending payment already exists"),
            None => {}
        };
        let retained_balance = match get_retainer_balance(&env, &retainor, &retainee) {
            Some(balance) => balance,
            None => panic!("No retained balance"),
        };
        if retained_balance.amount < amount {
            panic!("Insufficient retained balance");
        }
        let bill = Bill {
            amount,
            notes,
            date,
            token: retained_balance.token.clone(),
        };
        set_pending_payment(&env, &retainor, &retainee, bill);
    }

    pub fn unsubmit_bill(env: Env, retainor: Address, retainee: Address) {
        retainee.require_auth();
        clear_pending_payment(&env, &retainor, &retainee);
    }

    pub fn resolve_bill(env: Env, retainor: Address, retainee: Address, status: ApprovalStatus, notes: String, date: String) {
        retainor.require_auth();
        let bill = match get_pending_payment(&env, &retainor, &retainee) {
            Some(bill) => bill,
            None => panic!("No pending payment"),
        };
        let receipt = Receipt {
            bill: bill.clone(),
            notes,
            date,
            status: status.clone(),
        };
        if status == ApprovalStatus::Approved {
            // send payment
            token::Client::new(&env, &bill.token).transfer(&env.current_contract_address(), &retainee, &bill.amount);
            // update retained balance
            let mut retainer_balance = get_retainer_balance(&env, &retainor, &retainee).unwrap();
            retainer_balance.amount = retainer_balance.amount.checked_sub(bill.amount).unwrap();
            set_retainer_balance(&env, &retainor, &retainee, retainer_balance);
        }
        let index = get_history_index(&env, &retainor, &retainee) + 1;
        set_receipt(&env, &retainor, &retainee, index, receipt);
        set_history_index(&env, &retainor, &retainee, index);
        clear_pending_payment(&env, &retainor, &retainee);
    }

    pub fn view_bill(env: Env, retainor: Address, retainee: Address) -> Option<Bill> {
        get_pending_payment(&env, &retainor, &retainee)
    }

    pub fn view_receipt(env: Env, retainor: Address, retainee: Address, index: u32) -> Option<Receipt> {
        get_receipt(&env, &retainor, &retainee, index)
    }

    pub fn history_index(env: Env, retainor: Address, retainee: Address) -> u32 {
        get_history_index(&env, &retainor, &retainee)
    }

    pub fn view_receipt_history_range(env: Env, retainor: Address, retainee: Address, start: u32, end: u32) -> Vec<Receipt> {
        let mut history = Vec::new(&env);
        for i in start..=end {
            match get_receipt(&env, &retainor, &retainee, i) {
                Some(receipt) => {
                    history.push_back(receipt);
                }
                None => {}
            }
        }
        history
    }

    pub fn view_receipt_history(env: Env, retainor: Address, retainee: Address, limit: u32) -> Vec<Receipt> {
        let index = get_history_index(&env, &retainor, &retainee);
        if index < 1 {
            return Vec::new(&env);
        }
        if limit > 0 && index > limit {
            return Self::view_receipt_history_range(
                env.clone(),
                retainor.clone(),
                retainee.clone(),
                index - limit + 1,
                index);
        } else {
            return Self::view_receipt_history_range(
                env.clone(),
                retainor.clone(),
                retainee.clone(),
                1,
                index);
        }
    }

    pub fn retainer_balance(env: Env, retainor: Address, retainee: Address) -> Option<RetainerBalance> {
        get_retainer_balance(&env, &retainor, &retainee)
    }

    pub fn add_retainer_balance(env: Env, retainor: Address, retainee: Address, additional_amount: i128, token: Address) {
        retainor.require_auth();
        check_positive_amount(additional_amount);
        let mut retainer_balance = get_retainer_balance(&env, &retainor, &retainee).unwrap_or(RetainerBalance {
            amount: 0,
            token: token.clone(),
        });
        if retainer_balance.token != token {
            panic!("Token mismatch");
        }
        retainer_balance.amount = retainer_balance.amount.checked_add(additional_amount).unwrap();
        // transfer tokens to contract
        token::Client::new(&env, &retainer_balance.token).transfer(&retainor, &env.current_contract_address(), &additional_amount);
        // update state
        set_retainer_balance(&env, &retainor, &retainee, retainer_balance);
    }

    pub fn retainee_info(env: Env, retainee: Address) -> RetaineeInfo {
        get_retainee_info(&env, &retainee).unwrap()
    }

    pub fn set_retainee_info(env: Env, retainee: Address, name: String, retainors: Vec<Address>) {
        retainee.require_auth();
        let retainee_info = RetaineeInfo {
            name,
            retainors: retainors,
        };
        set_retainee_info(&env, &retainee, retainee_info);
    }

    pub fn retainor_info(env: Env, retainor: Address) -> RetainorInfo {
        get_retainor_info(&env, &retainor).unwrap()
    }
    
    pub fn set_retainor_info(env: Env, retainor: Address, name: String, retainees: Vec<Address>) {
        retainor.require_auth();
        let retainor_info = RetainorInfo {
            name,
            retainees: retainees,
        };
        set_retainor_info(&env, &retainor, retainor_info);
    }
}

mod test;
