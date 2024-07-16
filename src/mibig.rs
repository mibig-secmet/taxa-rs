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

//! Library implementation

use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{self, BufRead, Read, Write};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_json;

use crate::errors::MibigTaxonError;
use crate::taxa::{Entry, NcbiTaxEntry};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TaxonCache {
    pub deprecated_ids: HashMap<i64, i64>,
    pub mappings: HashMap<i64, NcbiTaxEntry>,
}

impl TaxonCache {
    pub fn new() -> TaxonCache {
        TaxonCache {
            deprecated_ids: HashMap::new(),
            mappings: HashMap::new(),
        }
    }

    pub fn initialise(
        &mut self,
        taxdump: impl Read,
        merged_id_dump: impl Read,
        taxids: &mut HashSet<i64>,
    ) -> Result<(), MibigTaxonError> {
        populate_merged_ids(merged_id_dump, taxids, &mut self.deprecated_ids)?;

        populate_mappings(taxdump, taxids, &self.deprecated_ids, &mut self.mappings)?;

        Ok(())
    }

    pub fn initialise_from_paths(
        &mut self,
        taxdump_path: PathBuf,
        merged_id_dump_path: PathBuf,
        datadir_path: PathBuf,
    ) -> Result<(), MibigTaxonError> {
        let mut taxids = self.find_taxids(datadir_path)?;
        let taxdump = fs::File::open(taxdump_path)?;
        let mergeddump = fs::File::open(merged_id_dump_path)?;

        self.initialise(taxdump, mergeddump, &mut taxids)?;

        Ok(())
    }

    pub fn find_taxids(&self, datadir: PathBuf) -> Result<HashSet<i64>, MibigTaxonError> {
        let mut taxids: HashSet<i64> = HashSet::new();
        let mut entries = fs::read_dir(datadir)?
            .map(|res| res.map(|e| e.path()))
            .filter(|p| p.is_ok() && p.as_ref().unwrap().extension() == Some("json".as_ref()))
            .collect::<Result<Vec<_>, io::Error>>()?;

        entries.sort();

        for path in entries {
            let content = fs::read_to_string(&path)?;
            let entry: Entry = serde_json::from_str(&content)?;
            taxids.insert(entry.taxonomy.ncbi_tax_id);
        }
        Ok(taxids)
    }

    pub fn save(&self, mut output: impl Write) -> Result<usize, MibigTaxonError> {
        let json_data = serde_json::to_string(self)?;
        output.write(json_data.as_bytes())?;

        Ok(self.mappings.len())
    }

    pub fn save_path(&self, outfile: &PathBuf) -> Result<usize, MibigTaxonError> {
        let out = fs::File::create(outfile)?;
        self.save(out)
    }

    pub fn load(&mut self, mut input: impl Read) -> Result<usize, MibigTaxonError> {
        let mut json_data = String::new();
        input.read_to_string(&mut json_data)?;
        let loaded_cache: TaxonCache = serde_json::from_str(&json_data)?;
        self.mappings = loaded_cache.mappings;
        self.deprecated_ids = loaded_cache.deprecated_ids;

        Ok(self.mappings.len())
    }

    pub fn load_path(&mut self, infile: &PathBuf) -> Result<usize, MibigTaxonError> {
        let handle = fs::File::open(infile)?;
        self.load(handle)
    }
}

fn populate_merged_ids(
    merged_id_dump: impl Read,
    taxids: &mut HashSet<i64>,
    deprecated_ids: &mut HashMap<i64, i64>,
) -> Result<(), MibigTaxonError> {
    for line_option in io::BufReader::new(merged_id_dump).lines() {
        if let Ok(line) = line_option {
            let parts: Vec<String> = line
                .trim()
                .splitn(3, "|")
                .map(|part| part.trim().to_string())
                .collect();

            let old_id: i64 = parts[0].parse()?;
            if !taxids.contains(&old_id) {
                continue;
            }

            let new_id: i64 = parts[1].parse()?;

            deprecated_ids.insert(old_id, new_id);
            taxids.remove(&old_id);
            taxids.insert(new_id);
        }
    }
    Ok(())
}

fn populate_mappings(
    taxdump: impl Read,
    taxids: &HashSet<i64>,
    deprecated_ids: &HashMap<i64, i64>,
    mappings: &mut HashMap<i64, NcbiTaxEntry>,
) -> Result<(), MibigTaxonError> {
    for line_option in io::BufReader::new(taxdump).lines() {
        if let Ok(line) = line_option {
            let parts: Vec<String> = line
                .trim()
                .splitn(11, "|")
                .map(|part| match part.trim() {
                    "" => "Unknown".to_string(),
                    part => part.to_string(),
                })
                .collect();

            let mut tax_id: i64 = parts[0].parse()?;
            if deprecated_ids.contains_key(&tax_id) {
                tax_id = *deprecated_ids.get(&tax_id).unwrap();
            }

            if !taxids.contains(&tax_id) {
                continue;
            }

            let entry = NcbiTaxEntry {
                tax_id,
                name: parts[1].to_owned(),
                species: parts[2]
                    .split_whitespace()
                    .next_back()
                    .unwrap_or(parts[2].as_str())
                    .to_owned(),
                genus: parts[3].to_owned(),
                family: parts[4].to_owned(),
                order: parts[5].to_owned(),
                class: parts[6].to_owned(),
                phylum: parts[7].to_owned(),
                kingdom: parts[8].to_owned(),
                superkingdom: parts[9].to_owned(),
            };

            mappings.insert(tax_id, entry.to_owned());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialise() {
        let mut taxids: HashSet<i64> = HashSet::new();
        taxids.insert(12345);

        let merged_ids = "12345   |    23456  |".as_bytes();
        let taxdump = "23456  |       Streptomyces examplis NBC12345   |       Streptomyces examplis |       Streptomyces    |       Streptomycetaceae       |       Streptomycetales        |       Actinomycetia   |       Actinobacteria  |               |      Bacteria |".as_bytes();

        let mut taxon_cache = TaxonCache::new();

        let res = taxon_cache.initialise(taxdump, merged_ids, &mut taxids);
        assert_eq!(res.is_ok(), true);
        assert_eq!(
            taxon_cache.deprecated_ids.len(),
            1,
            "unexpected length of deprecated_ids: {}",
            taxon_cache.deprecated_ids.len()
        );
        assert_eq!(
            taxon_cache.mappings.len(),
            1,
            "unexpected length of mappings: {}",
            taxon_cache.mappings.len()
        );
        assert_eq!(
            taxon_cache.mappings.get(&23456).unwrap().name,
            "Streptomyces examplis NBC12345"
        );
    }
}
