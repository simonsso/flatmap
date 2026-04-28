use flatmap::{ConstantFlatMap, ConstantFlatSet, FlatMap, FlatMapEntry, FlatSet};

#[cfg(test)]
mod flatmap_tests {
    use super::*;

    #[test]
    fn test_flatmap_entry() {
        let entry = FlatMapEntry::new("key", 42);
        assert_eq!(entry.key(), &"key");
        assert_eq!(entry.value(), &42);

        let entry_from_tuple: FlatMapEntry<&str, i32> = ("key", 42).into();
        assert_eq!(entry_from_tuple.key(), &"key");
        assert_eq!(entry_from_tuple.value(), &42);

        let tuple: (&str, i32) = entry_from_tuple.into();
        assert_eq!(tuple, ("key", 42));
    }

    #[test]
    fn test_flatmap_basic_operations() {
        let mut map = FlatMap::new();

        assert_eq!(map.get(&"key"), None);
        assert_eq!(map.insert("key", 42), None);
        assert_eq!(map.get(&"key"), Some(&42));
        assert_eq!(map.insert("key", 100), Some(42));
        assert_eq!(map.get(&"key"), Some(&100));

        assert_eq!(map.delete(&"key"), Some(100));
        assert_eq!(map.get(&"key"), None);
        assert_eq!(map.delete(&"nonexistent"), None);
    }

    #[test]
    fn test_flatmap_from_entries() {
        let entries = vec![
            FlatMapEntry::new("a", 1),
            FlatMapEntry::new("b", 2),
            FlatMapEntry::new("a", 3), // duplicate key
        ];

        let map = FlatMap::from_entries(entries.into_iter());
        assert_eq!(map.get(&"a"), Some(&3)); // last value wins
        assert_eq!(map.get(&"b"), Some(&2));
    }

    #[test]
    fn test_flatmap_from_tuples() {
        let tuples = vec![("x", 10), ("y", 20)];
        let map: FlatMap<&str, i32> = tuples.into_iter().into();

        assert_eq!(map.get(&"x"), Some(&10));
        assert_eq!(map.get(&"y"), Some(&20));
    }

    #[test]
    fn test_flatmap_iteration() {
        let mut map = FlatMap::new();
        map.insert("a", 1);
        map.insert("b", 2);

        let mut count = 0;
        for entry in map.iter() {
            count += 1;
            assert!(entry.key() == &"a" || entry.key() == &"b");
        }
        assert_eq!(count, 2);

        // Note: Cannot modify values through iter_mut as fields are private
        // This would require a public setter method

        // Verify original values remain unchanged
        assert_eq!(map.get(&"a"), Some(&1));
        assert_eq!(map.get(&"b"), Some(&2));
    }

    #[test]
    fn test_flatmap_into_iterator() {
        let mut map = FlatMap::new();
        map.insert("x", 100);
        map.insert("y", 200);

        let mut collected: Vec<_> = map.into_iter().collect();
        collected.sort_by_key(|entry| *entry.key());

        assert_eq!(collected.len(), 2);
        assert_eq!(collected[0].key(), &"x");
        assert_eq!(collected[0].value(), &100);
    }

    #[test]
    fn test_constant_flatmap() {
        let entries = [FlatMapEntry::new("key1", 10), FlatMapEntry::new("key2", 20)];

        let map = ConstantFlatMap::from(entries);
        assert_eq!(map.get(&"key1"), Some(&10));
        assert_eq!(map.get(&"key2"), Some(&20));
        assert_eq!(map.get(&"nonexistent"), None);
    }

    #[test]
    fn test_constant_flatmap_from_tuples() {
        let map = ConstantFlatMap::from([("a", 1), ("b", 2)]);
        assert_eq!(map.get(&"a"), Some(&1));
        assert_eq!(map.get(&"b"), Some(&2));
    }

    #[test]
    fn test_constant_flatmap_duplicate_detection() {
        let entries = [
            FlatMapEntry::new("key", 10),
            FlatMapEntry::new("key", 20), // duplicate
        ];

        let result = ConstantFlatMap::from_entries(entries);
        assert!(result.is_err());
        if let Err(err) = result {
            assert_eq!(err, (0, 1));
        }
    }

