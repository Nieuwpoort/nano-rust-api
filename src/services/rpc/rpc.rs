use once_cell::sync::Lazy;
use anyhow::{Context, Result};
use reqwest::{Client};
use serde_json::json;

use crate::{r#enum::api::error::rpc::RpcError, structs::{account::{ balance::balance_response_rpc::BalanceResponseRpc, history::account_history_response_rpc::AccountHistoryResponseRpc, 
    info::{account_get_response_rpc::AccountGetResponseRpc, 
    account_info_response_rpc::AccountInfoResponseRpc, validate_account_number_response_rpc::ValidateAccountNumberResponseRpc}, 
    key::{deterministic_key_response_rpc::DeterministicKeyResponseRpc, key_expand_response::KeyExpandResponse, public_key_rpc::PublicKeyRpc}, 
    wallet::create_wallet_response::CreateWalletResponse}, api::api::ApiErrorResult, block::{block_info::StateBlock, block_info_response_rpc::BlockInfoResponseRpc, 
        block_pending_response_rpc::BlocksPerAccountResponse, create_block_response_rpc::CreateBlockResponseRpc, 
        create_block_rpc::CreateBlockRpc}, cache::env::ENV_CACHE, nano_network::active_difficulty::ActiveDifficultyResponseRpc, nano_rpc::rpc_error_response::RpcErrorResponse, transaction::process_response_rpc::ProcessResponseRpc, work::work_generate_response::WorkGenerateResponseRpc}};

static CLIENT: Lazy<Client> = Lazy::new(|| { Client::new()});

pub async fn account_exists_rpc(account: &str) -> Result<bool, ApiErrorResult> {
    let request = json!({
        "action": "validate_account_number",
        "account": account
    });

    let response = get_rpc_response(request, "Failed to send account_exists request").await?;

    let response_text = response.text().await
        .context("Failed to read response body")
        .map_err(|_| RpcError::ReadError.to_response())?;
    
    if let Ok(valid_resp) = serde_json::from_str::<ValidateAccountNumberResponseRpc>(&response_text) {
        return Ok(valid_resp.valid == "1");
    } else {
        return Ok(false);
    }
}


pub async fn get_public_key_rpc(address: &str) -> Result<String, ApiErrorResult> {
    let request = json!({
        "action": "account_key",
        "account": address
    });

    let response = get_rpc_response(request, "Failed to send account_key request").await?;

    if !response.status().is_success() {
        let error: RpcErrorResponse = response.json().await.unwrap_or(RpcErrorResponse {
            error: "Unknown error".to_string(),
        });
        return Err(RpcError::ResponseError(error.error).to_response());
    }

    let key_response: PublicKeyRpc = response.json().await
        .context("Failed to parse account_key response")
        .map_err(|e| RpcError::ParseError(format!("Failed to parse RPC response: {}", e)).to_response())?;
    
    Ok(key_response.key)
}

pub async fn account_info_rpc(account: &str) -> Result<AccountInfoResponseRpc, ApiErrorResult> {
    let request = json!({
        "action": "account_info".to_string(),
        "account": account.to_string(),
        "representative": "true".to_string(),
    });

    let response = get_rpc_response(request, "Failed to send account_info request").await?;

    if !response.status().is_success() {
        let error: RpcErrorResponse = response.json().await.unwrap_or(RpcErrorResponse {
            error: "Unknown error".to_string(),
        });
        return Err(RpcError::ResponseError(error.error).to_response());
    }

    let info: AccountInfoResponseRpc = response.json().await
        .context("Failed to parse account_info response")
        .map_err(|e| RpcError::ParseError(format!("Failed to parse RPC response: {}", e)).to_response())?;

    Ok(info)
}

