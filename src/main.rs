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
use sp_core::crypto::{PublicError, Ss58Codec};
use sp_io::hashing::blake2_256;
use sp_runtime::AccountId32 as AccountId;

struct Address(AccountId);

impl std::str::FromStr for Address {
    type Err = PublicError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        <AccountId as Ss58Codec>::from_string(s).map(Self)
    }
}

fn main() {
    let matches = clap_app!((env!("CARGO_PKG_NAME")) =>
        (version: env!("CARGO_PKG_VERSION"))
        (author: crate_authors!("\n"))
        (about: crate_description!())
        (@arg THRESHOLD: +required "The number of signatures needed to perform the operation")
        (@arg ADDRESSES: +required ... "The addresses to use")
    )
    .get_matches();
    let threshold = value_t_or_exit!(matches, "THRESHOLD", u16);
    let mut who: Vec<_> = values_t_or_exit!(matches, "ADDRESSES", Address)
        .into_iter()
        .map(|e| e.0)
        .collect();
    who.sort_unstable();
    let entropy = (b"modlpy/utilisuba", who, threshold).using_encoded(blake2_256);
    println!(
        "{}",
        AccountId::decode(&mut &entropy[..])
            .unwrap_or_default()
            .to_ss58check()
    )
}
