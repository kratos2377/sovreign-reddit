//! Cache key/value definitions

use alloc::vec::Vec;
use core::fmt;

use sov_rollup_interface::maybestd::collections::hash_map::Entry;
use sov_rollup_interface::maybestd::collections::HashMap;
use alloc::sync::Arc as RefCount;

use crate::common::{MergeError, ReadError};
use crate::storage::{Storage, StorageKey, StorageValue};

/// A key for a cache set.
#[derive(Debug, Eq, PartialEq, Clone, Hash, PartialOrd, Ord)]
pub struct CacheKey {
    /// The key of the cache entry.
    pub key: RefCount<Vec<u8>>,
}

impl fmt::Display for CacheKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO revisit how we display keys
        write!(f, "{:?}", self.key)
    }
}

/// A value stored in the cache.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct CacheValue {
    /// The value of the cache entry.
    pub value: RefCount<Vec<u8>>,
}

impl fmt::Display for CacheValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO revisit how we display values
        write!(f, "{:?}", self.value)
    }
}

/// `Access` represents a sequence of events on a particular value.
/// For example, a transaction might read a value, then take some action which causes it to be updated
/// The rules for defining causality are as follows:
/// 1. If a read is preceded by another read, check that the two reads match and discard one.
/// 2. If a read is preceded by a write, check that the value read matches the value written. Discard the read.
/// 3. Otherwise, retain the read.
/// 4. A write is retained unless it is followed by another write.
#[derive(PartialEq, Eq, Debug, Clone)]
pub(crate) enum Access {
    Read(Option<CacheValue>),
    ReadThenWrite {
        original: Option<CacheValue>,
        modified: Option<CacheValue>,
    },
    Write(Option<CacheValue>),
}

impl Access {
    pub fn last_value(&self) -> &Option<CacheValue> {
        match self {
            Access::Read(value) => value,
            Access::ReadThenWrite { modified, .. } => modified,
            Access::Write(value) => value,
        }
    }

    pub fn write_value(&mut self, new_value: Option<CacheValue>) {
        match self {
            // If we've already read this slot, turn it into a readThenWrite access
            Access::Read(original) => {
                // If we're resetting the key to its original value, we can just discard the write history
                if original == &new_value {
                    return;
                }
                // Otherwise, keep track of the original value and the new value
                *self = Access::ReadThenWrite {
                    original: original.take(),

                    modified: new_value,
                };
            }
            // For ReadThenWrite override the modified value with a new value
            Access::ReadThenWrite { original, modified } => {
                // If we're resetting the key to its original value, we can just discard the write history
                if original == &new_value {
                    *self = Access::Read(new_value)
                } else {
                    *modified = new_value
                }
            }
            // For Write override the original value with a new value
            // We can do this unconditionally, since overwriting a value with itself is a no-op
            Access::Write(value) => *value = new_value,
        }
    }

    pub fn merge(&mut self, rhs: Self) -> Result<(), MergeError> {
        // Pattern matching on (`self`, rhs) is a bit cleaner, but would move the `self` inside the tuple.
        // We need the `self` later on for *self = Access.. therefore the nested solution.
        match self {
            Access::Read(left_read) => match rhs {
                Access::Read(right_read) => {
                    if left_read != &right_read {
                        Err(MergeError::ReadThenRead {
                            left: left_read.clone(),
                            right: right_read,
                        })
                    } else {
                        Ok(())
                    }
                }
                Access::ReadThenWrite {
                    original: right_original,
                    modified: right_modified,
                } => {
                    if left_read != &right_original {
                        Err(MergeError::ReadThenRead {
                            left: left_read.clone(),
                            right: right_original,
                        })
                    } else {
                        *self = Access::ReadThenWrite {
                            original: right_original,
                            modified: right_modified,
                        };

                        Ok(())
                    }
                }
                Access::Write(right_write) => {
                    *self = Access::ReadThenWrite {
                        original: left_read.take(),
                        modified: right_write,
                    };
                    Ok(())
                }
            },
            Access::ReadThenWrite {
                original: left_original,
                modified: left_modified,
            } => match rhs {
                Access::Read(right_read) => {
                    if left_modified != &right_read {
                        Err(MergeError::WriteThenRead {
                            write: left_modified.clone(),
                            read: right_read,
                        })
                    } else {
                        Ok(())
                    }
                }
                Access::ReadThenWrite {
                    original: right_original,
                    modified: right_modified,
                } => {
                    if left_modified != &right_original {
                        Err(MergeError::WriteThenRead {
                            write: left_modified.clone(),
                            read: right_original,
                        })
                    } else {
                        *self = Access::ReadThenWrite {
                            original: left_original.take(),
                            modified: right_modified,
                        };
                        Ok(())
                    }
                }
                Access::Write(right_write) => {
                    *self = Access::ReadThenWrite {
                        original: left_original.take(),
                        modified: right_write,
                    };
                    Ok(())
                }
            },
            Access::Write(left_write) => match rhs {
                Access::Read(right_read) => {
                    if left_write != &right_read {
                        Err(MergeError::WriteThenRead {
                            write: left_write.clone(),
                            read: right_read,
                        })
                    } else {
                        Ok(())
                    }
                }
                Access::ReadThenWrite {
                    original: right_original,
                    modified: right_modified,
                } => {
                    if left_write != &right_original {
                        Err(MergeError::WriteThenRead {
                            write: left_write.clone(),
                            read: right_original,
                        })
                    } else {
                        *self = Access::Write(right_modified);
                        Ok(())
                    }
                }
                Access::Write(right_write) => {
                    *self = Access::Write(right_write);
                    Ok(())
                }
            },
        }
    }
}

