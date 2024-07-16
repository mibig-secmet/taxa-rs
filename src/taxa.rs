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

//! Taxonomy helper classes

use serde::{Deserialize, Serialize};

/// Everything the NCBI will tell us about a taxid
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NcbiTaxEntry {
    pub tax_id: i64,
    pub name: String,
    pub species: String,
    pub genus: String,
    pub family: String,
    pub order: String,
    pub class: String,
    pub phylum: String,
    pub kingdom: String,
    pub superkingdom: String,
}

// Helper to parse the MIBiG json
#[doc(hidden)]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct Entry {
    pub taxonomy: Taxonomy,
}

// Helper to parse the MIBiG json
#[doc(hidden)]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct Taxonomy {
    #[serde(rename = "ncbiTaxId")]
    pub ncbi_tax_id: i64,
}
