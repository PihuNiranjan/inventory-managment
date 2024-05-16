use soroban_sdk::{contractimpl, contracttype, BytesN, Env, Map, map, vec, Address, symbol};

pub struct InventoryContract;

#[contracttype]
pub enum Error {
    NotAuthorized,
    InsufficientStock,
    ItemNotFound,
}

#[derive(Clone, Debug, Eq, PartialEq, contracttype)]
pub struct Item {
    pub sku: BytesN,
    pub name: BytesN,
    pub quantity: u32,
    pub reorder_threshold: u32,
    pub price: u32, 
}

#[contractimpl]
impl InventoryContract {
    pub fn add_item(env: Env, item: Item) -> Result<(), Error> {
        if !env.invoker().is_contract() {
            return Err(Error::NotAuthorized);
        }

        let inventory = env.storage().persistent().map::<String, Item>(symbol!("items"));
        inventory.insert(item.sku.to_string(), item);
        Ok(())
    }

    pub fn remove_item(env: Env, sku: String, quantity: u32) -> Result<(), Error> {
        let inventory = env.storage().persistent().map::<String, Item>(symbol!("items"));
        let mut item = inventory.get(&sku).ok_or(Error::ItemNotFound)?;

        if item.quantity < quantity {
            return Err(Error::InsufficientStock);
        }

        item.quantity -= quantity;
        if item.quantity <= item.reorder_threshold {
            env.invoke_contract(
                &Address::from_pubkey(&env, &env.invoker().as_pubkey().unwrap()),
                &symbol!("place_order"),
                &vec![&env, &item.sku, &item.reorder_threshold],
            );
        }
        inventory.insert(sku, item);

        Ok(())
    }
}