    #[test]
    fn test_constant_flatmap_iteration() {
        let mut map = ConstantFlatMap::from([("x", 1), ("y", 2)]);

        let count = map.iter().count();
        assert_eq!(count, 2);

        // Note: Cannot modify values through iter_mut as fields are private
        let count_mut = map.iter_mut().count();
        assert_eq!(count_mut, 2);
    }
}

#[cfg(test)]
mod flatset_tests {
    use super::*;

    #[test]
    fn test_flatset_basic_operations() {
        let mut set = FlatSet::new();

        assert!(!set.has(&"item"));
        assert!(!set.insert("item"));
        assert!(set.has(&"item"));
        assert!(set.insert("item")); // already exists

        assert!(set.delete(&"item"));
        assert!(!set.has(&"item"));
        assert!(!set.delete(&"nonexistent"));
    }

    #[test]
    fn test_flatset_from_iter() {
        let items = vec!["a", "b", "a"]; // duplicate
        let set = FlatSet::from_iter(items.into_iter());

        assert!(set.has(&"a"));
        assert!(set.has(&"b"));

        let count = set.iter().count();
        assert_eq!(count, 2); // duplicates removed
    }

    #[test]
    fn test_flatset_from_iterator_trait() {
        let items = vec![1, 2, 3];
        let set: FlatSet<i32> = items.into_iter().into();

        assert!(set.has(&1));
        assert!(set.has(&2));
        assert!(set.has(&3));
        assert!(!set.has(&4));
    }

    #[test]
    fn test_flatset_iteration() {
        let mut set = FlatSet::new();
        set.insert(10);
        set.insert(20);

        let mut sum = 0;
        for item in set.iter() {
            sum += item;
        }
        assert_eq!(sum, 30);

        let collected: Vec<_> = set.into_iter().collect();
        assert_eq!(collected.len(), 2);
    }

    #[test]
    fn test_constant_flatset() {
        let set = unsafe { ConstantFlatSet::from_entries_unchecked([1, 2, 3]) };

        assert!(set.has(&1));
        assert!(set.has(&2));
        assert!(set.has(&3));
        assert!(!set.has(&4));
    }

    #[test]
    fn test_constant_flatset_duplicate_detection() {
        let result = ConstantFlatSet::from_entries([1, 2, 1]); // duplicate
        assert!(result.is_err());
        if let Err(err) = result {
            assert_eq!(err, (0, 2));
        }
    }

    #[test]
    fn test_constant_flatset_iteration() {
        let set = unsafe { ConstantFlatSet::from_entries_unchecked(["x", "y", "z"]) };

        let count = set.iter().count();
        assert_eq!(count, 3);

        let mut items: Vec<_> = set.iter().collect();
        items.sort();
        assert_eq!(items, vec![&"x", &"y", &"z"]);
    }
}

#[cfg(test)]
mod edge_cases {
    use super::*;

    #[test]
    fn test_empty_collections() {
        let map = FlatMap::<&str, i32>::new();
        assert_eq!(map.iter().count(), 0);

        let set = FlatSet::<i32>::new();
        assert_eq!(set.iter().count(), 0);
    }

    #[test]
    fn test_capacity_hint() {
        let map = FlatMap::<&str, i32>::with_capacity(10);
        let set = FlatSet::<i32>::with_capacity(10);

        // Just ensure they don't panic
        assert_eq!(map.iter().count(), 0);
        assert_eq!(set.iter().count(), 0);
    }

    #[test]
    fn test_shrink_to_fit() {
        let mut map = FlatMap::with_capacity(100);
        map.insert("key", 42);
        map.shrink_to_fit();
        assert_eq!(map.get(&"key"), Some(&42));

        let mut set = FlatSet::with_capacity(100);
        set.insert(42);
        set.shrink_to_fit();
        assert!(set.has(&42));
    }

    #[test]
    fn test_unsafe_constructors() {
        let entries = vec![FlatMapEntry::new("a", 1), FlatMapEntry::new("b", 2)];
        let map = unsafe { FlatMap::from_entries_unchecked(entries.into_iter()) };
        assert_eq!(map.get(&"a"), Some(&1));

        let items = vec![1, 2, 3];
        let set = unsafe { FlatSet::from_iter_unchecked(items.into_iter()) };
        assert!(set.has(&2));
    }
}
