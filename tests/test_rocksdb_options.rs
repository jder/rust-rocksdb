// Copyright 2014 Tyler Neely
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

mod util;

use crate::util::DBPath;
use rocksdb::{BlockBasedOptions, Options, ReadOptions, DB};
use std::{fs, io::Read as _};

#[test]
fn test_set_num_levels() {
    let n = DBPath::new("test_set_num_levels");
    {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.set_num_levels(2);
        let _db = DB::open(&opts, &n).unwrap();
    }
}

#[test]
fn test_increase_parallelism() {
    let n = DBPath::new("test_increase_parallelism");
    {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.increase_parallelism(4);
        let _db = DB::open(&opts, &n).unwrap();
    }
}

#[test]
fn test_set_level_compaction_dynamic_level_bytes() {
    let n = DBPath::new("test_set_level_compaction_dynamic_level_bytes");
    {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.set_level_compaction_dynamic_level_bytes(true);
        let _db = DB::open(&opts, &n).unwrap();
    }
}

#[test]
fn test_block_based_options() {
    let n = DBPath::new("test_block_based_options");
    {
        let mut opts = Options::default();
        opts.create_if_missing(true);

        let mut block_opts = BlockBasedOptions::default();
        block_opts.set_cache_index_and_filter_blocks(true);
        block_opts.set_pin_l0_filter_and_index_blocks_in_cache(true);
        block_opts.set_format_version(4);
        block_opts.set_index_block_restart_interval(16);

        opts.set_block_based_table_factory(&block_opts);
        let _db = DB::open(&opts, &n).unwrap();

        // read the setting from the LOG file
        let mut rocksdb_log = fs::File::open(format!("{}/LOG", (&n).as_ref().to_str().unwrap()))
            .expect("rocksdb creates a LOG file");
        let mut settings = String::new();
        rocksdb_log.read_to_string(&mut settings).unwrap();

        // check the settings are set in the LOG file
        assert!(settings.contains("cache_index_and_filter_blocks: 1"));
        assert!(settings.contains("pin_l0_filter_and_index_blocks_in_cache: 1"));
        assert!(settings.contains("format_version: 4"));
        assert!(settings.contains("index_block_restart_interval: 16"));
    }
}

#[test]
fn test_read_options() {
    let mut read_opts = ReadOptions::default();
    read_opts.set_verify_checksums(false);
}
