use alloc::vec::{IntoIter, Vec};
use core::fmt::{self, Debug};
use core::mem;

pub struct FlatMapEntry<K, V> {
    key: K,
    value: V,
}

impl<K, V> FlatMapEntry<K, V> {
    #[inline(always)]
    pub const fn new(key: K, value: V) -> Self {
        Self { key, value }
    }

    #[inline(always)]
    pub const fn key(&self) -> &K {
        &self.key
    }

    #[inline(always)]
    pub const fn value(&self) -> &V {
        &self.value
    }

    pub fn value_mut(&mut self) -> &mut V {
        &mut self.value
    }
}

impl<K: Debug, V: Debug> Debug for FlatMapEntry<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FlatMapEntry")
            .field("key", &self.key)
            .field("value", &self.value)
            .finish()
    }
}

impl<K: Clone, V: Clone> Clone for FlatMapEntry<K, V> {
    fn clone(&self) -> Self {
        Self {
            key: self.key.clone(),
            value: self.value.clone(),
        }
    }
}

impl<K, V> From<(K, V)> for FlatMapEntry<K, V> {
    fn from(value: (K, V)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl<K, V> Into<(K, V)> for FlatMapEntry<K, V> {
    fn into(self) -> (K, V) {
        (self.key, self.value)
    }
}

/// Linear Map with no sorting guarantee and no duplicate entries
pub struct FlatMap<K: Eq, V> {
    inner: Vec<FlatMapEntry<K, V>>,
}

impl<K: Eq + Debug, V: Debug> Debug for FlatMap<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FlatMap")
            .field("inner", &self.inner)
            .finish()
    }
}

impl<K: Eq + Clone, V: Clone> Clone for FlatMap<K, V> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<K: Eq, V> Default for FlatMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Eq, V> FlatMap<K, V>
where
    K: Eq,
{
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: Vec::with_capacity(capacity),
        }
    }

    /// inserts entries by checking for duplicates for every item
    /// if duplicate items are found, last one stays while earlier gets discarded
    /// not recommended for large list of entries, check for duplicates yourself and use FlatMap::from_entries_unchecked
    pub fn from_entries(iter: impl Iterator<Item = FlatMapEntry<K, V>>) -> Self
    where
        K: Ord,
    {
        let (cap, _) = iter.size_hint();
        let mut s = Self::with_capacity(cap);
        for entry in iter {
            s.insert(entry.key, entry.value);
        }
        s
    }

    /// construct inner vec without checking for duplicates
    pub unsafe fn from_entries_unchecked(iter: impl Iterator<Item = FlatMapEntry<K, V>>) -> Self {
        Self {
            inner: iter.collect(),
        }
    }

    pub fn get(&self, k: &K) -> Option<&V> {
        for entry in &self.inner {
            if &entry.key == k {
                return Some(&entry.value);
            }
        }

        None
    }

    pub fn insert(&mut self, k: K, v: V) -> Option<V>
    where
        K: Ord,
    {
        for entry in &mut self.inner {
            if &entry.key == &k {
                let mut new_value = v;
                mem::swap(&mut entry.value, &mut new_value);
                return Some(new_value);
            }
        }

        self.inner.push(FlatMapEntry::new(k, v));
        self.inner.sort_by(|a, b| a.key.cmp(&b.key));

        None
    }

    pub fn delete(&mut self, k: &K) -> Option<V> {
        for i in 0..self.inner.len() {
            if &self.inner[i].key == k {
                let value = self.inner.swap_remove(i);
                return Some(value.value);
            }
        }

        None
    }

    pub fn iter(&self) -> impl Iterator<Item = &FlatMapEntry<K, V>> {
        self.inner.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut FlatMapEntry<K, V>> {
        self.inner.iter_mut()
    }

    pub fn shrink_to_fit(&mut self) {
        self.inner.shrink_to_fit();
    }
}

impl<K: Eq + Ord, V, I> From<I> for FlatMap<K, V>
where
    I: Iterator<Item = (K, V)>,
{
    fn from(value: I) -> Self {
        Self::from_entries(value.map(FlatMapEntry::from))
    }
}

impl<K: Eq, V> IntoIterator for FlatMap<K, V> {
    type Item = FlatMapEntry<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }

    type IntoIter = IntoIter<FlatMapEntry<K, V>>;
}

pub struct ConstantFlatMap<K: Eq, V, const N: usize> {
    inner: [FlatMapEntry<K, V>; N],
}

impl<K: Eq, V, const N: usize> From<[FlatMapEntry<K, V>; N]> for ConstantFlatMap<K, V, N> {
    fn from(entries: [FlatMapEntry<K, V>; N]) -> Self {
        Self { inner: entries }
    }
}

impl<K: Eq, V, const N: usize> From<[(K, V); N]> for ConstantFlatMap<K, V, N> {
    fn from(entries: [(K, V); N]) -> Self {
        Self {
            inner: entries.map(FlatMapEntry::from),
        }
    }
}

impl<K: Eq + Debug, V: Debug, const N: usize> Debug for ConstantFlatMap<K, V, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ConstantFlatMap")
            .field("inner", &self.inner)
            .finish()
    }
}

impl<K: Eq + Clone, V: Clone, const N: usize> Clone for ConstantFlatMap<K, V, N> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<K: Eq + Ord, V, const N: usize> ConstantFlatMap<K, V, N> {
    pub fn get(&self, key: &K) -> Option<&V> {
        self.inner
            .binary_search_by(|entry| entry.key().cmp(key))
            .ok()
            .map(|i| self.inner[i].value())
    }

    /// checks for duplicates, if found will return the indices of duplicate
    /// not recommended for large list of entries, check for duplicates yourself and use ConstantFlatMap::from_entries_unchecked
    pub fn from_entries(entries: [FlatMapEntry<K, V>; N]) -> Result<Self, (usize, usize)> {
        for i in 0..N {
            for j in (i + 1)..N {
                if entries[i].key == entries[j].key {
                    return Err((i, j));
                }
            }
        }

        Ok(unsafe { Self::from_entries_unchecked(entries) })
    }

    /// construct Map without checking for duplicates
    pub unsafe fn from_entries_unchecked(entries: [FlatMapEntry<K, V>; N]) -> Self {
        Self { inner: entries }
    }

    pub fn iter(&self) -> impl Iterator<Item = &FlatMapEntry<K, V>> {
        self.inner.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut FlatMapEntry<K, V>> {
        self.inner.iter_mut()
    }
}
