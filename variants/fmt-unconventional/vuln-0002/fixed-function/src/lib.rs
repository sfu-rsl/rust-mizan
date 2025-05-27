#![deny(unsafe_code)]



use std::marker::PhantomData;
use std::ops::Deref;



/// Abstracts over ways code
/// can get access to a value
/// of type `T`.



pub trait Access<T>
{
	type Guard: Deref<Target=T>;



	fn load(&self) -> Self::Guard;
}



#[derive(Copy, Clone, Debug)]



pub struct MapGuard<G, F, T, R>
{
	guard : G,
	projection : F,
	_t : PhantomData<fn(&T) -> &R>,
}



impl<G, F, T, R> Deref for MapGuard<G, F, T, R>
	where G : Deref<Target=T>,
	      F : Fn(&T) -> &R,
{
	type Target = R;

	fn deref(&self) -> &R
	{



		(self.projection)(&self.guard)
	}
}



#[derive(Copy, Clone, Debug)]



pub struct Map<A, T, F>
{
	access : A,
	projection : F,
	_t : PhantomData<fn() -> T>,
}



impl<A, T, F> Map<A, T, F>
{
	pub fn new<R>(access : A,
	              projection : F)
	              -> Self
		where F : Fn(&T) -> &R+Clone,
	{



		Map { access,
		      projection,
		      _t : PhantomData }
	}
}



impl<A, F, T, R> Access<R> for Map<A, T, F>
	where A : Access<T>,
	      F : Fn(&T) -> &R+Clone,
{
	type Guard = MapGuard<A::Guard, F, T, R>;

	fn load(&self) -> Self::Guard
	{



		let guard = self.access
		                .load();



		MapGuard {
            guard,
            projection: self.projection.clone(),
            _t: PhantomData,
        }
	}
}



#[derive(Copy,
           Clone,
           Debug,
           Eq,
           PartialEq,
           Ord,
           PartialOrd,
           Hash)]



pub struct ConstantDeref<T>(T);



impl<T> Deref for ConstantDeref<T>
{
	type Target = T;

	fn deref(&self) -> &T { &self.0 }
}



#[derive(Copy,
           Clone,
           Debug,
           Eq,
           PartialEq,
           Ord,
           PartialOrd,
           Hash)]



pub struct Constant<T>(pub T);



impl<T : Clone> Access<T> for Constant<T>
{
	type Guard = ConstantDeref<T>;

	fn load(&self) -> Self::Guard
	{



		ConstantDeref(
		              self.0
		                  .clone(),
		)
	}
}
