//! # Overview
//!
//! `once_cell` provides two
//! new cell-like types,
//! `unsync::OnceCell` and
//! `sync::OnceCell`.
//! `OnceCell` might store
//! arbitrary non-`Copy`
//! types, can be assigned to
//! at most once and provide
//! direct access
//! to the stored contents. In
//! a nutshell, API looks
//! *roughly* like this:
//!
//! ```rust,ignore
//! impl<T> OnceCell<T> {
//! fn new() -> OnceCell<T> { ... }
//! fn set(&self, value: T) -> Result<(), T> { ... }
//! fn get(&self) -> Option<&T> { ... }
//! }
//! ```
//!
//! Note that, like with
//! `RefCell` and `Mutex`, the
//! `set` method requires only
//! a shared reference.
//! Because of the single
//! assignment restriction
//! `get` can return an `&T`
//! instead of `Ref<T>`
//! or `MutexGuard<T>`.
//!
//! # Patterns
//!
//! `OnceCell` might be useful
//! for a variety of patterns.
//!
//! ## Safe Initialization of global data
//!
//! ```rust
//! use std::{env, io};
//!
//! use once_cell::sync::OnceCell;
//!
//! #[derive(Debug)]
//! pub struct Logger {
//! ...
//! }
//! static INSTANCE: OnceCell<Logger> = OnceCell::new();
//!
//! impl Logger {
//! pub fn global() -> &'static Logger {
//! INSTANCE.get().expect("logger is not initialized")
//! }
//!
//! fn from_cli(args: env::Args) -> Result<Logger, std::io::Error> {
//! ...
//! #      Ok(Logger {})
//! }
//! }
//!
//! fn main() {
//! let logger = Logger::from_cli(env::args()).unwrap();
//! INSTANCE.set(logger).unwrap();
//! use `Logger::global()` from now on
//! }
//! ```
//!
//! ## Lazy initialized global data
//!
//! This is essentially
//! `lazy_static!` macro, but
//! without a macro.
//!
//! ```rust
//! use std::{sync::Mutex, collections::HashMap};
//!
//! use once_cell::sync::OnceCell;
//!
//! fn global_data() -> &'static Mutex<HashMap<i32, String>> {
//! static INSTANCE: OnceCell<Mutex<HashMap<i32, String>>> = OnceCell::new();
//! INSTANCE.get_or_init(|| {
//! let mut m = HashMap::new();
//! m.insert(13, "Spica".to_string());
//! m.insert(74, "Hoyten".to_string());
//! Mutex::new(m)
//! })
//! }
//! ```
//!
//! There are also
//! `sync::Lazy` and
//! `unsync::Lazy` convenience
//! types to streamline this
//! pattern:
//!
//! ```rust
//! use std::{sync::Mutex, collections::HashMap};
//! use once_cell::sync::Lazy;
//!
//! static GLOBAL_DATA: Lazy<Mutex<HashMap<i32, String>>> = Lazy::new(|| {
//! let mut m = HashMap::new();
//! m.insert(13, "Spica".to_string());
//! m.insert(74, "Hoyten".to_string());
//! Mutex::new(m)
//! });
//!
//! fn main() {
//! println!("{:?}", GLOBAL_DATA.lock().unwrap());
//! }
//! ```
//!
//! ## General purpose lazy evaluation
//!
//! Unlike `lazy_static!`,
//! `Lazy` works with local
//! variables.
//!
//! ```rust
//! use once_cell::unsync::Lazy;
//!
//! fn main() {
//! let ctx = vec![1, 2, 3];
//! let thunk = Lazy::new(|| {
//! ctx.iter().sum::<i32>()
//! });
//! assert_eq!(*thunk, 6);
//! }
//! ```
//!
//! If you need a lazy field
//! in a struct, you probably
//! should use `OnceCell`
//! directly, because that
//! will allow you to access
//! `self` during
//! initialization.
//!
//! ```rust
//! use std::{fs, path::PathBuf};
//!
//! use once_cell::unsync::OnceCell;
//!
//! struct Ctx {
//! config_path: PathBuf,
//! config: OnceCell<String>,
//! }
//!
//! impl Ctx {
//! pub fn get_config(&self) -> Result<&str, std::io::Error> {
//! let cfg = self.config.get_or_try_init(|| {
//! fs::read_to_string(&self.config_path)
//! })?;
//! Ok(cfg.as_str())
//! }
//! }
//! ```
//!
//! ## Building block
//!
//! Naturally, it is  possible
//! to build other
//! abstractions on top of
//! `OnceCell`. For example,
//! this is a `regex!` macro
//! which takes a string
//! literal and returns an
//! expression* that evaluates
//! to a `&'static Regex`:
//!
//! ```
//! macro_rules! regex {
//! ($re:literal $(,)?) => {{
//! static RE: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
//! RE.get_or_init(|| regex::Regex::new($re).unwrap())
//! }};
//! }
//! ```
//!
//! This macro can be useful
//! to avoid "compile regex on
//! every loop iteration"
//! problem.
//!
//! # Comparison with std
//!
//! |`!Sync` types         | Access Mode            | Drawbacks                                     |
//! |----------------------|------------------------|-----------------------------------------------|
//! |`Cell<T>`             | `T`                    | requires `T: Copy` for `get`                  |
//! |`RefCel<T>`           | `RefMut<T>` / `Ref<T>` | may panic at runtime                          |
//! |`unsync::OnceCell<T>` | `&T`                   | assignable only once                          |
//!
//! |`Sync` types          | Access Mode            | Drawbacks                                     |
//! |----------------------|------------------------|-----------------------------------------------|
//! |`AtomicT`             | `T`                    | works only with certain `Copy` types          |
//! |`Mutex<T>`            | `MutexGuard<T>`        | may deadlock at runtime, may block the thread |
//! |`sync::OnceCell<T>`   | `&T`                   | assignable only once, may block the thread    |
//!
//! Technically, calling
//! `get_or_init` will also
//! cause a panic or a
//! deadlock if it recursively
//! calls itself. However,
//! because the assignment can
//! happen only once, such
//! cases should be more rare
//! than equivalents with
//! `RefCell` and `Mutex`.
//!
//! # Minimum Supported `rustc` Version
//!
//! This crate's minimum
//! supported `rustc` version
//! is `1.31.1`.
//!
//! If only `std` feature is
//! enabled, MSRV will be
//! updated conservatively.
//! When using other features,
//! like `parking_lot`, MSRV
//! might be updated more
//! frequently, up to the
//! latest stable.
//! In both cases, increasing
//! MSRV is *not* considered a
//! semver-breaking change.
//!
//! # Implementation details
//!
//! Implementation is based on [`lazy_static`](https://github.com/rust-lang-nursery/lazy-static.rs/)
//! and [`lazy_cell`](https://github.com/indiv0/lazycell/) crates and `std::sync::Once`. In some sense,
//! `once_cell` just
//! streamlines and unifies
//! those APIs.
//!
//! To implement a sync flavor
//! of `OnceCell`, this crates
//! uses either a custom
//! re-implementation of
//! `std::sync::Once` or
//! `parking_lot::Mutex`. This
//! is controlled by the
//! `parking_lot` feature,
//! which is enabled by
//! default. Performance is
//! the same for both cases,
//! but `parking_lot` based
//! `OnceCell<T>` is smaller
//! by up to 16 bytes.
//!
//! This crate uses unsafe.
//!
//! # Related crates
//!
//! [double-checked-cell](https://github.com/niklasf/double-checked-cell)
//! [lazy-init](https://crates.io/crates/lazy-init)
//! [lazycell](https://crates.io/crates/lazycell)
//! [mitochondria](https://crates.io/crates/mitochondria)
//! [lazy_static](https://crates.io/crates/lazy_static)
//!



