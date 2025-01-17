// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

use crate::{
    client::api::PreparedTransactionData,
    types::block::{
        address::Address,
        output::{unlock_condition::AddressUnlockCondition, BasicOutputBuilder},
    },
    wallet::account::{handle::AccountHandle, operations::transaction::Transaction, TransactionOptions},
};

/// address with amount for `send_amount()`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressWithAmount {
    /// Bech32 encoded address
    pub address: String,
    /// Amount
    pub amount: u64,
}

impl AccountHandle {
    /// Function to create basic outputs with which we then will call
    /// [AccountHandle.send()](crate::account::handle::AccountHandle.send), the options can define the
    /// RemainderValueStrategy or custom inputs.
    /// Address needs to be Bech32 encoded
    /// ```ignore
    /// let outputs = vec![AddressWithAmount{
    ///     address: "rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu".to_string(),
    ///     amount: 1_000_000,
    /// }];
    ///
    /// let tx = account_handle.send_amount(outputs, None ).await?;
    /// println!("Transaction created: {}", tx.transaction_id);
    /// if let Some(block_id) = tx.block_id {
    ///     println!("Block sent: {}", block_id);
    /// }
    /// ```
    pub async fn send_amount(
        &self,
        addresses_with_amount: Vec<AddressWithAmount>,
        options: Option<TransactionOptions>,
    ) -> crate::wallet::Result<Transaction> {
        let prepared_transaction = self.prepare_send_amount(addresses_with_amount, options).await?;
        self.sign_and_submit_transaction(prepared_transaction).await
    }

    /// Function to prepare the transaction for
    /// [AccountHandle.send_amount()](crate::account::handle::AccountHandle.send_amount)
    pub async fn prepare_send_amount(
        &self,
        addresses_with_amount: Vec<AddressWithAmount>,
        options: Option<TransactionOptions>,
    ) -> crate::wallet::Result<PreparedTransactionData> {
        log::debug!("[TRANSACTION] prepare_send_amount");
        let mut outputs = Vec::new();
        let token_supply = self.client.get_token_supply().await?;

        for address_with_amount in addresses_with_amount {
            let (address, bech32_hrp) = Address::try_from_bech32_with_hrp(address_with_amount.address)?;
            self.client.bech32_hrp_matches(&bech32_hrp).await?;
            outputs.push(
                BasicOutputBuilder::new_with_amount(address_with_amount.amount)?
                    .add_unlock_condition(AddressUnlockCondition::new(address))
                    .finish_output(token_supply)?,
            )
        }

        self.prepare_transaction(outputs, options).await
    }
}
