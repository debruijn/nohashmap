//! # Nohashmap
//!
//! `nohashmap` allows you to use HashMap syntax in Rust for non-hashable types.
//!

pub mod nohashmap;
pub use nohashmap::{NoHashMapMultiVec, NoHashMapVecTuple};
pub type NoHashMap<K, V> = NoHashMapMultiVec<K, V>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut nhm = NoHashMapMultiVec::new();
        println!("{:?}", nhm);
        nhm.insert(0.1, 0.1);
        println!("{:?}", nhm);
    }
}
