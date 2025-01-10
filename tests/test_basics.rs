use serde_json::json;

#[tokio::test]
async fn test_contract_is_operational() -> Result<(), Box<dyn std::error::Error>> {
    let contract_wasm = near_workspaces::compile_project("./").await?;

    test_entity_state_functions(&contract_wasm).await?;
    Ok(())
}

async fn test_entity_state_functions(contract_wasm: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let sandbox = near_workspaces::sandbox().await?;
    let contract = sandbox.dev_deploy(contract_wasm).await?;

    let user_account = sandbox.dev_create_account().await?;

    // Update entity state
    let entity_id = 1;
    let state_hash = "new_state_hash";
    let last_updated: u64 = 1736457668;

    let outcome = user_account
        .call(contract.id(), "update_entity_state")
        .args_json(json!({
            "entity_id": entity_id.to_string(),
            "state_hash": state_hash,
            "last_updated": last_updated.to_string()
        }))
        .transact()
        .await?;
    assert!(outcome.is_success());

    // Retrieve entity state
    let retrieved_entity_state = user_account
        .view(contract.id(), "get_entity_state_hash")
        .args_json(json!({
            "entity_id": entity_id.to_string(),
        }))
        .await?;

    let retrieved_entity_state: Option<serde_json::Value> = retrieved_entity_state.json()?;
    assert!(retrieved_entity_state.is_some(), "Entity state was not found");

    let retrieved_state_hash = retrieved_entity_state.unwrap().get("state_hash").unwrap().as_str().unwrap().to_string();
    assert_eq!(retrieved_state_hash, state_hash, "Entity state hash was not updated correctly");

    Ok(())
}