pub async fn public_key_to_nano_address_rpc(public_key: &str) -> Result<String, ApiErrorResult> {
    let request = serde_json::json!({
        "action": "account_get",
        "key": public_key,
    });
    let response = get_rpc_response(request, "Failed to send account_get request").await?;

    if !response.status().is_success() {
        let error: RpcErrorResponse = response.json().await.unwrap_or(RpcErrorResponse {
            error: "Unknown error".to_string(),
        });
        return Err(RpcError::ResponseError(error.error).to_response());
    }

    let result: AccountGetResponseRpc = response.json().await
        .context("Failed to parse account_get response")
        .map_err(|e| RpcError::ParseError(format!("Failed to parse RPC response: {}", e)).to_response())?;
    
    Ok(result.account)
}

pub async fn account_balance_rpc(account: &str) -> Result<BalanceResponseRpc, ApiErrorResult> {
    let request = json!({
        "action": "account_balance",
        "account": account
    });
    let response = get_rpc_response(request, "Failed to send account_balance request").await?;

    if !response.status().is_success() {
        let error: RpcErrorResponse = response.json().await.unwrap_or(RpcErrorResponse {
            error: "Unknown error".to_string(),
        });
        return Err(RpcError::ResponseError(error.error).to_response());
    }

    let balance: BalanceResponseRpc = response.json().await
        .context("Failed to parse account_balance response")
        .map_err(|e| RpcError::ParseError(format!("Failed to parse RPC response: {}", e)).to_response())?;
    Ok(balance)
}

pub async fn account_history_rpc(account: &str, count: u32, head: Option<String>) -> Result<AccountHistoryResponseRpc, ApiErrorResult> {
    let mut request = json!({
        "action": "account_history",
        "account": account,
        "count": count.to_string(),
        "raw": true
    });

    if let Some(h) = head {
        request["head"] = json!(h);
    }

    let response = get_rpc_response(request, "Failed to send account_history request").await?;

    if !response.status().is_success() {
        let error: RpcErrorResponse = response.json().await.unwrap_or(RpcErrorResponse {
            error: "Unknown error".to_string(),
        });
        return Err(RpcError::ResponseError(error.error).to_response());
    }

    let history: AccountHistoryResponseRpc = response.json().await
        .context("Failed to parse account_history response")
        .map_err(|e| RpcError::ParseError(format!("Failed to parse RPC response: {}", e)).to_response())?;
    
    Ok(history)
}

pub async fn receivable_rpc(account: &str) -> Result<BlocksPerAccountResponse, ApiErrorResult> {
    let count = 5;
    let request = json!({
        "action": "accounts_receivable",
        "accounts": [account],
        "count": count.to_string(),
    });

    let response = get_rpc_response(request, "Failed to send receivable request").await?;
    if !response.status().is_success() {
        let error: RpcErrorResponse = response.json().await.unwrap_or(RpcErrorResponse {
            error: "Unknown error".to_string(),
        });
        return Err(RpcError::ResponseError(error.error).to_response());
    }
    let receivable: BlocksPerAccountResponse = response.json().await
        .context("Failed to parse accounts_receivable response")
        .map_err(|e| RpcError::ParseError(format!("Failed to parse RPC response: {}", e)).to_response())?;
    Ok(receivable)
}

pub async fn pending_rpc(account: &str, count: Option<u32>) -> Result<std::collections::HashMap<String, String>, ApiErrorResult> {
    let count = count.unwrap_or(10);
    let request = json!({
        "action": "pending",
        "account": account,
        "count": count,
        "source": true
    });

    let response = get_rpc_response(request, "Failed to send pending request").await?;

    if !response.status().is_success() {
        let error: RpcErrorResponse = response.json().await.unwrap_or(RpcErrorResponse {
            error: "Unknown error".to_string(),
        });
        return Err(RpcError::ResponseError(error.error).to_response());
    }

    let result: serde_json::Value = response.json().await
        .context("Failed to parse pending response")
        .map_err(|e| RpcError::ParseError(format!("Failed to parse RPC response: {}", e)).to_response())?;

    // Extract blocks map from response
    if let Some(blocks) = result.get("blocks").and_then(|v| v.as_object()) {
        let mut pending_blocks = std::collections::HashMap::new();
        for (hash, _data) in blocks {
            pending_blocks.insert(hash.clone(), "".to_string());
        }
        Ok(pending_blocks)
    } else {
        Ok(std::collections::HashMap::new())
    }
}