/// Cache entry can be in three states:
/// - Does not exists, a given key was never inserted in the cache:
///     ValueExists::No
/// - Exists but the value is empty.
///      ValueExists::Yes(None)
/// - Exists and contains a value:
///     ValueExists::Yes(Some(value))
pub enum ValueExists {
    /// The key exists in the cache.
    Yes(Option<CacheValue>),
    /// The key does not exist in the cache.
    No,
}

/// CacheLog keeps track of the original and current values of each key accessed.
/// By tracking original values, we can detect and eliminate write patterns where a key is
/// changed temporarily and then reset to its original value
#[derive(Default)]
pub struct CacheLog {
    log: HashMap<CacheKey, Access>,
}

impl CacheLog {
    /// Creates a cache log with the provided map capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            log: HashMap::with_capacity(capacity),
        }
    }
}

impl CacheLog {
    /// Returns the owned set of key/value pairs of the cache.
    pub fn take_writes(self) -> Vec<(CacheKey, Option<CacheValue>)> {
        self.log
            .into_iter()
            .filter_map(|(k, v)| match v {
                Access::Read(_) => None,
                Access::ReadThenWrite { modified, .. } => Some((k, modified)),
                Access::Write(write) => Some((k, write)),
            })
            .collect()
    }

    /// Returns a value corresponding to the key.
    pub fn get_value(&self, key: &CacheKey) -> ValueExists {
        match self.log.get(key) {
            Some(value) => ValueExists::Yes(value.last_value().clone()),
            None => ValueExists::No,
        }
    }

    /// The first read for a given key is inserted in the cache. For an existing cache entry
    /// checks if reads are consistent with previous reads/writes.
    pub fn add_read(&mut self, key: CacheKey, value: Option<CacheValue>) -> Result<(), ReadError> {
        match self.log.entry(key) {
            Entry::Occupied(existing) => {
                let last_value = existing.get().last_value().clone();

                if last_value != value {
                    return Err(ReadError::InconsistentRead {
                        expected: last_value,
                        found: value,
                    });
                }
                Ok(())
            }
            Entry::Vacant(vacancy) => {
                vacancy.insert(Access::Read(value));
                Ok(())
            }
        }
    }

    /// Adds a write entry to the cache.
    pub fn add_write(&mut self, key: CacheKey, value: Option<CacheValue>) {
        match self.log.entry(key) {
            Entry::Occupied(mut existing) => {
                existing.get_mut().write_value(value);
            }
            Entry::Vacant(vacancy) => {
                vacancy.insert(Access::Write(value));
            }
        }
    }

    /// Merges two cache logs in a way that preserves the first read (from self) and the last write (from rhs)
    /// for the same key in both caches.
    /// The merge succeeds if the first read in the right cache for a key 'k' is consistent with the last read/write
    /// in the self cache.
    ///
    /// Example:
    ///
    /// Cache1:        Cache2:
    ///     k1 => v1       k1 => v1'
    ///     k2 => v2       k3 => v3
    ///
    /// Merged Cache:
    ///     k1 => v1.merge(v1') <- preserves the first read and the last write for 'k1'
    ///     k2 => v2
    ///     k3 => v3
    pub fn merge_left(&mut self, rhs: Self) -> Result<(), MergeError> {
        self.merge_left_with_filter_map(rhs, Some)
    }

    /// Merges two cache logs in a way that preserves the first read (from self) and the last write (from rhs).
    pub fn merge_writes_left(&mut self, rhs: Self) -> Result<(), MergeError> {
        self.merge_left_with_filter_map(rhs, |(key, access)| match access {
            Access::Read(_) => None,
            Access::ReadThenWrite { modified, .. } => Some((key, Access::Write(modified))),
            Access::Write(w) => Some((key, Access::Write(w))),
        })
    }

    /// Merges two cache logs in a way that preserves the first read (from self) and the last write (from rhs).
    pub fn merge_reads_left(&mut self, rhs: Self) -> Result<(), MergeError> {
        self.merge_left_with_filter_map(rhs, |(key, access)| match access {
            Access::Read(read) => Some((key, Access::Read(read))),
            Access::ReadThenWrite { original, .. } => Some((key, Access::Read(original))),
            Access::Write(_) => None,
        })
    }

