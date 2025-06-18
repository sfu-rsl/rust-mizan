//! Projected atomic reference-counted pointers.
//!
//! Available pointer types:
//! - [`Parc`]
//! - [`Weak`]
//!
//! # Example
//! ```
//! # use std::sync::Arc;
//! use pared::sync::{Parc, Weak};
//! fn accepts_parc(parc: Parc<u8>) {}
//!
//! // Parc can be created by projecting references from an Arc
//! let from_tuple = Parc::from_arc(&Arc::new((16usize, 8u8)), |tuple| &tuple.1);
//! // Or by using any T: Into<Arc<_>>
//! let from_u8: Parc<u8> = Parc::new(8u8);
//!
//! std::thread::spawn(move || {
//!     // Functions accept any Parc<T>, regardless of which Arc<U> it was created from
//!     if (true) {
//!         accepts_parc(from_tuple);
//!     } else {
//!         accepts_parc(from_u8);
//!     }
//! });
//! ```
//!
//! Parc can only be created from `Arc`s (or other `Parc`s) for `T: Send + Sync`.
//!
//! ```compile_fail,E0277
//! # use std::sync::Arc;
//! use pared::sync::Parc;
//! // This Arc is !Send + !Sync
//! let arc: Arc<*const i32> = Arc::new(&1 as *const i32);
//! // Error: Parc can only be backed by an Arc<T>: Send + Sync
//! let parc: Parc<*const i32> = arc.into();
//! ```
//! ```compile_fail,E0277
//! # use std::sync::Arc;
//! use pared::sync::Parc;
//! let parc = Parc::new(1);
//! // This Parc is !Send and !Sync
//! let no_send = parc.project(|x| &(&1u8 as *const u8));
//! // Error
//! let denied = no_send.project(|x| x);
//! ```
//!
//! # Soundness
//! None of the following should compile:
//!
//! ```compile_fail,E0597
//! use pared::sync::Parc;
//! use std::sync::Arc;
//!
//! let x: Arc<()> = Arc::new(());
//! let z: Parc<str>;
//! {
//!     let s = "Hello World!".to_string();
//!     let s_ref: &str = &s;
//!     let y: Parc<&str> = Parc::from_arc(&x, |_| &s_ref);
//!     z = y.project(|s: &&str| *s);
//!     // s deallocated here
//! }
//! println!("{}", &*z); // printing garbage, accessing `s` after it’s freed
//! ```
//!
//! ```compile_fail,E0597
//! use pared::sync::Parc;
//!
//! let x: Parc<()> = Parc::new(());
//! let z: Parc<str>;
//! {
//!     let s = "Hello World!".to_string();
//!     let s_ref: &str = &s;
//!     let y: Parc<&str> = x.project(|_| &s_ref);
//!     z = y.project(|s: &&str| *s);
//!     // s deallocated here
//! }
//! println!("{}", &*z); // printing garbage, accessing `s` after it’s freed
//! ```
//!
//! ```compile_fail,E0597
//! use pared::sync::Parc;
//!
//! let x: Parc<()> = Parc::new(());
//! let z: Parc<str>;
//! {
//!     let s = "Hello World!".to_string();
//!     z = x.project(|_| &s as &str);
//!     // s deallocated here
//! }
//! println!("{}", &*z); // printing garbage, accessing `s` after it’s freed
//! ```

pub mod erased_arc;
