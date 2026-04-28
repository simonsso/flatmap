use alloc::vec::{IntoIter, Vec};
use core::fmt::{self, Debug};

pub struct FlatSet<K: Eq> {
    inner: Vec<K>,
}

impl<K: Eq + Debug> Debug for FlatSet<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FlatSet")
            .field("inner", &self.inner)
            .finish()
    }
}

impl<K: Eq + Clone> Clone for FlatSet<K> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<K: Eq> Default for FlatSet<K> {
    fn default() -> Self {
        Self::new()
    }
}

/// Linear Set with no sorting guarantees
impl<K: Eq> FlatSet<K> {
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: Vec::with_capacity(capacity),
        }
    }

    /// not recommended for large number of items
    /// use FlatSet::from_iter_unchecked after removing duplicates
    pub fn from_iter(iter: impl Iterator<Item = K>) -> Self {
        let mut s = Self::with_capacity(iter.size_hint().0);

        for item in iter {
            s.insert(item);
        }

        s
    }

    /// constructs FlatSet without checking for duplicates
    pub unsafe fn from_iter_unchecked(iter: impl Iterator<Item = K>) -> Self {
        {
            Self {
                inner: iter.collect(),
            }
        }
    }

    pub fn has(&self, key: &K) -> bool {
        for item in self.inner.iter() {
            if item == key {
                return true;
            }
        }

        false
    }

    // returns true if already exists
    pub fn insert(&mut self, key: K) -> bool {
        if self.has(&key) {
            return true;
        }

        self.inner.push(key);
        false
    }

    // returns true if key exists
    pub fn delete(&mut self, key: &K) -> bool {
        for i in 0..self.inner.len() {
            if &self.inner[i] == key {
                self.inner.swap_remove(i);
                return true;
            }
        }

        false
    }

    pub fn shrink_to_fit(&mut self) {
        self.inner.shrink_to_fit();
    }

    pub fn iter(&self) -> impl Iterator<Item = &K> {
        self.inner.iter()
    }
}

impl<K: Eq> IntoIterator for FlatSet<K> {
    type Item = K;

    type IntoIter = IntoIter<K>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<K: Eq, I> From<I> for FlatSet<K>
where
    I: Iterator<Item = K>,
{
    fn from(value: I) -> Self {
        Self::from_iter(value)
    }
}

pub struct ConstantFlatSet<K: Eq, const N: usize> {
    inner: [K; N],
}

impl<K: Eq + Debug, const N: usize> Debug for ConstantFlatSet<K, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ConstantFlatSet")
            .field("inner", &self.inner)
            .finish()
    }
}

impl<K: Eq + Clone, const N: usize> Clone for ConstantFlatSet<K, N> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<K: Eq, const N: usize> ConstantFlatSet<K, N> {
    pub fn has(&self, key: &K) -> bool {
        for item in self.inner.iter() {
            if item == key {
                return true;
            }
        }

        false
    }

    /// checks for duplicates, if found will return the indices of duplicate
    /// not recommended for large list of entries, check for duplicates yourself and use ConstantFlatSet::from_entries_unchecked
    pub fn from_entries(entries: [K; N]) -> Result<Self, (usize, usize)> {
        for i in 0..N {
            for j in (i + 1)..N {
                if entries[i] == entries[j] {
                    return Err((i, j));
                }
            }
        }

        Ok(unsafe { Self::from_entries_unchecked(entries) })
    }

    /// construct Set without checking for duplicates
    pub unsafe fn from_entries_unchecked(entries: [K; N]) -> Self {
        Self { inner: entries }
    }

    pub fn iter(&self) -> impl Iterator<Item = &K> {
        self.inner.iter()
    }
}