    fn merge_left_with_filter_map<F: FnMut((CacheKey, Access)) -> Option<(CacheKey, Access)>>(
        &mut self,
        rhs: Self,
        filter: F,
    ) -> Result<(), MergeError> {
        for (rhs_key, rhs_access) in rhs.log.into_iter().filter_map(filter) {
            match self.log.get_mut(&rhs_key) {
                Some(self_access) => self_access.merge(rhs_access)?,
                None => {
                    self.log.insert(rhs_key, rhs_access);
                }
            };
        }
        Ok(())
    }

    /// Returns the number of entries in the cache.
    pub fn len(&self) -> usize {
        self.log.len()
    }

    /// Returns `true` if the cache is empty, `false` otherwise.
    pub fn is_empty(&self) -> bool {
        self.log.is_empty()
    }
}

/// Caches reads and writes for a (key, value) pair. On the first read the value is fetched
/// from an external source represented by the `ValueReader` trait. On following reads,
/// the cache checks if the value we read was inserted before.
#[derive(Default)]
pub struct StorageInternalCache {
    /// Transaction cache.
    pub tx_cache: CacheLog,
    /// Ordered reads and writes.
    pub ordered_db_reads: Vec<(CacheKey, Option<CacheValue>)>,
    /// Version for versioned usage with cache
    pub version: Option<u64>,
}

impl StorageInternalCache {
    /// Wrapper around default that can create the cache with knowledge of the version
    pub fn new_with_version(version: u64) -> Self {
        StorageInternalCache {
            version: Some(version),
            ..Default::default()
        }
    }

    /// Gets a value from the cache or reads it from the provided `ValueReader`.
    pub fn get_or_fetch<S: Storage>(
        &mut self,
        key: &StorageKey,
        value_reader: &S,
        witness: &S::Witness,
    ) -> Option<StorageValue> {
        let cache_key = key.to_cache_key_version(self.version);
        let cache_value = self.get_value_from_cache(&cache_key);

        match cache_value {
            ValueExists::Yes(cache_value_exists) => cache_value_exists.map(Into::into),
            // If the value does not exist in the cache, then fetch it from an external source.
            ValueExists::No => {
                let storage_value = value_reader.get(key, self.version, witness);
                let cache_value = storage_value.as_ref().map(|v| v.clone().into_cache_value());

                self.add_read(cache_key, cache_value);
                storage_value
            }
        }
    }

    /// Gets a keyed value from the cache, returning a wrapper on whether it exists.
    pub fn try_get(&self, key: &StorageKey) -> ValueExists {
        let cache_key = key.to_cache_key_version(self.version);
        self.get_value_from_cache(&cache_key)
    }

    /// Replaces the keyed value on the storage.
    pub fn set(&mut self, key: &StorageKey, value: StorageValue) {
        let cache_key = key.to_cache_key_version(self.version);
        let cache_value = value.into_cache_value();
        self.tx_cache.add_write(cache_key, Some(cache_value));
    }

    /// Deletes a keyed value from the cache.
    pub fn delete(&mut self, key: &StorageKey) {
        let cache_key = key.to_cache_key_version(self.version);
        self.tx_cache.add_write(cache_key, None);
    }

    fn get_value_from_cache(&self, cache_key: &CacheKey) -> ValueExists {
        self.tx_cache.get_value(cache_key)
    }

    /// Merges the provided `StorageInternalCache` into this one.
    pub fn merge_left(&mut self, rhs: Self) -> Result<(), MergeError> {
        self.tx_cache.merge_left(rhs.tx_cache)
    }

    /// Merges the reads of the provided `StorageInternalCache` into this one.
    pub fn merge_reads_left(&mut self, rhs: Self) -> Result<(), MergeError> {
        self.tx_cache.merge_reads_left(rhs.tx_cache)
    }

    /// Merges the writes of the provided `StorageInternalCache` into this one.
    pub fn merge_writes_left(&mut self, rhs: Self) -> Result<(), MergeError> {
        self.tx_cache.merge_writes_left(rhs.tx_cache)
    }

    fn add_read(&mut self, key: CacheKey, value: Option<CacheValue>) {
        self.tx_cache
            .add_read(key.clone(), value.clone())
            // It is ok to panic here, we must guarantee that the cache is consistent.
            .unwrap_or_else(|e| panic!("Inconsistent read from the cache: {e:?}"));
        self.ordered_db_reads.push((key, value))
    }
}

/// A struct that contains the values read from the DB and the values to be written, both in
/// deterministic order.
#[derive(Debug, Default)]
pub struct OrderedReadsAndWrites {
    /// Ordered reads.
    pub ordered_reads: Vec<(CacheKey, Option<CacheValue>)>,
    /// Ordered writes.
    pub ordered_writes: Vec<(CacheKey, Option<CacheValue>)>,
}

impl From<StorageInternalCache> for OrderedReadsAndWrites {
    fn from(val: StorageInternalCache) -> Self {
        let mut writes = val.tx_cache.take_writes();
        // TODO: Make this more efficient
        writes.sort_by(|(k1, _), (k2, _)| k1.cmp(k2));
        Self {
            ordered_reads: val.ordered_db_reads,
            ordered_writes: writes,
        }
    }
}