pub mod unsync
{



	use core::{cell::UnsafeCell,
	           hint::unreachable_unchecked,
	           ops::Deref};
	#[cfg(feature = "std")]
	use std::panic::{RefUnwindSafe,
	                 UnwindSafe};



	/// A cell which
	/// can be written
	/// to only once.
	/// Not thread
	/// safe.
	///
	/// Unlike `:td::cell::RefCell`, a `OnceCell` provides simple `&`
	/// references to
	/// the contents.
	///
	/// # Example
	/// ```
	/// use once_cell::unsync::OnceCell;
	///
	/// let cell = OnceCell::new();
	/// assert!(cell.get().is_none());
	///
	/// let value: &String = cell.get_or_init(|| {
	///     "Hello, World!".to_string()
	/// });
	/// assert_eq!(value, "Hello, World!");
	/// assert!(cell.get().is_some());
	/// ```



	pub struct OnceCell<T>
	{
		// Invariant: written to at most
		// once.
		inner : UnsafeCell<Option<T>>,
	}



	// Similarly to a `Sync` bound
	// on `sync::OnceCell`, we can
	// use `&unsync::OnceCell` to
	// sneak a `T` through
	// `catch_unwind`,
	// by initializing the cell in
	// closure and extracting the
	// value in the `Drop`.
	#[cfg(feature = "std")]



