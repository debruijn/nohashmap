use itertools::{izip, Itertools};
use std::cmp::min;
use std::collections::TryReserveError;
use std::fmt;
use std::fmt::Debug;
use std::iter::Zip;
use std::slice::{Iter, IterMut};
use std::vec::{Drain, IntoIter};

#[derive(Clone, Default)]
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

    // pub fn values_mut<'a>(&mut self) -> Map<IterMut<'_, (K, V)>, fn(&'a mut (K, V)) -> V> {
    //     self.vec.iter_mut().map(|x| x.1)
    // }

    pub fn into_values(self) -> IntoIter<V> {
        self.vec.into_iter().map(|x| x.1).collect_vec().into_iter()
    }

    pub fn keys(&self) -> Vec<&K> {
        self.vec.iter().map(|x| &x.0).collect_vec()
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

#[test]
fn try_stuff_out() {
    use std::time::Instant;
    type NoHashMap<K, V> = NoHashMapMultiVec<K, V>;

    let mut nhm = NoHashMapMultiVec::new();
    println!("{:?}", nhm);
    nhm.insert(0.1, 0.1);
    println!("{:?}", nhm);

    println!("{:?}, {:?}", nhm.get(&0.1), nhm.get(&0.2));
    println!("{:?}, {:?}", nhm.contains_key(&0.1), nhm.contains_key(&0.2));

    let mut nhm = NoHashMapMultiVec::new();
    nhm.insert(0.1, "blue");
    nhm.insert(1.2, "green");
    nhm.insert(0.4, "red");
    nhm.insert(0.7, "yellow");
    nhm.insert(-2.3, "orange");
    println!("{}, {:?}", nhm.len(), nhm);
    nhm.remove(&0.4);
    println!("{}, {:?}", nhm.len(), nhm.iter().collect_vec().iter());
    nhm.swap_remove(&0.1);
    println!("{}, {:?}", nhm.len(), nhm);

    let mut nhm = NoHashMapVecTuple::new();
    println!("{:?}", nhm);
    nhm.insert(0.1, 0.1);
    println!("{:?}", nhm);

    println!("{:?}, {:?}", nhm.get(&0.1), nhm.get(&0.2));
    println!("{:?}, {:?}", nhm.contains_key(&0.1), nhm.contains_key(&0.2));

    let mut nhm = NoHashMapVecTuple::new();
    nhm.insert(0.1, "blue");
    nhm.insert(1.2, "green");
    nhm.insert(0.4, "red");
    nhm.insert(0.7, "yellow");
    nhm.insert(-2.3, "orange");
    println!("{}, {:?}", nhm.len(), nhm);
    nhm.remove(&0.4);
    println!("{}, {:?}", nhm.len(), nhm.iter());
    nhm.swap_remove(&0.1);
    println!("{}, {:?}", nhm.len(), nhm);

    let mut nhm = NoHashMap::new();
    nhm.insert(0.1, "blue");
    nhm.insert(1.2, "green");
    nhm.insert(0.4, "red");
    nhm.insert(0.7, "yellow");
    nhm.insert(-2.3, "orange");
    println!("{}, {:?}", nhm.len(), nhm);
    nhm.remove(&0.4);
    println!("{}, {:?}", nhm.len(), nhm.iter().collect_vec().iter());
    nhm.swap_remove(&0.1);
    println!("{}, {:?}", nhm.len(), nhm);

    let r = 10000isize;

    let before = Instant::now();
    let mut nhm1 = NoHashMapMultiVec::new();
    for i in 0..r {
        nhm1.insert(i, i);
    }
    let res = nhm1.iter().map(|x| x.1).sum::<isize>();
    let after = Instant::now();
    println!("{:?} in {:?}", res, after - before);
    let before = Instant::now();
    let mut nhm2 = NoHashMapVecTuple::new();
    for i in 0..r {
        nhm2.insert(i, i);
    }
    let res = nhm2.iter().map(|x| x.1).sum::<isize>();
    let after = Instant::now();
    println!("{:?} in {:?}", res, after - before);

    let before = Instant::now();
    let mut nhm1 = NoHashMapMultiVec::new();
    for i in 0..r {
        nhm1.insert(i, i);
    }
    let res = nhm1.values().into_iter().sum::<isize>();
    let after = Instant::now();
    println!("{:?} in {:?}", res, after - before);
    let before = Instant::now();
    let mut nhm2 = NoHashMapVecTuple::new();
    for i in 0..r {
        nhm2.insert(i, i);
    }
    let res = nhm2.into_values().sum::<isize>();
    let after = Instant::now();
    println!("{:?} in {:?}", res, after - before);
}
