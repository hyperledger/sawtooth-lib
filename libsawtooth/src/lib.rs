// Copyright 2019 Cargill Incorporated
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

#[cfg(any(
    feature = "lmdb-store",
    feature = "validator-internals",
    feature = "client"
))]
#[macro_use]
extern crate log;
#[cfg(feature = "validator-internals")]
#[macro_use]
extern crate metrics;

#[cfg(feature = "client")]
pub mod client;
#[cfg(feature = "validator-internals")]
pub mod consensus;
#[cfg(feature = "validator-internals")]
pub mod hashlib;
#[cfg(feature = "validator-internals")]
pub mod journal;
#[cfg(feature = "validator-internals")]
pub mod permissions;
pub mod protocol;
pub mod protos;
#[cfg(feature = "validator-internals")]
pub mod receipt;
#[cfg(feature = "validator-internals")]
pub mod scheduler;
#[cfg(feature = "validator-internals")]
pub mod state;
#[cfg(feature = "stores")]
pub mod store;
