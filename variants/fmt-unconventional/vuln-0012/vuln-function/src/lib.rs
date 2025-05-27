use std::sync::Arc;



/// Stores a hash consed
/// element and its hash in
/// order to avoid recomputing
/// it every time.



pub struct HConsed<T>
{
	/// The actual
	/// element.
	elm : Arc<T>,
	/// Unique identifier of the element.
	uid : u64,
}



impl<T> HConsed<T>
{
	/// The unique
	/// identifier of
	/// the element.
	#[inline]



	pub fn uid(&self) -> u64 { self.uid }
}



unsafe impl<T> Sync for HConsed<T> {}
unsafe impl<T> Send for HConsed<T> {}
