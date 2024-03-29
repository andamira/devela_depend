// devela_depend
//
//!
//

// warnings
#![warn(missing_docs, clippy::all)]
// nightly, safety, environment
#![cfg_attr(feature = "nightly", feature(doc_cfg))]
#![cfg_attr(feature = "safe", forbid(unsafe_code))]
#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(feature = "alloc")]
extern crate alloc;

/* optional crates */

reexport! { "atomic", atomic,
    "A generic atomic wrapper type."
}
reexport! { "bytemuck", bytemuck,
    "Small utilities for casting between plain data types."
}
reexport! { "const-str", const_str,
    "Compile-time string operations."
}
reexport! { "crossterm", crossterm,
    "A crossplatform terminal library for manipulating terminals."
}
reexport! { "hashbrown", hashbrown,
    also: "alloc",
    "A drop-in replacement for Rust’s standard `HashMap` and `HashSet`."
}
reexport! { "libm", libm,
    "A port of [`MUSL`](https://musl.libc.org/)'s libm to Rust."
}
reexport! { "memchr", memchr,
    "Optimized routines for string search primitives."
}
reexport! { "miniquad", miniquad,
    "Cross-platform window context and rendering library."
}
reexport! { "portable-atomic", portable_atomic,
    "Portable atomic types including 128-bit atomics, floats, etc."
}
reexport! { "rand_core", rand_core,
    "Random number generation traits."
}
reexport! { "unicode-segmentation", unicode_segmentation,
    "Split strings on Grapheme Clusters, Words or Sentences."
}
reexport! { "unicode-width", unicode_width,
    "Determine displayed width of `char` and `str` types."
}
reexport! { "wide", wide,
    "SIMD-compatible data types."
}

// Macro helper for documentation of re-exported items.
#[rustfmt::skip]
#[allow(unused)]
macro_rules! reexport {
    // depends just on the optional dependency
    ( $dep_name:literal, $dep_module:ident,
      $dep_description:literal
      $(,)?
    ) => {
        #[doc(inline)]
        #[doc = concat!("<span class='stab portability' title='re-exported `",
            $dep_name, "` crate'>`", $dep_name, "`</span>")]
        #[doc = $dep_description]
        #[doc = concat!("\n\n*Re-exported [`", $dep_name,
            "`](https://docs.rs/", $dep_name, ")* crate.\n\n---")]

        #[cfg(feature = $dep_name)]
        #[cfg_attr(feature = "nightly", doc(cfg(feature = $dep_name)))]
        pub use ::$dep_module;
    };

    // depends on an optional dependency and also on another feature, e.g. "alloc"
    ( $dep_name:literal, $dep_module:ident,
      also: $another_feature:literal,
      $dep_description:literal
      $(,)?
    ) => {
        #[doc(inline)]
        #[doc = concat!("<span class='stab portability' title='re-exported `",
            $dep_name, "` crate'>`", $dep_name, "`</span>")]
        #[doc = $dep_description]
        #[doc = concat!("\n\n*Re-exported [`", $dep_name,
            "`](https://docs.rs/", $dep_name, ")* crate.\n\n---")]

        #[cfg_attr(feature = "nightly",
            doc(cfg(all(feature = $dep_name, feature = $another_feature))))]
        #[cfg(all(feature = $dep_name, feature = $another_feature))]
        pub use ::$dep_module;
    };
}
#[allow(unused)]
use reexport;
