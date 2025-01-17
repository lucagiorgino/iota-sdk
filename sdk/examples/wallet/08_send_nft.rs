// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will send an nft.
//! Rename `.env.example` to `.env` first.
//!
//! `cargo run --example send_nft --release`

use std::str::FromStr;

use iota_sdk::{
    types::block::output::NftId,
    wallet::{AddressAndNftId, Result, Wallet},
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    // Create the wallet
    let wallet = Wallet::builder().finish().await?;

    // Get the account we generated with `01_create_wallet`
    let account = wallet.get_account("Alice").await?;

    // Set the stronghold password
    wallet
        .set_stronghold_password(&std::env::var("STRONGHOLD_PASSWORD").unwrap())
        .await?;

    let outputs = vec![AddressAndNftId {
        address: "rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu".to_string(),
        // Replace with an NftId that is available in the account
        nft_id: NftId::from_str("0xe192461b30098a5da889ef6abc9e8130bf3b2d980450fa9201e5df404121b932")?,
    }];

    let transaction = account.send_nft(outputs, None).await?;

    println!(
        "Transaction: {} Block sent: {}/api/core/v2/blocks/{}",
        transaction.transaction_id,
        &std::env::var("NODE_URL").unwrap(),
        transaction.block_id.expect("no block created yet")
    );

    Ok(())
}