pub async fn process_rpc(block: StateBlock, subtype: &str) -> Result<ProcessResponseRpc, ApiErrorResult> {
    let request = json!({
        "action": "process",
        "json_block": "true",
        "subtype": subtype,
        "block": block
    });

    let response = get_rpc_response(request, "Failed to send process request").await?;

    if !response.status().is_success() {
        let error: RpcErrorResponse = response.json().await.unwrap_or(RpcErrorResponse {
            error: "Unknown error".to_string(),
        });
        return Err(RpcError::ResponseError(error.error).to_response());
    }

    let result: ProcessResponseRpc = response.json().await
        .context("Failed to parse process response")
        .map_err(|e| RpcError::ParseError(format!("Failed to parse RPC response: {}", e)).to_response())?;
    Ok(result)
}

pub async fn work_generate_rpc(hash: &str, difficulty: Option<String>) -> Result<WorkGenerateResponseRpc, ApiErrorResult> {
    let mut request = json!({
        "action": "work_generate",
        "hash": hash
    });

    if let Some(d) = difficulty {
        request["difficulty"] = json!(d);
    }

    let response = get_rpc_response(request, "Failed to send work_generate request").await?;

    if !response.status().is_success() {
        let error: RpcErrorResponse = response.json().await.unwrap_or(RpcErrorResponse {
            error: "Unknown error".to_string(),
        });
        return Err(RpcError::ResponseError(error.error).to_response());
    }

    let work = response.json().await
        .context("Failed to parse work_generate response")
        .map_err(|e| RpcError::ParseError(format!("Failed to parse RPC response: {}", e)).to_response())?;
    Ok(work)
}

pub async fn block_info_rpc(hash: &str) -> Result<BlockInfoResponseRpc, ApiErrorResult> {
    let request = json!({
        "action": "block_info",
        "json_block": "true",
        "hash": hash
    });

    let response = get_rpc_response(request, "Failed to send block_info request").await?;

    if !response.status().is_success() {
        let error: RpcErrorResponse = response.json().await.unwrap_or(RpcErrorResponse {
            error: "Unknown error".to_string(),
        });
        return Err(RpcError::ResponseError(error.error).to_response());
    }

    let info: BlockInfoResponseRpc = response.json().await
        .context("Failed to parse block_info response")
        .map_err(|e| RpcError::ParseError(format!("Failed to parse RPC response: {}", e)).to_response())?;

    Ok(info)
}

pub async fn wallet_private_key_index_to_nano_address_rpc(private_key: &str, index: u32) -> Result<DeterministicKeyResponseRpc, ApiErrorResult> {
    let request = serde_json::json!({
        "action": "deterministic_key",
        "seed": private_key,
        "index": index,
    });

    let response = get_rpc_response(request, "Failed to send deterministic_key request").await?;
    
    if !response.status().is_success() {
        let error: RpcErrorResponse = response.json().await.unwrap_or(RpcErrorResponse {
            error: "Unknown error".to_string(),
        });
        return Err(RpcError::ResponseError(error.error).to_response());
    }

    let result: DeterministicKeyResponseRpc = response.json().await
        .context("Failed to parse deterministic_key response")
        .map_err(|e| RpcError::ParseError(format!("Failed to parse RPC response: {}", e)).to_response())?;

    Ok(result)
}

pub async fn health_check_rpc() -> Result<bool, ApiErrorResult> {
    let env = ENV_CACHE.get().unwrap();

    let request = json!({
        "action": "block_count"
    });

    let response = CLIENT.post(env.nano_node_rpc_url.as_str())
        .json(&request)
        .send()
        .await;

    match response {
        Ok(resp) => Ok(resp.status().is_success()),
        Err(_) => Ok(false),
    }
}

