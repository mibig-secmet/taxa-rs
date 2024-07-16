// Copyright 2021 Danmarks Tekniske Universitet
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! NCBI taxdump handling for MIBiG
//!
//! ## Usage
//!
//! To build a taxon cache, use [`TaxonCache`]:
//!
//! ```no_run
//! # use mibig_taxa::MibigTaxonError;
//! #
//! # fn main() -> Result<(), MibigTaxonError> {
//! use std::path::PathBuf;
//! use mibig_taxa::TaxonCache;
//!
//! let cachefile = PathBuf::from("path/to/cache");
//! let taxdump = PathBuf::from("path/to/rankedlineage.dmp");
//! let mergeddump = PathBuf::from("path/to/merged.dmp");
//! let datadir = PathBuf::from("path/to/mibig/jsons/");
//!
//! let mut taxon_cache = TaxonCache::new();
//! taxon_cache.initialise_from_paths(taxdump, mergeddump, datadir)?;
//! taxon_cache.save_path(&cachefile)?;
//! #
//! #     Ok(())
//! # }
//! ```

pub mod errors;
pub mod mibig;
pub mod taxa;

pub use crate::errors::MibigTaxonError;
pub use crate::mibig::TaxonCache;
pub use crate::taxa::NcbiTaxEntry;