	impl<T : RefUnwindSafe+UnwindSafe>
		RefUnwindSafe for OnceCell<T>
	{
	}



	#[cfg(feature = "std")]



	impl<T : UnwindSafe> UnwindSafe for OnceCell<T>
	{
	}



	impl<T : PartialEq> PartialEq for OnceCell<T>
	{
		fn eq(&self,
		      other : &Self)
		      -> bool
		{



			self.get() ==
			other.get()
		}
	}



	impl<T : Eq> Eq for OnceCell<T>
	{
	}



	impl<T> OnceCell<T>
	{
		/// Creates a new empty cell.



		pub const fn new()
		    -> OnceCell<T>
		{



			OnceCell { inner: UnsafeCell::new(None) }
		}

		/// Gets the reference to the underlying value.
		///
		/// Returns `None` if the cell is empty.



		pub fn get(&self) -> Option<&T>
		{



			// Safe due to
			// `inner`'s invariant
			unsafe {



				&*self.inner.get()
			}.as_ref()
		}

		/// Sets the contents of this cell to `value`.
		///
		/// Returns `Ok(())` if the cell was empty and `Err(value)` if it was
		/// full.
		///
		/// # Example
		/// ```
		/// 
		///
		///
		/// use once_cell::unsync::OnceCell;
		///
		///
		///
		/// let cell = OnceCell::new();
		///
		///
		///
		/// assert!(
		///         cell.get()
		///             .is_none()
		/// );
		///
		///
		///
		/// assert_eq!(
		///            cell.set(92),
		///            Ok(())
		/// );
		///
		///
		///
		/// assert_eq!(
		///            cell.set(62),
		///            Err(62)
		/// );
		///
		///
		///
		/// assert!(
		///         cell.get()
		///             .is_some()
		/// );
		/// ```



		pub fn set(&self,
		           value : T)
		           -> Result<(), T>
		{



			let slot = unsafe {



				&*self.inner.get()
			};



			if slot.is_some()
			{



				return Err(value);
			}



			let slot = unsafe {



				&mut *self.inner.get()
			};



			// This is the only
			// place where we set
			// the slot, no races
			// due to reentrancy/
			// concurrency are
			// possible, and
			// we've
			// checked that slot
			// is currently
			// `None`, so this
			// write maintains
			// the `inner`'s
			// invariant.
			*slot = Some(value);



			Ok(())
		}

		/// Gets the contents of the cell, initializing it with `f`
		/// if the cell was empty.
		///
		/// # Panics
		///
		/// If `f` panics, the panic is propagated to the caller, and the cell
		/// remains uninitialized.
		///
		/// It is an error to reentrantly initialize the cell from `f`. Doing
		/// so results in a panic.
		///
		/// # Example
		/// ```
		/// use once_cell::unsync::OnceCell;
		///
		/// let cell = OnceCell::new();
		/// let value = cell.get_or_init(|| 92);
		/// assert_eq!(value, &92);
		/// let value = cell.get_or_init(|| unreachable!());
		/// assert_eq!(value, &92);
		/// ```



