// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// The module for the address generation
pub(crate) mod address_generation;
/// The module to get the accounts balance
pub(crate) mod balance;
/// The module to find additional addresses with balance
pub(crate) mod balance_finder;
/// The module for the collection of outputs with [UnlockCondition]s, that aren't only [AddressUnlockCondition]
pub(crate) mod output_collection;
/// The module for the output consolidation
pub(crate) mod output_consolidation;
/// The module for synchronization of an account
pub(crate) mod syncing;
/// The module to get the local time
pub(crate) mod time;
/// The module for value transfers
pub(crate) mod transfer;