pub async fn get_nano_address_from_private_key_rpc(private_key: &str) -> Result<String, ApiErrorResult> {
    let request = serde_json::json!({
        "action": "key_expand",
        "key": private_key,
    });

    let response = get_rpc_response(request, "Failed to send key_expand request").await?;
    
    if !response.status().is_success() {
        let error: RpcErrorResponse = response.json().await.unwrap_or(RpcErrorResponse {
            error: "Unknown error".to_string(),
        });
        return Err(RpcError::ResponseError(error.error).to_response());
    }

    let result: KeyExpandResponse = response.json().await
        .context("Failed to parse key_expand response")
        .map_err(|e| RpcError::ParseError(format!("Failed to parse RPC response: {}", e)).to_response())?;
    Ok(result.account)
    
}

pub async fn create_wallet_rpc() -> Result<CreateWalletResponse, ApiErrorResult> {
    let env = ENV_CACHE.get().unwrap();

    let request = serde_json::json!({
        "action": "wallet_create"
    });

    let response = CLIENT.post(env.nano_node_rpc_url.as_str())
        .json(&request)
        .send()
        .await
        .context("Failed to send wallet_create request")
        .map_err(|e| RpcError::RequestError(format!("RPC request failed: {}", e)).to_response())?;

    let result: CreateWalletResponse = response.json().await
        .context("Failed to parse wallet_create response")
        .map_err(|e| RpcError::ParseError(format!("Failed to parse RPC response: {}", e)).to_response())?;

    Ok(result)
}

pub async fn get_rpc_response(request: serde_json::Value, error_message: &str) -> Result<reqwest::Response, ApiErrorResult> {
    let env = ENV_CACHE.get().unwrap();

    let response = CLIENT.post(env.nano_node_rpc_url.as_str())
        .json(&request)
        .send()
        .await
        .context(format!("{}", error_message))
        .map_err(|_| RpcError::RequestError(format!("RPC request failed")).to_response())?;
    
    Ok(response)
}

pub async fn active_difficulty_rpc() -> Result<ActiveDifficultyResponseRpc, ApiErrorResult> {
    let request = json!({
        "action": "active_difficulty"
    });

    let response = get_rpc_response(request, "Failed to send active_difficulty request").await?;

    if !response.status().is_success() {
        let error: RpcErrorResponse = response.json().await.unwrap_or(RpcErrorResponse {
            error: "Unknown error".to_string(),
        });
        return Err(RpcError::ResponseError(error.error).to_response());
    }

    let result: ActiveDifficultyResponseRpc = response.json().await
        .context("Failed to parse active_difficulty response")
        .map_err(|e| RpcError::ParseError(format!("Failed to parse RPC response: {}", e)).to_response())?;

    Ok(result)
}

pub async fn block_create_rpc(sign_block: CreateBlockRpc) -> Result<CreateBlockResponseRpc, ApiErrorResult> {
    let request = json!({
        "action": "block_create",
        "json_block": "true",
        "type": "state",
        "balance": sign_block.balance,
        "key": sign_block.key,
        "representative": sign_block.representative,
        "link": sign_block.link,
        "previous": sign_block.previous,
        "work": Some(sign_block.work),
    });
    let response = get_rpc_response(request, "Failed to send sign request").await?;

    if !response.status().is_success() {
        let error: RpcErrorResponse = response.json().await.unwrap_or(RpcErrorResponse {
            error: "Unknown error".to_string(),
        });
        return Err(RpcError::ResponseError(error.error).to_response());
    }
    let sign_response: CreateBlockResponseRpc = response.json().await
        .context("Failed to parse block_create response")
        .map_err(|e| RpcError::ParseError(format!("Failed to parse RPC response: {}", e)).to_response())?;

    Ok(sign_response)
}

#[allow(dead_code)]
pub async fn get_rpc_response_debug(request: &serde_json::Value, error_message: &str) -> Result<String, ApiErrorResult> {
    let env = ENV_CACHE.get().unwrap();

    let response = CLIENT.post(env.nano_node_rpc_url.as_str())
        .json(&request)
        .send()
        .await
        .context(format!("{}", error_message))
        .map_err(|e| RpcError::RequestError(format!("RPC request failed: {}", e)).to_response())?;

    let body_text = response.text().await
        .context("Failed to read response body")
        .map_err(|_| RpcError::ReadError.to_response())?;
    
    Ok(body_text)
}