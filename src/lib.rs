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

/* optional crates */

#[doc(inline)]
#[doc = d!(tag "atomic")]
#[doc = "A generic atomic wrapper type."]
#[doc = d!(link "atomic")]
#[cfg(feature = "atomic")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "atomic")))]
pub use ::atomic;

#[doc(inline)]
#[doc = d!(tag "az")]
#[doc = "Casts and checked casts."]
#[doc = d!(link "az")]
#[cfg(feature = "az")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "az")))]
pub use ::az;

#[doc(inline)]
#[doc = d!(tag "bytemuck")]
#[doc = "Small utilities for casting between plain data types."]
#[doc = d!(link "bytemuck")]
#[cfg(feature = "bytemuck")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "bytemuck")))]
pub use ::bytemuck;

#[doc(inline)]
#[doc = d!(tag "const-str")]
#[doc = "Compile-time string operations."]
#[doc = d!(link "const-str")]
#[cfg(feature = "const-str")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "const-str")))]
pub use ::const_str;

#[doc(inline)]
#[doc = d!(tag "devela_macros")]
#[doc = "Procedural macros for `devela`."]
#[doc = d!(link "devela_macros")]
#[cfg(feature = "devela_macros")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "devela_macros")))]
pub use ::devela_macros;

#[doc(inline)]
#[doc = d!(tag "hashbrown")]
#[doc = "A drop-in replacement for Rust’s standard `HashMap` and `HashSet`."]
#[doc = d!(link "hashbrown")]
#[cfg(feature = "hashbrown")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "hashbrown")))]
pub use ::hashbrown;

#[doc(inline)]
#[doc = d!(tag "portable-atomic")]
#[doc = "Portable atomic types including 128-bit atomics, floats, etc."]
#[doc = d!(link "portable-atomic")]
#[cfg(feature = "portable-atomic")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "portable-atomic")))]
pub use ::portable_atomic;

#[doc(inline)]
#[doc = d!(tag "unicode-segmentation")]
#[doc = "Split strings on Grapheme Cluster, Word or Sentence boundaries."]
#[doc = d!(link "unicode-segmentation")]
#[cfg(feature = "unicode-segmentation")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "unicode-segmentation")))]
pub use ::unicode_segmentation;

/* helper macro */

macro_rules! d {
    (tag $crate_name:literal) => {
        concat!(
            "<span class='stab portability' title='re-exported `",
            $crate_name,
            "` crate'>`",
            $crate_name,
            "`</span>"
        )
    };
    (link $crate_name:literal) => {
        concat!(
            "\n\n*Re-exported [`",
            $crate_name,
            "`](https://docs.rs/",
            $crate_name,
            ")* crate.\n\n---"
        )
    };
}
use d;
