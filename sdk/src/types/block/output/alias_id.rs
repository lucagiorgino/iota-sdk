// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::block::{
    address::{Address, AliasAddress},
    output::OutputId,
};

impl_id!(pub AliasId, 32, "TODO.");

#[cfg(feature = "serde")]
string_serde_impl!(AliasId);

impl From<&OutputId> for AliasId {
    fn from(output_id: &OutputId) -> Self {
        Self::from(output_id.hash())
    }
}

impl AliasId {
    ///
    pub fn or_from_output_id(self, output_id: &OutputId) -> Self {
        if self.is_null() { Self::from(output_id) } else { self }
    }

    /// Returns the bech32 encoding of the alias ID.
    pub fn to_bech32(&self, bech32_hrp: &str) -> String {
        Address::Alias(AliasAddress::new(*self)).to_bech32(bech32_hrp)
    }
}

#[cfg(feature = "dto")]
#[allow(missing_docs)]
pub mod dto {
    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::types::block::error::dto::DtoError;

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    pub struct AliasIdDto(pub String);

    impl From<&AliasId> for AliasIdDto {
        fn from(value: &AliasId) -> Self {
            Self(value.to_string())
        }
    }

    impl TryFrom<&AliasIdDto> for AliasId {
        type Error = DtoError;

        fn try_from(value: &AliasIdDto) -> Result<Self, Self::Error> {
            value.0.parse::<Self>().map_err(|_| DtoError::InvalidField("alias id"))
        }
    }
}