		pub fn get_or_init<F>(&self,
		                      f : F)
		                      -> &T
			where F : FnOnce()
			                 -> T,
		{



			enum Void {}



			match self.get_or_try_init(|| Ok::<T, Void>(f())) {
                Ok(val) => val,
                Err(void) => match void {},
            }
		}

		/// Gets the contents of the cell, initializing it with `f` if
		/// the cell was empty. If the cell was empty and `f` failed, an
		/// error is returned.
		///
		/// # Panics
		///
		/// If `f` panics, the panic is propagated to the caller, and the cell
		/// remains uninitialized.
		///
		/// It is an error to reentrantly initialize the cell from `f`. Doing
		/// so results in a panic.
		///
		/// # Example
		/// ```
		/// use once_cell::unsync::OnceCell;
		///
		/// let cell = OnceCell::new();
		/// assert_eq!(cell.get_or_try_init(|| Err(())), Err(()));
		/// assert!(cell.get().is_none());
		/// let value = cell.get_or_try_init(|| -> Result<i32, ()> {
		///     Ok(92)
		/// });
		/// assert_eq!(value, Ok(&92));
		/// assert_eq!(cell.get(), Some(&92))
		/// ```
        pub fn get_or_try_init<F, E>(&self, f: F) -> Result<&T, E>
        where
            F: FnOnce() -> Result<T, E>,
        {



			if let Some(val) = self.get() {
                return Ok(val);
            }



			let val = f()?;



			assert!(self.set(val).is_ok(), "reentrant init");



			Ok(
			   self.get()
			       .unwrap(),
			)
		}
	}



	/// A value which
	/// is initialized
	/// on the first
	/// access.
	///
	/// # Example
	/// ```
	/// 
	///
	///
	/// use once_cell::unsync::Lazy;
	///
	///
	///
	/// let lazy: Lazy<i32> = Lazy::new(|| {
	///     println!("initializing");
	///     92
	/// });
	///
	///
	///
	/// println!("ready");
	///
	///
	///
	/// println!(
	///          "{}",
	///          *lazy
	/// );
	///
	///
	///
	/// println!(
	///          "{}",
	///          *lazy
	/// );
	///
	/// // Prints:
	/// //   ready
	/// //   initializing
	/// //   92
	/// //   92
	/// ```



	pub struct Lazy<T, F=fn() -> T>
	{
		cell : OnceCell<T>,
		init : UnsafeCell<Option<F>>,
	}



	impl<T, F> Lazy<T, F>
	{
		/// Creates a new lazy value with the given initializing function.
		///
		/// # Example
		/// ```
		/// # fn main() {
		/// use once_cell::unsync::Lazy;
		///
		/// let hello = "Hello, World!".to_string();
		///
		/// let lazy = Lazy::new(|| hello.to_uppercase());
		///
		/// assert_eq!(&*lazy, "HELLO, WORLD!");
		/// # }
		/// ```



		pub const fn new(init : F)
		                 -> Lazy<T, F>
		{



			Lazy { cell: OnceCell::new(), init: UnsafeCell::new(Some(init)) }
		}
	}



	impl<T, F : FnOnce() -> T> Lazy<T, F>
	{
		/// Forces the evaluation of
		/// this lazy value and
		/// returns a reference to
		/// the result.
		///
		/// This is equivalent to the
		/// `Deref` impl, but is
		/// explicit.
		///
		/// # Example
		/// ```
		/// use once_cell::unsync::Lazy;
		///
		/// let lazy = Lazy::new(|| 92);
		///
		/// assert_eq!(Lazy::force(&lazy), &92);
		/// assert_eq!(&*lazy, &92);
		/// ```
		pub fn force(this: &Lazy<T, F>) -> &T {
            // Safe because closure is guaranteed to be called at most once
            // so we only call `F` once, this also guarantees no race conditions
            this.cell.get_or_init(|| unsafe {
                match (*this.init.get()).take() {
                    Some(f) => f(),
                    None => unreachable_unchecked(),
                }
            })
        }
	}



