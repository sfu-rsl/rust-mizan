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



pub struct MapGuard<G, T>
{
	_guard : G,
	value : *const T,
}



impl<G, T> Deref for MapGuard<G, T>
{
	type Target = T;

	fn deref(&self) -> &T {
        unsafe { &*self.value }
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
		where F : Fn(&T) -> &R,
	{



		Map { access,
		      projection,
		      _t : PhantomData }
	}
}



impl<A, T, F, R> Access<R> for Map<A, T, F>
	where A : Access<T>,
	      F : Fn(&T) -> &R,
{
	type Guard = MapGuard<A::Guard, R>;

	fn load(&self) -> Self::Guard {
        let guard = self.access.load();
        let value: *const _ = (self.projection)(&guard);
        MapGuard {
            _guard: guard,
            value,
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

	fn deref(&self) -> &T {
        &self.0
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



pub struct Constant<T>(pub T);



impl<T: Clone> Access<T> for Constant<T> {
    type Guard = ConstantDeref<T>;
    fn load(&self) -> Self::Guard {
        ConstantDeref(self.0.clone())
    }
}
