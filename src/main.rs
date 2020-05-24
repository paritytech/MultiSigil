// Copyright (C) 2017-2020 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
// Copyright (C) 2020 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

use clap::{clap_app, crate_authors, crate_description, value_t_or_exit, values_t_or_exit};
use codec::{Decode as _, Encode as _};
use sp_core::crypto::{PublicError, Ss58AddressFormat, Ss58Codec};
use sp_io::hashing::blake2_256;
use sp_runtime::AccountId32 as AccountId;

#[derive(Debug)]
enum Network {
    Polkadot,
    Kusama,
}

impl std::str::FromStr for Network {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "polkadot" => Ok(Network::Polkadot),
            "kusama" => Ok(Network::Kusama),
            _ => Err(()),
        }
    }
}

impl Network {
    fn format(&self) -> Ss58AddressFormat {
        match self {
            Network::Polkadot => Ss58AddressFormat::PolkadotAccount,
            Network::Kusama => Ss58AddressFormat::KusamaAccount,
        }
    }
}

struct Address {
    account: AccountId,
    format: Ss58AddressFormat,
}

impl std::str::FromStr for Address {
    type Err = PublicError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        <AccountId as Ss58Codec>::from_string_with_version(s)
            .map(|(account, format)| Self { account, format })
    }
}

fn main() {
    let matches = clap_app!((env!("CARGO_PKG_NAME")) =>
        (version: env!("CARGO_PKG_VERSION"))
        (author: crate_authors!("\n"))
        (about: crate_description!())
        (@arg NETWORK: --network +takes_value possible_value[kusama polkadot] default_value[kusama] "Network to calculate multisig for; defaults to Kusama")
        (@arg THRESHOLD: +required "The number of signatures needed to perform the operation")
        (@arg ADDRESSES: +required ... "The addresses to use")
    )
    .get_matches();

    let network = value_t_or_exit!(matches, "NETWORK", Network);
    let threshold = value_t_or_exit!(matches, "THRESHOLD", u16);
    let mut who: Vec<_> = values_t_or_exit!(matches, "ADDRESSES", Address)
        .into_iter()
        .map(|address| {
            if address.format == network.format() {
                address.account
            } else {
                panic!(
                    "Address type mismatch, please make sure to only specify {:?} addresses",
                    network
                )
            }
        })
        .collect();
    who.sort_unstable();
    let entropy = (b"modlpy/utilisuba", who, threshold).using_encoded(blake2_256);
    println!(
        "{}",
        AccountId::decode(&mut &entropy[..])
            .unwrap_or_default()
            .to_ss58check_with_version(network.format())
    )
}
