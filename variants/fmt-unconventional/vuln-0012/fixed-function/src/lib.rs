use std::sync::Arc;



/// A hashconsed value.



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
