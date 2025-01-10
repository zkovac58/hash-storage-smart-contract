use near_sdk::json_types::U64;
use near_sdk::store::LookupMap;
use near_sdk::{log, near};

#[near(serializers = [borsh, json])]
#[derive(Clone)]
pub struct EntityState {
    state_hash: String,
    last_updated: U64,
}

#[near(contract_state)]
pub struct Contract {
    entity_states: LookupMap<U64, EntityState>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            entity_states: LookupMap::new(b"e"),
        }
    }
}

#[near]
impl Contract {

    pub fn get_entity_state_hash(&self, entity_id: U64) -> Option<EntityState> {
        self.entity_states.get(&entity_id).cloned()
    }

    pub fn update_entity_state(&mut self, entity_id: U64, state_hash: String, last_updated: U64) {
        log!("Saving state hash for {:?}: {}", entity_id, state_hash);

        let entity_state = EntityState {
            state_hash: state_hash.clone(),
            last_updated: last_updated,
        };

        self.entity_states.insert(entity_id, entity_state);

        log!("Entity state updated successfully for {:?}", entity_id);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_entity_state_hash_then_check_storage() {
        let mut contract = Contract::default();

        let entity_id: u64 = 1;
        let state_hash = "new_state_hash".to_string();

        contract.update_entity_state(U64::from(entity_id), state_hash.clone(), U64::from(1736457668));

        let retrieved_entity_state_hash = contract.get_entity_state_hash(U64::from(entity_id));

        assert!(retrieved_entity_state_hash.is_some(), "Entity state was not found"); 
        let retrieved_state_hash = retrieved_entity_state_hash.unwrap(); 
        assert_eq!(retrieved_state_hash.state_hash, state_hash, "Entity state hash was not updated correctly");
    }
}