	impl<T, F : FnOnce() -> T> Deref for Lazy<T, F>
	{
		type Target = T;

		fn deref(&self) -> &T
		{



			Lazy::force(self)
		}
	}
}



#[cfg(feature = "std")]



pub mod sync
{



	use crate::OnceCell as Imp;
	use std::{cell::UnsafeCell,
	          hint::unreachable_unchecked};



	/// A thread-safe
	/// cell which
	/// can be written
	/// to only once.
	///
	/// Unlike `std::sync::Mutex`, a `OnceCell` provides simple `&` references
	/// to the contents.
	///
	/// # Example
	/// ```
	/// use once_cell::sync::OnceCell;
	///
	/// static CELL: OnceCell<String> = OnceCell::new();
	/// assert!(CELL.get().is_none());
	///
	/// std::thread::spawn(|| {
	///     let value: &String = CELL.get_or_init(|| {
	///         "Hello, World!".to_string()
	///     });
	///     assert_eq!(value, "Hello, World!");
	/// }).join().unwrap();
	///
	/// let value: Option<&String> = CELL.get();
	/// assert!(value.is_some());
	/// assert_eq!(value.unwrap().as_str(), "Hello, World!");
	/// ```



	pub struct OnceCell<T>(Imp<T>);



	impl<T : PartialEq> PartialEq for OnceCell<T>
	{
		fn eq(&self,
		      other : &OnceCell<T>)
		      -> bool
		{



			self.get() ==
			other.get()
		}
	}



	impl<T : Eq> Eq for OnceCell<T>
	{
	}



	impl<T> OnceCell<T>
	{
		/// Creates a new empty cell.



		pub const fn new()
		    -> OnceCell<T>
		{



			OnceCell(Imp::new())
		}

		/// Gets the reference to the underlying value.
		///
		/// Returns `None` if the cell is empty, or being initialized. This
		/// method never blocks.



		pub fn get(&self) -> Option<&T>
		{



			self.0
			    .get()
		}

		/// Gets the contents of the cell, initializing it with `f` if the cell
		/// was empty.
		///
		/// Many threads may call `get_or_init` concurrently with different
		/// initializing functions, but it is guaranteed that only one function
		/// will be executed.
		///
		/// # Panics
		///
		/// If `f` panics, the panic is propagated to the caller, and the cell
		/// remains uninitialized.
		///
		/// It is an error to reentrantly initialize the cell from `f`. The
		/// exact outcome is unspecified. Current implementation deadlocks, but
		/// this may be changed to a panic in the future.
		///
		/// # Example
		/// ```
		/// use once_cell::sync::OnceCell;
		///
		/// let cell = OnceCell::new();
		/// let value = cell.get_or_init(|| 92);
		/// assert_eq!(value, &92);
		/// let value = cell.get_or_init(|| unreachable!());
		/// assert_eq!(value, &92);
		/// ```



		pub fn get_or_init<F>(&self,
		                      f : F)
		                      -> &T
			where F : FnOnce()
			                 -> T,
		{



			enum Void {}



			match self.get_or_try_init(|| Ok::<T, Void>(f())) {
                Ok(val) => val,
                Err(void) => match void {},
            }
		}

		/// Gets the contents of the cell, initializing it with `f` if
		/// the cell was empty. If the cell was empty and `f` failed, an
		/// error is returned.
		///
		/// # Panics
		///
		/// If `f` panics, the panic is propagated to the caller, and
		/// the cell remains uninitialized.
		///
		/// It is an error to reentrantly initialize the cell from `f`.
		/// The exact outcome is unspecified. Current implementation
		/// deadlocks, but this may be changed to a panic in the future.
		///
		/// # Example
		/// ```
		/// use once_cell::sync::OnceCell;
		///
		/// let cell = OnceCell::new();
		/// assert_eq!(cell.get_or_try_init(|| Err(())), Err(()));
		/// assert!(cell.get().is_none());
		/// let value = cell.get_or_try_init(|| -> Result<i32, ()> {
		///     Ok(92)
		/// });
		/// assert_eq!(value, Ok(&92));
		/// assert_eq!(cell.get(), Some(&92))
		/// ```
        pub fn get_or_try_init<F, E>(&self, f: F) -> Result<&T, E>
        where
            F: FnOnce() -> Result<T, E>,
        {



			self.0.get_or_try_init(f)
		}
	}



