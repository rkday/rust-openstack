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

//! Foundation bits exposing the object storage API.

use osauth::services::OBJECT_STORAGE;
use osauth::sync::SyncStream;

use super::super::session::Session;
use super::super::utils::Query;
use super::super::Result;
use super::protocol::*;

const NO_PATH: Option<&'static str> = None;

/// Download the requested container.
pub fn download_object<C, O>(session: &Session, container: C, object: O) -> Result<SyncStream>
where C: AsRef<str>, O: AsRef<str> {
    let c_id = container.as_ref();
    let o_id = object.as_ref();
    trace!("Requesting object {} from container {}", o_id, c_id);
    Ok(session.download(session.get(OBJECT_STORAGE, &[c_id, o_id], None)?))
}

/// List containers for the current account.
pub fn list_containers(session: &Session, mut query: Query) -> Result<Vec<Container>> {
    query.push_str("format", "json");
    trace!("Listing containers with {:?}", query);
    let root: Vec<Container> = session.get_json_query(OBJECT_STORAGE, NO_PATH, query, None)?;
    trace!("Received containers: {:?}", root);
    Ok(root)
}

/// List objects in a given container.
pub fn list_objects<C>(session: &Session, container: C, mut query: Query) -> Result<Vec<Object>>
where
    C: AsRef<str>,
{
    query.push_str("format", "json");
    let id = container.as_ref();
    trace!("Listing objects in container {} with {:?}", id, query);
    let root: Vec<Object> = session.get_json_query(OBJECT_STORAGE, &[id], query, None)?;
    trace!("Received objects: {:?}", root);
    Ok(root)
}
