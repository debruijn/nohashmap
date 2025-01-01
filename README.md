# NoHashMap
Use HashMap syntax in Rust for non-hashable types. 

## Motivation
Imagine working on a Rust project that heavily involves a `HashMap<T, U>` with `T=isize` but at a certain point you are 
curious for the output when using a `HashMap<f32, U>`. The only problem is: floats in Rust are not hashable, so you 
can't just switch it out and have everything work. Instead, add NoHashMap to your project and simply include the line 
`type HashMap<T, U> = nohashmap::NoHashMap<T, U>;`. Then you can adjust your input to be `f32` or whatever else
that is not hashable.

I had this literal problem while participating in `everybody.codes` using Rust and being curious for some property of
the problem which would involve floats (the actual problem statement only used integers).

### Alternatives
- use a hashable float like `Decimal` or my other quickly-put-together package `IntFloat`
  - if you want to properly support floats, then `Decimal` is probably the way to go, but is a bit more work if you just
    want to test it out
- rewrite the code to not be dependent on a HashMap
  - this is often too much work for a quick test, so then you are just not going to do it

## Implementation and performance implications
There are two implementations available that both can work as a HashMap replacement:
- NoHashMapMultiVec, that uses two Vec<>s to represent the keys and values of the fake map
- NoHashMapVecTuple, that uses a Vec<> containing (key, value) tuples

There is a NoHashMap utility type equals NoHashMapMultiVec since out of the 2, that option has the fewest corner cases 
where the syntax breaks compared to a default HashMap (e.g. HashMap's `value_mut()` returns an `IterMut<V>` and 
NoHashMapMultiVec does so as well, but NoHashMapVecTuple returns an `IntoIter<&mut V>`; practically still an iterable
with mutable values in it, but might require some changes to function signatures and variable definitions). 

Depending on the details of how it will be used, either one can be faster than the other. Just to be sure: both are 
almost always slower than an actual proper HashMap (except for very small maps perhaps) so it is not advised to go into 
production with an implementation based around `NoHashMap`.

## Bugs or ideas
Have you found any bug or other ideas? Please let me know by opening an issue.

## Licensing
As is often the case for Rust packages, this is dual licensed (APACHE & MIT). See the corresponding license files for
more details.
