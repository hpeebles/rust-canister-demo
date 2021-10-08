use crate::TimestampMillis;
use candid::Principal;
use rand::rngs::StdRng;
use rand::{RngCore, SeedableRng};

pub trait Environment {
    fn now(&self) -> TimestampMillis;
    fn caller(&self) -> Principal;
    fn canister_id(&self) -> Principal;
    fn random_u32(&mut self) -> u32;
}

pub struct CanisterEnv {
    rng: StdRng,
}

impl CanisterEnv {
    pub fn new() -> Self {
        CanisterEnv {
            // Seed the PRNG with the current time.
            rng: {
                let now_millis = ic_cdk::api::time();
                let mut seed = [0u8; 32];
                seed[..8].copy_from_slice(&now_millis.to_be_bytes());
                seed[8..16].copy_from_slice(&now_millis.to_be_bytes());
                seed[16..24].copy_from_slice(&now_millis.to_be_bytes());
                seed[24..32].copy_from_slice(&now_millis.to_be_bytes());
                StdRng::from_seed(seed)
            },
        }
    }
}

impl Environment for CanisterEnv {
    fn now(&self) -> TimestampMillis {
        ic_cdk::api::time()
    }

    fn caller(&self) -> Principal {
        ic_cdk::caller()
    }

    fn canister_id(&self) -> Principal {
        ic_cdk::id()
    }

    fn random_u32(&mut self) -> u32 {
        self.rng.next_u32()
    }
}

pub struct TestEnv {
    pub now: u64,
    pub caller: Principal,
    pub canister_id: Principal,
    pub random_u32: u32,
}

impl Environment for TestEnv {
    fn now(&self) -> u64 {
        self.now
    }

    fn caller(&self) -> Principal {
        self.caller
    }

    fn canister_id(&self) -> Principal {
        self.canister_id
    }

    fn random_u32(&mut self) -> u32 {
        self.random_u32
    }
}

pub struct EmptyEnv {}

impl Environment for EmptyEnv {
    fn now(&self) -> TimestampMillis {
        0
    }

    fn caller(&self) -> Principal {
        Principal::anonymous()
    }

    fn canister_id(&self) -> Principal {
        Principal::anonymous()
    }

    fn random_u32(&mut self) -> u32 {
        0
    }
}
