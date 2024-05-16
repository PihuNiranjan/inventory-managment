use soroban_sdk::{contractimpl, Env};
mod contract;

#[contractimpl]
impl contract::InventoryContract for InventoryContractClient {
    fn add_item(env: Env, item: contract::Item) {
        contract::add_item(env, item);
    }

    fn remove_item(env: Env, sku: String, quantity: u32) {
        contract::remove_item(env, sku, quantity);
    }
}
