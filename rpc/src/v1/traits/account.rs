// Copyright 2018 Kodebox, Inc.
// This file is part of CodeChain.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use ckey::{Address, SignatureData};
use jsonrpc_core::Result;
use primitives::{H160, H256};

build_rpc_trait! {
    pub trait Account {
        /// Gets a list of accounts
        # [rpc(name = "account_getList")]
        fn get_account_list(&self) -> Result<Vec<Address>>;

        /// Creates a new account
        # [rpc(name = "account_create")]
        fn create_account(&self, Option<String>) -> Result<Address>;

        /// Imports a private key
        # [rpc(name = "account_importRaw")]
        fn create_account_from_secret(&self, H256, Option<String>) -> Result<Address>;

        /// Removes the account
        # [rpc(name = "account_remove")]
        fn remove_account(&self, H160, Option<String>) -> Result<()>;

        /// Calculates the account's signature for a given message
        # [rpc(name = "account_sign")]
        fn sign(&self, H256, H160, Option<String>) -> Result<SignatureData>;
    }
}
