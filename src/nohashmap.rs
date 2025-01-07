use itertools::{izip, Itertools};
use std::cmp::min;
use std::collections::TryReserveError;
use std::fmt;
use std::fmt::Debug;
use std::iter::Zip;
use std::slice::{Iter, IterMut};
use std::vec::{Drain, IntoIter};

#[derive(Clone, Default)]
/// A NoHashMap using two Vec's to collect keys and values separately
///
/// # Examples
///
/// ```
///  use itertools::assert_equal;
///  use nohashmap::NoHashMapMultiVec;
///  let mut nhm = NoHashMapMultiVec::new();
///  for (k, v) in vec![(0.1, 1.2), (2.3, 3.4), (4.5, 5.6), (6.7, 7.8)].into_iter() {
///      nhm.insert(k, v);
///     }
///  assert_equal(nhm.clone().into_keys(), vec![0.1, 2.3, 4.5, 6.7].into_iter());
///  assert_equal(nhm.into_values(), vec![1.2, 3.4, 5.6, 7.8].into_iter());
/// ```
pub struct NoHashMapMultiVec<K, V> {
    vec_k: Vec<K>,
    vec_v: Vec<V>,
}

impl<K, V> NoHashMapMultiVec<K, V> {
    pub fn iter(&self) -> Zip<Iter<'_, K>, Iter<'_, V>> {
        izip!(self.vec_k.iter(), self.vec_v.iter())
    }

    pub fn iter_mut(&mut self) -> Zip<IterMut<'_, K>, IterMut<'_, V>> {
        izip!(self.vec_k.iter_mut(), self.vec_v.iter_mut())
    }

    pub fn len(&self) -> usize {
        self.vec_k.len()
    }

    pub fn values(&self) -> &Vec<V> {
        &self.vec_v
    }

    pub fn values_mut(&mut self) -> IterMut<'_, V> {
        self.vec_v.iter_mut()
    }

    pub fn into_values(self) -> IntoIter<V> {
        self.vec_v.into_iter()
    }

    pub fn keys(&self) -> &Vec<K> {
        &self.vec_k
    }

    pub fn keys_mut(&mut self) -> IterMut<'_, K> {
        self.vec_k.iter_mut()
    }

    pub fn into_keys(self) -> IntoIter<K> {
        self.vec_k.into_iter()
    }

    pub fn is_empty(&self) -> bool {
        self.vec_k.is_empty()
    }

    pub fn capacity(&self) -> usize {
        min(self.vec_k.capacity(), self.vec_v.capacity())
    }

    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.vec_k.shrink_to(min_capacity);
        self.vec_v.shrink_to(min_capacity);
    }

    pub fn shrink_to_fit(&mut self) {
        self.vec_k.shrink_to_fit();
        self.vec_v.shrink_to_fit();
    }

    pub fn reserve(&mut self, additional: usize) {
        self.vec_k.reserve(additional);
        self.vec_v.reserve(additional);
    }

    pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.vec_k.try_reserve(additional)?;
        Ok(self.vec_v.try_reserve(additional)?)
    }

    pub fn clear(&mut self) {
        self.vec_k.clear();
        self.vec_v.clear();
    }

    pub fn drain(&mut self) -> Zip<Drain<'_, K>, Drain<'_, V>> {
        izip!(self.vec_k.drain(..), self.vec_v.drain(..))
    }
}

impl<K: Copy, V: Copy> NoHashMapMultiVec<K, V> {
    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&K, &V) -> bool,
    {
        (self.vec_k, self.vec_v) = self
            .iter()
            .filter(|x| f(x.0, x.1))
            .collect_vec()
            .into_iter()
            .multiunzip();
    }
}

impl<K: Default, V: Default> NoHashMapMultiVec<K, V> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let mut this: Self = Default::default();
        this.reserve(capacity);
        this
    }
}

impl<K: PartialEq, V> NoHashMapMultiVec<K, V> {
    pub fn insert(&mut self, k: K, v: V) {
        let loc_k = self.vec_k.iter().position(|x| *x == k);
        match loc_k {
            None => {
                self.vec_k.push(k);
                self.vec_v.push(v);
            }
            Some(loc) => {
                self.vec_v[loc] = v;
            }
        }
    }

    // For future when this is stable
    // pub fn try_insert(&mut self, k: K, v: V) -> Result<&mut V, OccupiedError<'_, K, V>> {
    //     let loc_k = self.vec_k.iter().position(|x| *x == k);
    //     match loc_k {
    //         None => {
    //             self.vec_k.push(k);
    //             self.vec_v.push(v);
    //             Ok(self.vec_v.last_mut().unwrap())
    //         }
    //         Some(loc) => Err(OccupiedError)
    //     }
    // }

