// Copyright 2021 Datafuse Labs
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

use common_exception::Result;
use common_hashtable::FastHash;

use crate::types::string::StringIterator;
use crate::types::DataType;
use crate::Column;
use crate::HashMethod;
use crate::KeysState;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HashMethodSingleString {}

impl HashMethod for HashMethodSingleString {
    type HashKey = [u8];

    type HashKeyIter<'a> = StringIterator<'a>;

    fn name(&self) -> String {
        "SingleString".to_string()
    }

    fn build_keys_state(
        &self,
        group_columns: &[(Column, DataType)],
        _rows: usize,
    ) -> Result<KeysState> {
        Ok(KeysState::Column(group_columns[0].0.clone()))
    }

    fn build_keys_iter<'a>(&self, keys_state: &'a KeysState) -> Result<Self::HashKeyIter<'a>> {
        match keys_state {
            KeysState::Column(Column::String(col))
            | KeysState::Column(Column::Variant(col))
            | KeysState::Column(Column::Bitmap(col)) => Ok(col.iter()),
            _ => unreachable!(),
        }
    }

    fn build_keys_iter_and_hashes<'a>(
        &self,
        keys_state: &'a KeysState,
    ) -> Result<(Self::HashKeyIter<'a>, Vec<u64>)> {
        match keys_state {
            KeysState::Column(Column::String(col))
            | KeysState::Column(Column::Variant(col))
            | KeysState::Column(Column::Bitmap(col)) => {
                let mut hashes = Vec::with_capacity(col.len());
                hashes.extend(col.iter().map(|key| key.fast_hash()));
                Ok((col.iter(), hashes))
            }
            _ => unreachable!(),
        }
    }
}