	/// A value which
	/// is initialized
	/// on the first
	/// access.
	///
	/// This type is
	/// thread-safe
	/// and can be
	/// used in statics:
	///
	///
	/// # Example
	/// ```
	/// use std::collections::HashMap;
	///
	/// use once_cell::sync::Lazy;
	///
	/// static HASHMAP: Lazy<HashMap<i32, String>> = Lazy::new(|| {
	///     println!("initializing");
	///     let mut m = HashMap::new();
	///     m.insert(13, "Spica".to_string());
	///     m.insert(74, "Hoyten".to_string());
	///     m
	/// });
	///
	/// fn main() {
	///     println!("ready");
	///     std::thread::spawn(|| {
	///         println!("{:?}", HASHMAP.get(&13));
	///     }).join().unwrap();
	///     println!("{:?}", HASHMAP.get(&74));
	///
	///     // Prints:
	///     //   ready
	///     //   initializing
	///     //   Some("Spica")
	///     //   Some("Hoyten")
	/// }
	/// ```



	pub struct Lazy<T, F=fn() -> T>
	{
		cell : OnceCell<T>,
		init : UnsafeCell<Option<F>>,
	}



	// We never create a `&F` from
	// a `&Lazy<T, F>` so it is
	// fine to not impl `Sync`
	// for `F` we do create a
	// `&mut Option<F>` in
	// `force`, but this is
	// properly synchronized, so
	// it only happens once so it
	// also does not contribute to
	// this impl.
	unsafe impl<T, F : Send> Sync for Lazy<T, F>
		where OnceCell<T> : Sync
	{
	}



	// auto-derived `Send` impl is
	// OK.



	impl<T, F> Lazy<T, F>
	{
		/// Creates a new lazy value with the given initializing
		/// function.



		pub const fn new(f : F)
		                 -> Lazy<T, F>
		{



			Lazy { cell: OnceCell::new(), init: UnsafeCell::new(Some(f)) }
		}
	}



	impl<T, F : FnOnce() -> T> Lazy<T, F>
	{
		/// Forces the evaluation of
		/// this lazy value and
		/// returns a reference to
		/// result. This is equivalent
		/// to the `Deref` impl, but
		/// is explicit.
		///
		/// # Example
		/// ```
		/// use once_cell::sync::Lazy;
		///
		/// let lazy = Lazy::new(|| 92);
		///
		/// assert_eq!(Lazy::force(&lazy), &92);
		/// assert_eq!(&*lazy, &92);
		/// ```
		pub fn force(this: &Lazy<T, F>) -> &T {
            // Safe because closure is guaranteed to be called at most once
            // so we only call `F` once, this also guarantees no race conditions
            this.cell.get_or_init(|| unsafe {
                match (*this.init.get()).take() {
                    Some(f) => f(),
                    None => unreachable_unchecked(),
                }
            })
        }
	}



	impl<T, F : FnOnce() -> T> ::std::ops::Deref for Lazy<T, F>
	{
		type Target = T;

		fn deref(&self) -> &T
		{



			Lazy::force(self)
		}
	}



	/// ```compile_fail
	/// struct S(*mut ());
	/// unsafe impl Sync for S {}
	///
	/// fn share<T: Sync>(_: &T) {}
	/// share(&once_cell::sync::OnceCell::<S>::new());
	/// ```
	///
	/// ```compile_fail
	/// struct S(*mut ());
	/// unsafe impl Sync for S {}
	///
	/// fn share<T: Sync>(_: &T) {}
	/// share(&once_cell::sync::Lazy::<S>::new(|| unimplemented!()));
	/// ```



