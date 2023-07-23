/// contains the public and internal contract types derived from the ally [sol!] macro
pub mod contracts;

/// the core types of the sdk
/// implements the verification and quote logic
/// also you can find some helper function on the [Order] type as well
pub mod order;

/// this is where [OrderCache] is implemented
///
/// there is also some useful helper functions for working with tokio
pub mod utils;
