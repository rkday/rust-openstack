// Copyright 2017 Dmitry Tantsur <divius.inside@gmail.com>
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

extern crate env_logger;
extern crate futures;
extern crate hyper;
extern crate tokio_core;

extern crate openstack;

use futures::Future;


#[cfg(feature = "compute")]
fn main() {
    env_logger::init()
        .expect("Unable to initialize logging");
    let mut core = tokio_core::reactor::Core::new()
        .expect("Unable to initialize reactor");
    let auth = openstack::auth::Identity::from_env(&core.handle())
        .expect("Unable to load authentication from the environment");

    let fut = openstack::compute::Compute::new(auth).and_then(|compute| {
        let sorting = openstack::compute::v2::ServerSortKey::AccessIpv4;
        compute.find_servers().sort_by(openstack::Sort::Asc(sorting)).fetch()
    });

    let servers = core.run(fut).expect("Request failed");
    for s in &servers {
        println!("ID = {}, Name = {}", s.id(), s.name());
    }

    /*
    let active = server_manager.query()
        .sort_by(openstack::Sort::Asc(sorting)).with_status("ACTIVE")
        .fetch().expect("Cannot list servers");
    println!("Only active servers:");
    for s in &active {
        println!("ID = {}, Name = {}", s.id(), s.name());
    }
    */
}

#[cfg(not(feature = "compute"))]
fn main() {
    panic!("This example cannot run with 'compute' feature disabled");
}
