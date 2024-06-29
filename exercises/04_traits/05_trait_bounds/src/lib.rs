// TODO: Add the necessary trait bounds to `min` so that it compiles successfully.
//   Refer to the documentation of the `std::cmp` module for more information on the traits you might need.
//
// Note: there are different trait bounds that'll make the compiler happy, but they come with
// different _semantics_. We'll cover those differences later in the course when we talk about ordered
// collections (e.g. BTreeMap).

/// Return the minimum of two values.
pub fn min<T: PartialOrd>(left: T, right: T) -> T {
    if left <= right {
        left
    } else {
        right
    }
}

/*
   Compiling trait_bounds v0.1.0 (/home/namn/100-exercises-to-learn-rust/exercises/04_traits/05_trait_bounds)
error[E0369]: binary operation `<=` cannot be applied to type `T`
  --> exercises/04_traits/05_trait_bounds/src/lib.rs:10:13
   |
10 |     if left <= right {
   |        ---- ^^ ----- T
   |        |
   |        T
   |
help: consider restricting type parameter `T`
   |
9  | pub fn min<T: std::cmp::PartialOrd>(left: T, right: T) -> T {
   |             ++++++++++++++++++++++

For more information about this error, try `rustc --explain E0369`.
error: could not compile `trait_bounds` (lib) due to 1 previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `trait_bounds` (lib test) due to 1 previous error
*/
