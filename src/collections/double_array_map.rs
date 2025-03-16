#![allow(dead_code)]

use core::fmt;

const EXCEED_CAPACITY: &str = "Capacity exceeded!";

#[derive(Debug, PartialEq)]
pub enum DoubleArrayMapError {
    ExceedCapacity,
}

impl fmt::Display for DoubleArrayMapError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use DoubleArrayMapError as E;
        match self {
            E::ExceedCapacity => write!(f, "{EXCEED_CAPACITY}"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct DoubleArrayMap<const CAPACITY: usize, K, V> {
    size: usize,
    keys: [K; CAPACITY],
    values: [V; CAPACITY],
}

impl<const CAPACITY: usize, K, V> DoubleArrayMap<CAPACITY, K, V> {
    fn len(&self) -> usize {
        self.size
    }

    #[allow(clippy::unused_self)]
    fn get_capacity(&self) -> usize {
        CAPACITY
    }
}

impl<const CAPACITY: usize, K, V> DoubleArrayMap<CAPACITY, K, V>
where
    K: Default + Copy,
    V: Default + Copy,
{
    #[must_use]
    pub fn new() -> Self {
        Self {
            size: 0,
            keys: [K::default(); CAPACITY],
            values: [V::default(); CAPACITY],
        }
    }
}

impl<const CAPACITY: usize, K, V> Default for DoubleArrayMap<CAPACITY, K, V>
where
    K: Default + Copy,
    V: Default + Copy,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<const CAPACITY: usize, K, V> DoubleArrayMap<CAPACITY, K, V>
where
    K: Eq,
{
    /// # Errors
    ///
    /// Will return 'Err' if you try to insert more than the capacity.
    pub fn insert(&mut self, key: K, mut value: V) -> Result<Option<V>, DoubleArrayMapError> {
        for i in 0..self.size {
            if self.keys[i].eq(&key) {
                core::mem::swap(&mut self.values[i], &mut value);
                return Ok(Some(value));
            }
        }

        if self.size == CAPACITY {
            return Err(DoubleArrayMapError::ExceedCapacity);
        }

        self.keys[self.size] = key;
        self.values[self.size] = value;
        self.size += 1;

        Ok(None)
    }

    #[must_use]
    pub fn get(&self, key: &K) -> Option<&V> {
        for i in 0..self.size {
            if self.keys[i].eq(key) {
                return self.values.get(i);
            }
        }

        None
    }
}

#[cfg(test)]
mod double_array_map_test {
    use crate::assert_eq;

    use super::DoubleArrayMap as Dam;
    use super::*;

    #[test_case]
    fn test_insert_get_size_capacity() {
        let mut map: Dam<3, usize, &str> = Dam::new();
        assert_eq!(map.len(), 0);
        assert_eq!(map.get_capacity(), 3);

        assert_eq!(
            map.insert(0, "zero"),
            Ok::<Option<&str>, DoubleArrayMapError>(None::<&str>)
        );
        assert_eq!(
            map.insert(1, "un"),
            Ok::<Option<&str>, DoubleArrayMapError>(None::<&str>)
        );
        assert_eq!(
            map.insert(2, "deux"),
            Ok::<Option<&str>, DoubleArrayMapError>(None::<&str>)
        );
        assert_eq!(
            map.insert(3, "trois"),
            Err::<Option<&str>, DoubleArrayMapError>(DoubleArrayMapError::ExceedCapacity)
        );

        assert_eq!(map.len(), 3);

        assert_eq!(map.get(&0), Some(&"zero"));
        assert_eq!(map.get(&1), Some(&"un"));
        assert_eq!(map.get(&2), Some(&"deux"));
        assert_eq!(map.get(&3), None::<&&str>);

        assert_eq!(
            map.insert(1, "meilleur un"),
            Ok::<Option<&str>, DoubleArrayMapError>(Some("un"))
        );
    }
}
