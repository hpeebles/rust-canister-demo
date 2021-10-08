mod env;

use crate::env::{CanisterEnv, EmptyEnv, Environment};
use candid::CandidType;
use ic_cdk_macros::*;
use serde::Deserialize;
use std::cell::RefCell;

type TimestampMillis = u64;

thread_local! {
    // If RuntimeState doesn't implement Default you can wrap it in an Option instead
    static RUNTIME_STATE: RefCell<RuntimeState> = RefCell::default();
}

struct RuntimeState {
    pub env: Box<dyn Environment>,
    pub data: Data,
}

impl Default for RuntimeState {
    fn default() -> Self {
        RuntimeState {
            env: Box::new(EmptyEnv {}),
            data: Data::default(),
        }
    }
}

#[derive(CandidType, Deserialize, Default)]
struct Data {
    items: Vec<TodoItem>
}

#[derive(CandidType, Deserialize, Clone)]
struct TodoItem {
    id: u32,
    added: TimestampMillis,
    name: String,
    done: bool,
}

#[init]
fn init() {
    let env = Box::new(CanisterEnv::new());
    let data = Data::default();
    let runtime_state = RuntimeState { env, data };

    RUNTIME_STATE.with(|state| *state.borrow_mut() = runtime_state);
}

#[pre_upgrade]
fn pre_upgrade() {
    RUNTIME_STATE.with(|state| ic_cdk::storage::stable_save((&state.borrow().data,)).unwrap());
}

#[post_upgrade]
fn post_upgrade() {
    let env = Box::new(CanisterEnv::new());
    let (data,): (Data,) = ic_cdk::storage::stable_restore().unwrap();
    let runtime_state = RuntimeState { env, data };

    RUNTIME_STATE.with(|state| *state.borrow_mut() = runtime_state);
}

#[update]
fn add(name: String) -> u32 {
    RUNTIME_STATE.with(|state| add_impl(name, &mut state.borrow_mut()))
}

fn add_impl(name: String, runtime_state: &mut RuntimeState) -> u32 {
    let id = runtime_state.env.random_u32();
    let now = runtime_state.env.now();

    runtime_state.data.items.push(TodoItem {
        id,
        added: now,
        name,
        done: false,
    });

    id
}

#[query]
fn get(done_filter: Option<bool>) -> Vec<TodoItem> {
    RUNTIME_STATE.with(|state| get_impl(done_filter, &state.borrow()))
}

fn get_impl(done_filter: Option<bool>, runtime_state: &RuntimeState) -> Vec<TodoItem> {
    runtime_state.data.items.iter().filter(|i| done_filter.map_or(true, |d| i.done == d)).cloned().collect()
}

#[update]
fn mark_done(id: u32) -> bool {
    RUNTIME_STATE.with(|state| mark_done_impl(id, &mut state.borrow_mut()))
}

fn mark_done_impl(id: u32, runtime_state: &mut RuntimeState) -> bool {
    if let Some(item) = runtime_state.data.items.iter_mut().find(|i| i.id == id) {
        item.done = true;
        true
    } else {
        false
    }
}