    pub fn contains_key(&self, k: &K) -> bool {
        let loc_k = self.vec_k.iter().position(|x| *x == *k);
        match loc_k {
            None => false,
            Some(_) => true,
        }
    }

    pub fn get(&self, k: &K) -> Option<&V> {
        let loc_k = self.vec_k.iter().position(|x| *x == *k);
        match loc_k {
            None => None,
            Some(loc) => self.vec_v.get(loc),
        }
    }

    pub fn get_mut(&mut self, k: &K) -> Option<&mut V> {
        let loc_k = self.vec_k.iter().position(|x| *x == *k);
        match loc_k {
            None => None,
            Some(loc) => self.vec_v.get_mut(loc),
        }
    }

    pub fn get_key_value<'a>(&'a self, k: &'a K) -> Option<(&'a K, &'a V)> {
        let loc_k = self.vec_k.iter().position(|x| *x == *k);
        match loc_k {
            None => None,
            Some(loc) => Some((k, self.vec_v.get(loc).unwrap())),
        }
    }

    pub fn remove(&mut self, k: &K) -> Option<V> {
        let loc_k = self.vec_k.iter().position(|x| *x == *k);
        match loc_k {
            None => None,
            Some(loc) => {
                self.vec_k.remove(loc);
                Some(self.vec_v.remove(loc))
            }
        }
    }

    pub fn swap_remove(&mut self, k: &K) -> Option<V> {
        let loc_k = self.vec_k.iter().position(|x| *x == *k);
        match loc_k {
            None => None,
            Some(loc) => {
                self.vec_k.swap_remove(loc);
                Some(self.vec_v.swap_remove(loc))
            }
        }
    }
}

impl<K, V> Debug for NoHashMapMultiVec<K, V>
where
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

#[derive(Clone, Default)]
/// A NoHashMap using a Vec of a key-value tuple to collect them jointly
///
/// # Examples
///
/// ```
///  use itertools::assert_equal;
///  use nohashmap::NoHashMapVecTuple;
///  let mut nhm = NoHashMapVecTuple::new();
///  for (k, v) in vec![(0.1, 1.2), (2.3, 3.4), (4.5, 5.6), (6.7, 7.8)].into_iter() {
///      nhm.insert(k, v);
///     }
///  assert_equal(nhm.clone().into_keys(), vec![0.1, 2.3, 4.5, 6.7].into_iter());
///  assert_equal(nhm.into_values(), vec![1.2, 3.4, 5.6, 7.8].into_iter());
/// ```
pub struct NoHashMapVecTuple<K, V> {
    vec: Vec<(K, V)>,
}

impl<K, V> NoHashMapVecTuple<K, V> {
    pub fn iter(&self) -> Iter<'_, (K, V)> {
        self.vec.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, (K, V)> {
        self.vec.iter_mut()
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn values(&self) -> Vec<&V> {
        self.vec.iter().map(|x| &x.1).collect_vec()
    }

    pub fn values_mut<'a>(&mut self) -> IntoIter<&mut V> {
        self.vec
            .iter_mut()
            .map(|(_, v)| v)
            .collect_vec()
            .into_iter()
    }

    pub fn into_values(self) -> IntoIter<V> {
        self.vec.into_iter().map(|x| x.1).collect_vec().into_iter()
    }

    pub fn keys(&self) -> Vec<&K> {
        self.vec.iter().map(|x| &x.0).collect_vec()
    }

    pub fn keys_mut<'a>(&mut self) -> IntoIter<&mut K> {
        self.vec
            .iter_mut()
            .map(|(k, _)| k)
            .collect_vec()
            .into_iter()
    }

    pub fn into_keys<'a>(self) -> IntoIter<K> {
        self.vec.into_iter().map(|x| x.0).collect_vec().into_iter()
    }

    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    pub fn capacity(&self) -> usize {
        self.vec.capacity()
    }

    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.vec.shrink_to(min_capacity);
    }

    pub fn shrink_to_fit(&mut self) {
        self.vec.shrink_to_fit();
    }

    pub fn reserve(&mut self, additional: usize) {
        self.vec.reserve(additional)
    }

    pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.vec.try_reserve(additional)
    }

    pub fn clear(&mut self) {
        self.vec.clear();
    }

    pub fn drain(&mut self) -> Drain<'_, (K, V)> {
        self.vec.drain(..)
    }

    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&K, &V) -> bool,
    {
        self.vec.retain(|x| f(&x.0, &x.1))
    }
}

