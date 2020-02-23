// Copyright 2020 David Li
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

//! Route guide utilities.

use crate::routeguide::{FeatureDatabase, Point};
use std::fmt::{Display, Formatter};
use std::path::PathBuf;

const COORD_FACTOR: f64 = 1e7;

/// Gets the default features file.
#[inline]
fn get_default_features_file() -> PathBuf {
    let dir = env!("CARGO_MANIFEST_DIR");
    let path = PathBuf::from(dir).join("data/route_guide_db.json");
    assert!(path.exists());
    path
}

/// Parses the JSON input file containing the list of features.
#[inline]
pub fn load_database() -> FeatureDatabase {
    let file = get_default_features_file();
    let file = std::fs::File::open(file).unwrap();
    serde_json::from_reader(file).unwrap()
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.get_latitude(), self.get_longitude())
    }
}

impl Point {
    #[inline]
    pub fn new(latitude: i32, longitude: i32) -> Self {
        Self {
            latitude,
            longitude,
        }
    }

    /// Gets the latitude for the given point.
    #[inline]
    pub fn get_latitude(&self) -> f64 {
        self.latitude as f64 / COORD_FACTOR
    }

    /// Gets the longitude for the given point.
    #[inline]
    pub fn get_longitude(&self) -> f64 {
        self.longitude as f64 / COORD_FACTOR
    }
}