	fn _dummy()
	{
	}
}



// There's a lot of scary
// concurrent code in this
// module, but it is copied
// from `std::sync::Once` with
// two changes:
//   * no poisoning
//   * init function can fail



use std::{cell::UnsafeCell,
          marker::PhantomData,
          panic::{RefUnwindSafe,
                  UnwindSafe},
          ptr,
          sync::atomic::{AtomicBool,
                         AtomicUsize,
                         Ordering},
          thread::{self,
                   Thread}};



#[derive(Debug)]



pub(crate) struct OnceCell<T>
{
	// This `state` word is actually an encoded
	// version of just a pointer to a
	// `Waiter`, so we add the `PhantomData`
	// appropriately.
	state : AtomicUsize,
	_marker : PhantomData<*mut Waiter>,
	// FIXME: switch to `std::mem::MaybeUninit`
	// once we are ready to bump MSRV
	// that far. It was stabilized in 1.36.0, so,
	// if you are reading this and it's higher
	// than 1.46.0 outside, please send a PR! ;)
	// (and to the same for `Lazy`, while we are
	// at it).
	value : UnsafeCell<Option<T>>,
}



// Why do we need `T: Send`?
// Thread A creates a
// `OnceCell` and shares it
// with scoped thread B, which
// fills the cell, which is
// then destroyed by A. That
// is, destructor observes
// a sent value.
unsafe impl<T : Sync+Send> Sync for OnceCell<T>
{
}



unsafe impl<T : Send> Send for OnceCell<T>
{
}



impl<T : RefUnwindSafe+UnwindSafe> RefUnwindSafe
	for OnceCell<T>
{
}



impl<T : UnwindSafe> UnwindSafe for OnceCell<T>
{
}



// Three states that a
// OnceCell can be in, encoded
// into the lower bits of
// `state` in the OnceCell
// structure.
const INCOMPLETE : usize = 0x0;



const RUNNING : usize = 0x1;



const COMPLETE : usize = 0x2;



// Mask to learn about the
// state. All other bits are
// the queue of waiters if
// this is in the RUNNING
// state.
const STATE_MASK : usize = 0x3;



// Representation of a node in
// the linked list of waiters
// in the RUNNING state.
struct Waiter
{
	thread : Option<Thread>,
	signaled : AtomicBool,
	next : *mut Waiter,
}



// Helper struct used to clean
// up after a closure call
// with a `Drop`
// implementation to also run
// on panic.
struct Finish<'a>
{
	failed : bool,
	my_state : &'a AtomicUsize,
}



impl<T> OnceCell<T>
{
	pub(crate) const fn new() -> OnceCell<T>
	{



		OnceCell {
            state: AtomicUsize::new(INCOMPLETE),
            _marker: PhantomData,
            value: UnsafeCell::new(None),
        }
	}

	pub(crate) fn get(&self) -> Option<&T>
	{



		if self.is_completed()
		{



			let slot: &Option<T> = unsafe { &*self.value.get() };



			match slot {
                Some(value) => Some(value),
                // This unsafe does improve performance, see `examples/bench`.
                None => unsafe { std::hint::unreachable_unchecked() },
            }
		}
		else
		{



			None
		}
	}

	pub fn get_or_try_init<F, E>(
		&self,
		f : F)
		-> Result<&T, E>
		where F : FnOnce() -> Result<T, E>,
	{



		// Fast path check
		if let Some(value) = self.get()
		{



			return Ok(value);
		}



		let mut f = Some(f);



		let mut err : Option<E> = None;



		let slot = &self.value;



		get_or_try_init_inner(&self.state, &mut || {
            let f = f.take().unwrap();
            match f() {
                Ok(value) => {
                    unsafe { *slot.get() = Some(value) };
                    true
                }
                Err(e) => {
                    err = Some(e);
                    false
                }
            }
        });



		match err
		{
			| Some(err) =>
				Err(err),
			| None =>
			{



				let value: &T = unsafe { &*slot.get() }.as_ref().unwrap();



				Ok(value)
			},
		}
	}