impl<K: Default, V: Default> NoHashMapVecTuple<K, V> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let mut this: Self = Default::default();
        this.reserve(capacity);
        this
    }
}

impl<K: PartialEq, V> NoHashMapVecTuple<K, V> {
    pub fn insert(&mut self, k: K, v: V) {
        let loc_k = self.vec.iter().position(|x| x.0 == k);
        match loc_k {
            None => {
                self.vec.push((k, v));
            }
            Some(loc) => {
                self.vec[loc] = (k, v);
            }
        }
    }
    pub fn contains_key(&self, k: &K) -> bool {
        let loc = self.vec.iter().position(|x| x.0 == *k);
        match loc {
            None => false,
            Some(_) => true,
        }
    }

    pub fn get(&self, k: &K) -> Option<&V> {
        let loc = self.vec.iter().position(|x| x.0 == *k);
        match loc {
            None => None,
            Some(loc) => Some(&self.vec.get(loc).unwrap().1),
        }
    }

    pub fn get_mut(&mut self, k: &K) -> Option<&mut V> {
        let loc = self.vec.iter().position(|x| x.0 == *k);
        match loc {
            None => None,
            Some(loc) => Some(&mut self.vec.get_mut(loc).unwrap().1),
        }
    }

    pub fn get_key_value(&self, k: &K) -> Option<&(K, V)> {
        let loc = self.vec.iter().position(|x| x.0 == *k);
        match loc {
            None => None,
            Some(loc) => self.vec.get(loc),
        }
    }
    pub fn remove(&mut self, k: &K) -> Option<V> {
        let loc = self.vec.iter().position(|x| x.0 == *k);
        match loc {
            None => None,
            Some(loc) => Some(self.vec.remove(loc).1),
        }
    }

    pub fn swap_remove(&mut self, k: &K) -> Option<V> {
        let loc = self.vec.iter().position(|x| x.0 == *k);
        match loc {
            None => None,
            Some(loc) => Some(self.vec.swap_remove(loc).1),
        }
    }
}

