// Copyright 2018 Bitwise IO
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crypto::digest::Digest;
use crypto::sha2::Sha256;

const FAMILY_NAMESPACE: &str = "bond namespace";
const ORGANIZATION: &str = "organization";
const PARTICIPANT: &str = "participant";
const BOND: &str = "bond";
const HOLDING: &str = "holding";
const SETTLEMENT: &str = "settlement";
const RECEIPT: &str = "receipt";
const ORDER: &str = "order";
const QUOTE: &str = "quote";

const PREFIX_SIZE: usize = 6;

pub fn hash(object: &str, num: usize) -> String {
    let mut sha = Sha256::new();
    sha.input_str(object);
    sha.result_str()[..num].to_string()
}

pub fn make_organization_address(organization_id: &str) -> String {
    hash(&FAMILY_NAMESPACE, PREFIX_SIZE)
        + &hash(&ORGANIZATION, PREFIX_SIZE)
        + &hash(organization_id, 58)
}

pub fn make_participant_address(public_key: &str) -> String {
    hash(&FAMILY_NAMESPACE, PREFIX_SIZE) + &hash(&PARTICIPANT, PREFIX_SIZE) + &hash(public_key, 58)
}

pub fn make_bond_address(bond_id: &str) -> String {
    hash(&FAMILY_NAMESPACE, PREFIX_SIZE) + &hash(&BOND, PREFIX_SIZE) + &hash(bond_id, 58)
}

pub fn make_holding_address(organization_id: &str, asset_id: &str) -> String {
    hash(&FAMILY_NAMESPACE, PREFIX_SIZE)
        + &hash(&HOLDING, PREFIX_SIZE)
        + &hash(organization_id, 22)
        + &hash(asset_id, 36)
}

pub fn make_settlement_address(organization_id: &str, order_id: &str) -> String {
    hash(&FAMILY_NAMESPACE, PREFIX_SIZE)
        + &hash(&SETTLEMENT, PREFIX_SIZE)
        + &hash(organization_id, 22)
        + &hash(order_id, 36)
}

pub fn make_receipt_address(organization_id: &str, bond_id: &str) -> String {
    hash(&FAMILY_NAMESPACE, PREFIX_SIZE)
        + &hash(&RECEIPT, PREFIX_SIZE)
        + &hash(organization_id, 22)
        + &hash(bond_id, 36)
}

pub fn make_quote_address(organization_id: &str, bond_id: &str) -> String {
    hash(&FAMILY_NAMESPACE, PREFIX_SIZE)
        + &hash(&QUOTE, PREFIX_SIZE)
        + &hash(organization_id, 22)
        + &hash(bond_id, 36)
}

pub fn make_order_address(organization_id: &str, bond_id: &str) -> String {
    hash(&FAMILY_NAMESPACE, PREFIX_SIZE)
        + &hash(&ORDER, PREFIX_SIZE)
        + &hash(organization_id, 22)
        + &hash(bond_id, 36)
}
