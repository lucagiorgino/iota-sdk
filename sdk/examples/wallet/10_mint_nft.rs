// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will mint a native token.
//! Rename `.env.example` to `.env` first.
//!
//! `cargo run --example mint_nft --release`

use iota_sdk::{
    types::block::output::{
        feature::{IssuerFeature, SenderFeature},
        unlock_condition::AddressUnlockCondition,
        NftId, NftOutputBuilder,
    },
    wallet::{NftOptions, Result, Wallet},
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    // Create the wallet
    let wallet = Wallet::builder().finish().await?;

    // Get the account we generated with `01_create_wallet`
    let account = wallet.get_account("Alice").await?;

    account.sync(None).await?;

    // Set the stronghold password
    wallet
        .set_stronghold_password(&std::env::var("STRONGHOLD_PASSWORD").unwrap())
        .await?;

    let nft_options = vec![NftOptions {
        address: Some("rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu".to_string()),
        sender: Some("rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy".to_string()),
        metadata: Some(b"some NFT metadata".to_vec()),
        tag: Some(b"some NFT tag".to_vec()),
        issuer: Some("rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy".to_string()),
        immutable_metadata: Some(b"some NFT immutable metadata".to_vec()),
    }];

    let transaction = account.mint_nfts(nft_options, None).await?;

    println!("Transaction: {}.", transaction.transaction_id,);
    println!(
        "Block sent: {}/api/core/v2/blocks/{}.",
        &std::env::var("NODE_URL").unwrap(),
        transaction.block_id.expect("no block created yet")
    );

    // Build nft output manually
    let sender_address = account.addresses().await?[0].address().clone();
    let token_supply = account.client().get_token_supply().await?;
    let outputs = vec![
        // address of the owner of the NFT
        NftOutputBuilder::new_with_amount(1_000_000, NftId::null())?
            .add_unlock_condition(AddressUnlockCondition::new(*sender_address.as_ref()))
            .add_feature(SenderFeature::new(*sender_address.as_ref()))
            .add_immutable_feature(IssuerFeature::new(*sender_address.as_ref()))
            .finish_output(token_supply)?,
    ];

    let transaction = account.send(outputs, None).await?;

    println!(
        "Transaction: {} Block sent: {}/api/core/v2/blocks/{}",
        transaction.transaction_id,
        &std::env::var("NODE_URL").unwrap(),
        transaction.block_id.expect("No block created yet")
    );

    Ok(())
}