impl<K, V> Debug for NoHashMapVecTuple<K, V>
where
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map()
            .entries(self.vec.iter().map(|&(ref k, ref v)| (k, v)))
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::assert_equal;
    use std::collections::HashMap;

    #[test]
    fn test_insert_get() {
        let mut nhmmv = NoHashMapMultiVec::new();
        let mut nhmvt = NoHashMapVecTuple::new();
        nhmmv.insert(0, 1);
        nhmvt.insert(0, 1);
        assert_eq!(nhmmv.get(&0), Some(&1));
        assert_eq!(nhmvt.get(&0), Some(&1));
    }

    #[test]
    fn test_get_mut() {
        let mut nhmmv = NoHashMapMultiVec::new();
        let mut nhmvt = NoHashMapVecTuple::new();
        nhmmv.insert(0, 1);
        nhmvt.insert(0, 1);
        *nhmmv.get_mut(&0).unwrap() = 3;
        assert_eq!(nhmmv.get(&0), Some(&3));
        *nhmvt.get_mut(&0).unwrap() = 3;
        assert_eq!(nhmvt.get(&0), Some(&3));
    }

    #[test]
    fn test_get_key_value() {
        let mut nhmmv = NoHashMapMultiVec::new();
        let mut nhmvt = NoHashMapVecTuple::new();
        nhmmv.insert(0, 1);
        nhmvt.insert(0, 1);
        assert_eq!(nhmmv.get_key_value(&0), Some((&0, &1)));
        assert_eq!(nhmvt.get_key_value(&0), Some(&(0, 1)));
    }

    #[test]
    fn test_remove() {
        let mut nhmmv = NoHashMapMultiVec::new();
        let mut nhmvt = NoHashMapVecTuple::new();
        nhmmv.insert(0, 1);
        nhmvt.insert(0, 1);
        nhmmv.remove(&0);
        nhmvt.remove(&0);
        assert_eq!(nhmmv.get(&0), None);
        assert_eq!(nhmvt.get(&0), None);
    }

    #[test]
    fn test_swap_remove() {
        let mut nhmmv = NoHashMapMultiVec::new();
        let mut nhmvt = NoHashMapVecTuple::new();
        nhmmv.insert(0, 1);
        nhmvt.insert(0, 1);
        nhmmv.swap_remove(&0);
        nhmvt.swap_remove(&0);
        assert_eq!(nhmmv.get(&0), None);
        assert_eq!(nhmvt.get(&0), None);
    }

    #[test]
    fn test_float() {
        let mut nhmmv = NoHashMapMultiVec::new();
        let mut nhmvt = NoHashMapVecTuple::new();

        for (k, v) in vec![(0.1, 1.2), (2.3, 3.4), (4.5, 5.6), (6.7, 7.8)].into_iter() {
            nhmmv.insert(k, v);
            nhmvt.insert(k, v);
        }

        assert_equal(nhmmv.iter(), nhmvt.iter().map(|x| (&x.0, &x.1)));
    }

    #[test]
    fn test_iter() {
        let hm: HashMap<usize, usize> = HashMap::from_iter(vec![(0, 1), (2, 3), (4, 5), (6, 7)]);
        let mut nhmmv = NoHashMapMultiVec::new();
        let mut nhmvt = NoHashMapVecTuple::new();

        for (k, v) in vec![(0, 1), (2, 3), (4, 5), (6, 7)].into_iter() {
            nhmmv.insert(k, v);
            nhmvt.insert(k, v);
        }
        assert_equal(hm.iter().sorted(), nhmmv.iter().sorted());
        assert_equal(hm.into_iter().sorted(), nhmvt.iter().map(|x| *x).sorted());
    }

    #[test]
    fn test_iter_mut() {
        let mut nhmmv = NoHashMapMultiVec::new();
        let mut nhmvt = NoHashMapVecTuple::new();

        for (k, v) in vec![(0, 1), (2, 3), (4, 5), (6, 7)].into_iter() {
            nhmmv.insert(k, v);
            nhmvt.insert(k, v);
        }

        nhmmv.iter_mut().for_each(|(k, v)| {
            *k += 10;
            *v += 10
        });
        nhmvt.iter_mut().for_each(|(k, v)| {
            *k += 10;
            *v += 10
        });

        assert_equal(nhmmv.iter(), nhmvt.iter().map(|x| (&x.0, &x.1)));
        assert!(nhmmv.values().iter().min().unwrap() > &10);
    }

    #[test]
    fn test_len() {
        let mut nhmmv = NoHashMapMultiVec::new();
        let mut nhmvt = NoHashMapVecTuple::new();

        for (k, v) in vec![(0, 1), (2, 3), (4, 5), (6, 7)].into_iter() {
            nhmmv.insert(k, v);
            nhmvt.insert(k, v);
        }

        assert_eq!(nhmmv.len(), 4);
        assert_eq!(nhmvt.len(), 4);
    }

    #[test]
    fn test_values() {
        let mut nhmmv = NoHashMapMultiVec::new();
        let mut nhmvt = NoHashMapVecTuple::new();
        let values = [1, 3, 5, 7];

        for (k, v) in vec![(0, 1), (2, 3), (4, 5), (6, 7)].into_iter() {
            nhmmv.insert(k, v);
            nhmvt.insert(k, v);
        }

        assert_equal(nhmmv.values(), values.iter());
        assert_equal(nhmvt.values(), values.iter());
    }

    #[test]
    fn test_keys() {
        let mut nhmmv = NoHashMapMultiVec::new();
        let mut nhmvt = NoHashMapVecTuple::new();
        let keys = [0, 2, 4, 6];

        for (k, v) in vec![(0, 1), (2, 3), (4, 5), (6, 7)].into_iter() {
            nhmmv.insert(k, v);
            nhmvt.insert(k, v);
        }

        assert_equal(nhmmv.keys(), keys.iter());
        assert_equal(nhmvt.keys(), keys.iter());
    }

    #[test]
    fn test_values_mut() {
        let mut nhmmv = NoHashMapMultiVec::new();
        let mut nhmvt = NoHashMapVecTuple::new();

        for (k, v) in vec![(0, 1), (2, 3), (4, 5), (6, 7)].into_iter() {
            nhmmv.insert(k, v);
            nhmvt.insert(k, v);
        }

        nhmmv.values_mut().for_each(|v| *v += 10);
        nhmvt.values_mut().for_each(|v| *v += 10);

        assert_equal(nhmmv.values(), nhmvt.values());
    }

    #[test]
    fn test_keys_mut() {
        let mut nhmmv = NoHashMapMultiVec::new();
        let mut nhmvt = NoHashMapVecTuple::new();

        for (k, v) in vec![(0, 1), (2, 3), (4, 5), (6, 7)].into_iter() {
            nhmmv.insert(k, v);
            nhmvt.insert(k, v);
        }

        nhmmv.keys_mut().for_each(|k| *k += 10);
        nhmvt.keys_mut().for_each(|k| *k += 10);

        assert_equal(nhmmv.keys(), nhmvt.keys());
    }

    #[test]
    fn test_into_values() {
        let mut nhmmv = NoHashMapMultiVec::new();
        let mut nhmvt = NoHashMapVecTuple::new();
        let values = [1, 3, 5, 7];

        for (k, v) in vec![(0, 1), (2, 3), (4, 5), (6, 7)].into_iter() {
            nhmmv.insert(k, v);
            nhmvt.insert(k, v);
        }

        assert_equal(nhmmv.into_values(), values.into_iter());
        assert_equal(nhmvt.into_values(), values.into_iter());
    }

    #[test]
    fn test_into_keys() {
        let mut nhmmv = NoHashMapMultiVec::new();
        let mut nhmvt = NoHashMapVecTuple::new();
        let keys = [0, 2, 4, 6];

        for (k, v) in vec![(0, 1), (2, 3), (4, 5), (6, 7)].into_iter() {
            nhmmv.insert(k, v);
            nhmvt.insert(k, v);
        }

        assert_equal(nhmmv.into_keys(), keys.into_iter());
        assert_equal(nhmvt.into_keys(), keys.into_iter());
    }

    #[test]
    fn test_contains_key() {
        let mut nhmmv = NoHashMapMultiVec::new();
        let mut nhmvt = NoHashMapVecTuple::new();
        let keys = [0, 2, 4, 6];

        for (k, v) in vec![(0, 1), (2, 3), (4, 5), (6, 7)].into_iter() {
            nhmmv.insert(k, v);
            nhmvt.insert(k, v);
        }

        for k in keys.iter() {
            assert!(nhmmv.contains_key(k));
            assert!(nhmvt.contains_key(k));
        }
    }

    #[test]
    fn test_is_empty() {
        let mut nhmmv = NoHashMapMultiVec::new();
        let mut nhmvt = NoHashMapVecTuple::new();

        assert!(nhmmv.is_empty());
        assert!(nhmvt.is_empty());

        for (k, v) in vec![(0, 1), (2, 3), (4, 5), (6, 7)].into_iter() {
            nhmmv.insert(k, v);
            nhmvt.insert(k, v);
        }

        assert!(!nhmmv.is_empty());
        assert!(!nhmvt.is_empty());
    }

    #[test]
    fn test_capacity() {
        let mut nhmmv = NoHashMapMultiVec::new();
        let mut nhmvt = NoHashMapVecTuple::new();

        assert_eq!(nhmmv.capacity(), 0);
        assert_eq!(nhmvt.capacity(), 0);

        for (k, v) in vec![(0, 1), (2, 3), (4, 5), (6, 7)].into_iter() {
            nhmmv.insert(k, v);
            nhmvt.insert(k, v);
        }

        assert_eq!(nhmmv.capacity(), nhmvt.capacity());
    }

    #[test]
    fn test_clear() {
        let mut nhmmv = NoHashMapMultiVec::new();
        let mut nhmvt = NoHashMapVecTuple::new();

        for (k, v) in vec![(0, 1), (2, 3), (4, 5), (6, 7)].into_iter() {
            nhmmv.insert(k, v);
            nhmvt.insert(k, v);
        }

        nhmmv.clear();
        nhmvt.clear();

        assert!(nhmmv.is_empty());
        assert!(nhmvt.is_empty());
    }

    #[test]
    fn test_drain() {
        let hm: HashMap<usize, usize> = HashMap::from_iter(vec![(0, 1), (2, 3), (4, 5), (6, 7)]);
        let mut nhmmv: NoHashMapMultiVec<usize, usize> = NoHashMapMultiVec::new();
        let mut nhmvt = NoHashMapVecTuple::new();

        for (k, v) in vec![(0, 1), (2, 3), (4, 5), (6, 7)].into_iter() {
            nhmmv.insert(k, v);
            nhmvt.insert(k, v);
        }
        assert_equal(hm.clone().into_iter().sorted(), nhmmv.drain().sorted());
        assert_equal(hm.into_iter().sorted(), nhmvt.drain().sorted());

        assert!(nhmmv.is_empty());
        assert!(nhmvt.is_empty());
    }
}
