// Copyright 2020 Nym Technologies SA
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

use crate::filter;
use crypto::asymmetric::{encryption, identity};
use nymsphinx_addressing::nodes::NymNodeRoutingAddress;
use nymsphinx_types::Node as SphinxNode;
use std::convert::TryInto;
use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub struct Client {
    pub pub_key: String,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub location: String,
    pub client_listener: String,
    pub mixnet_listener: SocketAddr,
    pub identity_key: identity::PublicKey,
    pub sphinx_key: encryption::PublicKey, // TODO: or nymsphinx::PublicKey? both are x25519
    pub registered_clients: Vec<Client>,
    pub last_seen: u64,
    pub version: String,
}

impl Node {
    pub fn has_client(&self, client_pub_key: String) -> bool {
        self.registered_clients
            .iter()
            .find(|client| client.pub_key == client_pub_key)
            .is_some()
    }
}

impl filter::Versioned for Node {
    fn version(&self) -> String {
        self.version.clone()
    }
}

impl<'a> Into<SphinxNode> for &'a Node {
    fn into(self) -> SphinxNode {
        let node_address_bytes = NymNodeRoutingAddress::from(self.mixnet_listener)
            .try_into()
            .unwrap();

        SphinxNode::new(node_address_bytes, (&self.sphinx_key).into())
    }
}
