// devela_depend
//
//!
//

// warnings
#![warn(clippy::all)]
// environment
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "safe", forbid(unsafe_code))]
#![cfg_attr(feature = "nightly", feature(doc_cfg))]
#[cfg(feature = "alloc")]
extern crate alloc;

/// <span class="stab portability" title="re-exported `atomic` crate">`atomic`</span>
#[doc = "A generic atomic wrapper type.\n\n"]
#[doc = "*Re-exported [`atomic`](https://docs.rs/atomic)* crate.\n\n---"]
#[doc(inline)]
#[cfg(feature = "atomic")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "atomic")))]
pub use ::atomic;

/// <span class="stab portability" title="re-exported `az` crate">`az`</span>
#[doc = "Casts and checked casts.\n\n"]
#[doc = "*Re-exported [`az`](https://docs.rs/az)* crate.\n\n---"]
#[doc(inline)]
#[cfg(feature = "az")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "az")))]
pub use ::az;

/// <span class="stab portability" title="re-exported `bytemuck` crate">`bytemuck`</span>
#[doc = "Small utilities for casting between plain data types.\n\n"]
#[doc = "*Re-exported [`bytemuck`](https://docs.rs/bytemuck)* crate.\n\n---"]
#[doc(inline)]
#[cfg(feature = "bytemuck")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "bytemuck")))]
pub use ::bytemuck;

/// <span class="stab portability" title="re-exported `const-str` crate">`const-str`</span>
#[doc = "Compile-time string operations.\n\n"]
#[doc = "*Re-exported [`const-str`](https://docs.rs/const-str)* crate.\n\n---"]
#[doc(inline)]
#[cfg(feature = "const-str")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "const-str")))]
pub use ::const_str;

/// <span class="stab portability" title="re-exported `devela_macros` crate">`devela_macros`</span>
#[doc = "Procedural macros for `devela`.\n\n"]
#[doc = "*Re-exported [`devela_macros`](https://docs.rs/devela_macros)* crate.\n\n---"]
#[doc(inline)]
#[cfg(feature = "devela_macros")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "devela_macros")))]
pub use ::devela_macros;

/// <span class="stab portability" title="re-exported `hashbrown`
/// crate">`hashbrown`</span>
#[doc = "A drop-in replacement for Rust’s standard `HashMap` and `HashSet`.\n\n"]
#[doc = "*Re-exported [`hashbrown`](https://docs.rs/hashbrown)* crate.\n\n---"]
#[doc(inline)]
#[cfg(feature = "hashbrown")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "hashbrown")))]
pub use ::hashbrown;

/// <span class="stab portability" title="re-exported `portable-atomic`
/// crate">`portable-atomic`</span>
#[doc = "Portable atomic types including 128-bit atomics, floats, etc.\n\n"]
#[doc = "*Re-exported [`portable-atomic`](https://docs.rs/portable-atomic)* crate.\n\n---"]
#[doc(inline)]
#[cfg(feature = "portable-atomic")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "portable-atomic")))]
pub use ::portable_atomic;

/// <span class="stab portability" title="re-exported `unicode-segmentation`
/// crate">`unicode-segmentation`</span>
#[doc = "Split strings on Grapheme Cluster, Word or Sentence boundaries.\n\n"]
#[doc = "*Re-exported [`unicode-segmentation`](https://docs.rs/unicode-segmentation)*
crate.\n\n---"]
#[doc(inline)]
#[cfg(feature = "unicode-segmentation")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "unicode-segmentation")))]
pub use ::unicode_segmentation;
