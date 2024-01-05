#[cfg(not(feature = "std"))]
pub use alloc::boxed::Box;
#[cfg(not(feature = "std"))]
pub use alloc::collections::{BinaryHeap, VecDeque};
#[cfg(not(feature = "std"))]
pub use alloc::vec;
#[cfg(not(feature = "std"))]
pub use alloc::vec::Vec;
#[cfg(not(feature = "std"))]
pub use hashbrown::{hash_map, HashMap, HashSet};

#[cfg(feature = "std")]
pub use std::boxed::Box;
#[cfg(feature = "std")]
pub use std::collections::{hash_map, BinaryHeap, HashMap, HashSet, VecDeque};
#[cfg(feature = "std")]
pub use std::vec;
#[cfg(feature = "std")]
pub use std::vec::Vec;