	#[inline]



	fn is_completed(&self) -> bool
	{



		// An `Acquire` load is enough
		// because that makes all the
		// initialization operations
		// visible to us, and, this
		// being a fast path, weaker
		// ordering helps with
		// performance. This `Acquire`
		// synchronizes with
		// `SeqCst` operations on the
		// slow path.
		self.state
		    .load(Ordering::Acquire) ==
		COMPLETE
	}
}



// Note: this is intentionally
// monomorphic
#[cold]



fn get_or_try_init_inner(my_state : &AtomicUsize,
                         init : &mut dyn FnMut() -> bool)
                         -> bool
{



	// This cold path uses SeqCst
	// consistently because the
	// performance difference
	// really does not matter
	// there, and
	// SeqCst minimizes the
	// chances of something going
	// wrong.
	let mut state =
		my_state.load(Ordering::SeqCst);



	'outer: loop
	{



		match state {
            // If we're complete, then there's nothing to do, we just
            // jettison out as we shouldn't run the closure.
            COMPLETE => return true,

            // Otherwise if we see an incomplete state we will attempt to
            // move ourselves into the RUNNING state. If we succeed, then
            // the queue of waiters starts at null (all 0 bits).
            INCOMPLETE => {
                let old = my_state.compare_and_swap(state, RUNNING, Ordering::SeqCst);
                if old != state {
                    state = old;
                    continue;
                }

                // Run the initialization routine, letting it know if we're
                // poisoned or not. The `Finish` struct is then dropped, and
                // the `Drop` implementation here is responsible for waking
                // up other waiters both in the normal return and panicking
                // case.
                let mut complete = Finish { failed: true, my_state };
                let success = init();
                complete.failed = !success;
                return success;
            }

            // All other values we find should correspond to the RUNNING
            // state with an encoded waiter list in the more significant
            // bits. We attempt to enqueue ourselves by moving us to the
            // head of the list and bail out if we ever see a state that's
            // not RUNNING.
            _ => {
                assert!(state & STATE_MASK == RUNNING);
                let mut node = Waiter {
                    thread: Some(thread::current()),
                    signaled: AtomicBool::new(false),
                    next: ptr::null_mut(),
                };
                let me = &mut node as *mut Waiter as usize;
                assert!(me & STATE_MASK == 0);

                while state & STATE_MASK == RUNNING {
                    node.next = (state & !STATE_MASK) as *mut Waiter;
                    let old = my_state.compare_and_swap(state, me | RUNNING, Ordering::SeqCst);
                    if old != state {
                        state = old;
                        continue;
                    }

                    // Once we've enqueued ourselves, wait in a loop.
                    // Afterwards reload the state and continue with what we
                    // were doing from before.
                    while !node.signaled.load(Ordering::SeqCst) {
                        thread::park();
                    }
                    state = my_state.load(Ordering::SeqCst);
                    continue 'outer;
                }
            }
        }
	}
}



impl Drop for Finish<'_>
{
	fn drop(&mut self)
	{



		// Swap out our state with
		// however we finished. We
		// should only ever see
		// an old state which was
		// RUNNING.
		let queue = if self.failed
		{



			self.my_state.swap(INCOMPLETE, Ordering::SeqCst)
		}
		else
		{



			self.my_state.swap(COMPLETE, Ordering::SeqCst)
		};



		assert_eq!(
		           queue & STATE_MASK,
		           RUNNING
		);



		// Decode the RUNNING to a
		// list of waiters, then walk
		// that entire list and wake
		// them up. Note that it is
		// crucial that after we store
		// `true` in the node it can
		// be free'd! As a result we
		// load the `thread` to
		// signal ahead of time and
		// then unpark it after the
		// store.
		unsafe {



			let mut queue = (queue & !STATE_MASK) as *mut Waiter;



			while !queue.is_null() {
                let next = (*queue).next;
                let thread = (*queue).thread.take().unwrap();
                (*queue).signaled.store(true, Ordering::SeqCst);
                thread.unpark();
                queue = next;
            }
		}
	}
}
