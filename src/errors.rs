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

//! MIBiG taxa error definitions

use std::error;
use std::fmt;
use std::io;
use std::num;

use serde_json;

#[derive(Debug)]
pub enum MibigTaxonError {
    Io(io::Error),
    InvalidTaxId(String),
    NotFound(i64),
    JSONParserError(serde_json::Error),
    IntParserError(num::ParseIntError),
}

macro_rules! implement_custom_error_from {
    ($f: ty, $e: expr) => {
        impl From<$f> for MibigTaxonError {
            fn from(f: $f) -> MibigTaxonError {
                $e(f)
            }
        }
    };
}

implement_custom_error_from!(io::Error, MibigTaxonError::Io);
implement_custom_error_from!(serde_json::Error, MibigTaxonError::JSONParserError);
implement_custom_error_from!(num::ParseIntError, MibigTaxonError::IntParserError);

impl fmt::Display for MibigTaxonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MibigTaxonError::Io(ref err) => write!(f, "IO error: {}", err),
            MibigTaxonError::InvalidTaxId(ref err) => write!(f, "Invalid TaxID: {}", err),
            MibigTaxonError::NotFound(ref err) => write!(f, "TaxID not found: {}", err),
            MibigTaxonError::JSONParserError(ref err) => write!(f, "Failed to parse JSON: {}", err),
            MibigTaxonError::IntParserError(ref err) => write!(f, "Failed to parse int: {}", err),
        }
    }
}

impl error::Error for MibigTaxonError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            MibigTaxonError::Io(ref err) => Some(err),
            MibigTaxonError::JSONParserError(ref err) => Some(err),
            MibigTaxonError::IntParserError(ref err) => Some(err),
            MibigTaxonError::NotFound(_) | MibigTaxonError::InvalidTaxId(_) => None,
        }
    }
}
