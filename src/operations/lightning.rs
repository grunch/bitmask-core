use crate::{
    data::constants::LNDHUB_ENDPOINT,
    util::{get, post_json_auth},
};
use anyhow::{Ok, Result};
use lightning_invoice::Invoice;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;

/// Lightning wallet credentials
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

/// Wallet creation response
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum CreateWalletResponse {
    Username { username: String },
    Error { error: String },
}

/// Lightning wallet tokens
#[derive(Debug, Serialize, Deserialize)]
pub struct Tokens {
    pub refresh: String,
    pub token: String,
}

/// Amount and currency
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Money {
    pub value: String,
    pub currency: String,
}

/// Add Invoice response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddInvoiceResponse {
    pub req_id: String,
    pub uid: u32,
    pub payment_request: Option<String>,
    pub meta: Option<String>,
    pub metadata: Option<String>,
    pub amount: Money,
    pub rate: Option<String>,
    pub currency: String,
    pub target_account_currency: Option<String>,
    pub account_id: Option<String>,
    pub error: Option<String>,
    pub fees: Option<String>,
}

/// User balance response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalancesResponse {
    pub uid: u32,
    pub accounts: HashMap<String, Account>,
    pub error: Option<String>,
}

/// User account
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub account_id: String,
    pub balance: String,
    pub currency: String,
}

/// Pay Invoice request
#[derive(Debug, Serialize, Deserialize)]
pub struct PayInvoiceRequest {
    pub payment_request: String,
}

/// Lightning transaction
#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub txid: String,
    pub fee_txid: Option<String>,
    pub outbound_txid: Option<String>,
    pub inbound_txid: Option<String>,
    pub created_at: u64,
    pub outbound_amount: String,
    pub inbound_amount: String,
    pub outbound_account_id: String,
    pub inbound_account_id: String,
    pub outbound_uid: u32,
    pub inbound_uid: u32,
    pub outbound_currency: String,
    pub inbound_currency: String,
    pub exchange_rate: String,
    pub tx_type: String,
    pub fees: String,
    pub reference: Option<String>,
}

/// Pay invoice response
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PayInvoiceResponse {
    pub payment_hash: String,
    pub uid: u32,
    pub success: bool,
    pub currency: String,
    pub payment_request: Option<String>,
    pub amount: Option<Money>,
    pub fees: Option<Money>,
    pub error: Option<String>,
    pub payment_preimage: Option<String>,
    pub destination: Option<String>,
    pub description: Option<String>,
}

/// Creates a new lightning custodial wallet
pub async fn create_wallet(username: &str, password: &str) -> Result<CreateWalletResponse> {
    let endpoint = LNDHUB_ENDPOINT.to_string();
    let creds = Credentials {
        username: username.to_string(),
        password: password.to_string(),
    };
    let create_url = format!("{endpoint}/create");
    let response = post_json_auth(&create_url, &Some(creds), None).await?;

    let res: CreateWalletResponse = serde_json::from_str(&response)?;

    Ok(res)
}

/// Get a auth tokens
pub async fn auth(username: &str, password: &str) -> Result<Tokens> {
    let creds = Credentials {
        username: username.to_string(),
        password: password.to_string(),
    };
    let endpoint = LNDHUB_ENDPOINT.to_string();
    let auth_url = format!("{endpoint}/auth");
    let response = post_json_auth(&auth_url, &Some(creds), None).await?;
    let tokens: Tokens = serde_json::from_str(&response)?;

    Ok(tokens)
}

/// Creates a lightning invoice
pub async fn create_invoice(
    description: &str,
    amount: u32,
    token: &str,
) -> Result<AddInvoiceResponse> {
    let endpoint = LNDHUB_ENDPOINT.to_string();
    let amount = amount as f32 / 100_000_000.0;
    let amt_str = amount.to_string();
    let url = format!("{endpoint}/addinvoice?amount={amt_str}&meta={description}");
    let response = get(&url, Some(token)).await?;
    let invoice: AddInvoiceResponse = serde_json::from_str(&response)?;

    Ok(invoice)
}

/// Decode a lightning invoice (bolt11)
pub fn decode_invoice(payment_request: &str) -> Result<Invoice> {
    let invoice = Invoice::from_str(payment_request)?;

    Ok(invoice)
}

/// Get user lightning balance
pub async fn get_balance(token: &str) -> Result<Vec<Account>> {
    let endpoint = LNDHUB_ENDPOINT.to_string();
    let url = format!("{endpoint}/balance");
    let response = get(&url, Some(token)).await?;
    let balance: BalancesResponse = serde_json::from_str(&response)?;
    let mut accounts = Vec::new();
    for (_, value) in balance.accounts {
        accounts.push(value);
    }

    Ok(accounts)
}

/// Pay a lightning invoice
pub async fn pay_invoice(payment_request: &str, token: &str) -> Result<PayInvoiceResponse> {
    let endpoint = LNDHUB_ENDPOINT.to_string();
    let url = format!("{endpoint}/payinvoice");
    let req = PayInvoiceRequest {
        payment_request: payment_request.to_string(),
    };
    let response = post_json_auth(&url, &Some(req), Some(token)).await?;
    let response: PayInvoiceResponse = serde_json::from_str(&response)?;

    Ok(response)
}

/// Get successful lightning transactions user made. Order newest to oldest.
pub async fn get_txs(token: &str) -> Result<Vec<Transaction>> {
    let endpoint = LNDHUB_ENDPOINT.to_string();
    let url = format!("{endpoint}/gettxs");
    let response = get(&url, Some(token)).await?;
    let txs = serde_json::from_str(&response)?;

    Ok(txs)
}
