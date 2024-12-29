pub mod nohashmap;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut nhm = nohashmap::NoHashMapMultiVec::new();
        println!("{:?}", nhm);
        nhm.insert(0.1, 0.1);
        println!("{:?}", nhm);
    }
}
