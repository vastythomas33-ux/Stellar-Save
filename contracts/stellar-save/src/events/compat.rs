use soroban_sdk::{Address, Env, Symbol};

pub(crate) fn emit_group_created(env: &Env, creator: Address, group_id: u64, token: Address) {
    env.events().publish(
        (Symbol::new(env, "GroupCreated"), creator),
        (group_id, token),
    );
}

pub(crate) fn emit_group_updated(env: &Env, group_id: u64, creator: Address) {
    env.events()
        .publish((Symbol::new(env, "GroupUpdated"), group_id), creator);
}

pub(crate) fn emit_group_deleted(env: &Env, group_id: u64, creator: Address) {
    env.events()
        .publish((Symbol::new(env, "GroupDeleted"), group_id), creator);
}

pub(crate) fn emit_group_activated(env: &Env, group_id: u64, timestamp: u64, member_count: u32) {
    env.events().publish(
        (Symbol::new(env, "group_activated"), group_id),
        (timestamp, member_count),
    );
}

pub(crate) fn emit_group_completed_signal(env: &Env, group_id: u64) {
    env.events()
        .publish((Symbol::new(env, "group_completed"),), group_id);
}

pub(crate) fn emit_emergency_withdrawal(env: &Env, group_id: u64, member: Address, amount: i128) {
    env.events().publish(
        (Symbol::new(env, "emergency_withdrawal"),),
        (group_id, member, amount),
    );
}
