// Copyright 2019 Dmitry Tantsur <divius.inside@gmail.com>
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

//! JSON structures and protocol bits for the object storage API.

#![allow(missing_docs)]

use chrono::{DateTime, Utc};
use serde::Deserialize;

use super::super::common;

#[derive(Debug, Clone, Deserialize)]
pub struct Container {
    pub bytes: u64,
    pub count: u64,
    #[serde(deserialize_with = "common::protocol::deser_http_date")]
    pub last_modified: DateTime<Utc>,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Object {
    pub bytes: u64,
    pub content_type: String,
    #[serde(deserialize_with = "common::protocol::deser_http_date")]
    pub last_modified: DateTime<Utc>,
    pub name: String,
}
