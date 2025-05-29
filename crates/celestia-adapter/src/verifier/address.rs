use std::fmt::{Display, Formatter};
use std::str::FromStr;

use anyhow::Context;
use celestia_types::state::{AccAddress, AddressKind, AddressTrait};
// use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Eq, Serialize, Deserialize, Hash)] // TODO: , BorshDeserialize, BorshSerialize)]
pub struct CelestiaAddress(AccAddress);

impl AsRef<[u8]> for CelestiaAddress {
    fn as_ref(&self) -> &[u8] {
        self.0.id_ref().as_ref()
    }
}

/// Decodes slice of bytes into CelestiaAddress
/// Treats it as string if it starts with HRP and the rest is valid ASCII
/// Otherwise just decodes the tendermint Id and creates address from that.
impl<'a> TryFrom<&'a [u8]> for CelestiaAddress {
    type Error = anyhow::Error;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        let hrp = AddressKind::Account.prefix();

        if value.starts_with(hrp.as_bytes()) && value.is_ascii() {
            // safety, because we checked that it is ASCII
            let s = unsafe { std::str::from_utf8_unchecked(value) };
            s.parse().context("failed parsing celestia address")
        } else {
            let array = value.try_into().context("invalid slice length")?;
            let id = tendermint::account::Id::new(array);
            Ok(Self(AccAddress::new(id)))
        }
    }
}

impl Display for CelestiaAddress {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for CelestiaAddress {
    type Err = <AccAddress as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

impl sov_rollup_interface::BasicAddress for CelestiaAddress {}
