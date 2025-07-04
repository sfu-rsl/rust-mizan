//! Projected reference-counted pointers.
//!
//! Available pointer types:
//! - [`Prc`]
//! - [`Weak`]
//!
//! # Example
//! ```
//! # use std::rc::Rc;
//! use pared::prc::{Prc, Weak};
//! fn accepts_prc(prc: Prc<u8>) {}
//!
//! // Prc can be created by projecting references from an Rc
//! let from_tuple = Prc::from_rc(&Rc::new((16usize, 8u8)), |tuple| &tuple.1);
//! // Or by using any T: Into<Rc<_>>
//! let from_u8: Prc<u8> = Prc::new(8u8);
//!
//! // Functions accept any Prc<T>, regardless of which Rc<U> it was created from
//! if (true) {
//!     accepts_prc(from_tuple);
//! } else {
//!     accepts_prc(from_u8);
//! }
//! ```
//! //! # Soundness
//! None of the following should compile:
//!
//! ```compile_fail,E0597
//! use pared::prc::Prc;
//! use std::rc::Rc;
//!
//! let x: Rc<()> = Rc::new(());
//! let z: Prc<str>;
//! {
//!     let s = "Hello World!".to_string();
//!     let s_ref: &str = &s;
//!     let y: Prc<&str> = Prc::from_rc(&x, |_| &s_ref);
//!     z = y.project(|s: &&str| *s);
//!     // s deallocated here
//! }
//! println!("{}", &*z); // printing garbage, accessing `s` after it’s freed
//! ```
//!
//! ```compile_fail,E0597
//! use pared::prc::Prc;
//!
//! let x: Prc<()> = Prc::new(());
//! let z: Prc<str>;
//! {
//!     let s = "Hello World!".to_string();
//!     let s_ref: &str = &s;
//!     let y: Prc<&str> = x.project(|_| &s_ref);
//!     z = y.project(|s: &&str| *s);
//!     // s deallocated here
//! }
//! println!("{}", &*z); // printing garbage, accessing `s` after it’s freed
//! ```
//!
//! ```compile_fail,E0597
//! use pared::prc::Prc;
//! use std::sync::Arc;
//!
//! let x: Prc<()> = Prc::new(());
//! let z: Prc<str>;
//! {
//!     let s = "Hello World!".to_string();
//!     z = x.project(|_| &s as &str);
//!     // s deallocated here
//! }
//! println!("{}", &*z); // printing garbage, accessing `s` after it’s freed
//! ```

pub mod erased_rc;