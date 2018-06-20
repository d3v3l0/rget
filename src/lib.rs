// This file is part of rget.
//
// Copyright (C) 2016-2017 Arcterus (Alex Lyon) and rget contributors.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate pbr;
extern crate reqwest;
extern crate serde;
extern crate term;
extern crate toml;
#[macro_use]
extern crate serde_derive;
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate broadcast;
extern crate number_prefix;
//extern crate ftp;

pub use config::Config;
pub use download::Downloader;
pub use error::RgetError;
pub use network::Rget;
pub use output::OutputManager;

pub mod config;
pub mod download;
pub mod error;
pub mod network;
pub mod output;
mod partial;
pub mod protocol;
pub mod ui;
mod util;
