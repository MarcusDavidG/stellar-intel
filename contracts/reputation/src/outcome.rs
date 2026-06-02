use soroban_sdk::{contracttype, Address, BytesN, Env, Vec, Symbol};

#[derive(Clone)]
#[contracttype]
pub struct OutcomeRecord {
    pub outcome_hash: BytesN<32>,
    pub settle_seconds: u32,
    pub success: bool,
    pub timestamp: u64,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    Outcomes(Address), // Maps Anchor Address to Vec<OutcomeRecord>
}

pub fn read_outcomes(env: &Env, anchor: &Address) -> Vec<OutcomeRecord> {
    let key = DataKey::Outcomes(anchor.clone());
    env.storage().persistent().get(&key).unwrap_or(Vec::new(env))
}

pub fn write_outcome(env: &Env, anchor: &Address, record: OutcomeRecord) {
    let key = DataKey::Outcomes(anchor.clone());
    let mut outcomes = read_outcomes(env, anchor);
    outcomes.push_back(record);
    env.storage().persistent().set(&key, &outcomes);
}