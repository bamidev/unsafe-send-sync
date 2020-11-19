use std::ops::{Deref, DerefMut};



/// A wrapper type that is always `Send`.
pub struct UnsafeSend<T> {
	pub i: T
}
unsafe impl<T> Send for UnsafeSend<T> {}

impl<T> UnsafeSend<T> {
	pub fn new( internal: T ) -> Self {
		Self { i: internal }
	}

	pub fn unwrap( self ) -> T {
		self.i
	}
}

impl<T> Clone for UnsafeSend<T> where
	T: Clone
{
	fn clone( &self ) -> Self {
		Self::new( self.i.clone() )
	}
}

impl<T> Default for UnsafeSend<T> where T: Default {
	fn default() -> Self {
		Self { i: T::default() }
	}
}

impl<T> Deref for UnsafeSend<T> {
	type Target = T;

	fn deref( &self ) -> &Self::Target  { &self.i }
}

impl<T> DerefMut for UnsafeSend<T> {
	fn deref_mut( &mut self ) -> &mut Self::Target  { &mut self.i }
}



/// A wrapper type that is always `Sync`.
pub struct UnsafeSync<T> {
	pub i: T
}
unsafe impl<T> Sync for UnsafeSync<T> {}

impl<T> UnsafeSync<T> {
	pub fn new( internal: T ) -> Self {
		Self { i: internal }
	}

	pub fn unwrap( self ) -> T {
		self.i
	}
}

impl<T> Clone for UnsafeSync<T> where
	T: Clone
{
	fn clone( &self ) -> Self {
		Self::new( self.i.clone() )
	}
}


impl<T> Default for UnsafeSync<T> where T: Default {
	fn default() -> Self {
		Self { i: T::default() }
	}
}

impl<T> Deref for UnsafeSync<T> {
	type Target = T;

	fn deref( &self ) -> &Self::Target  { &self.i }
}

impl<T> DerefMut for UnsafeSync<T> {
	fn deref_mut( &mut self ) -> &mut Self::Target  { &mut self.i }
}

/// A wrapper type that is always `Send` and `Sync`.
pub struct UnsafeSendSync<T> {
	pub i: T
}
unsafe impl<T> Send for UnsafeSendSync<T> {}
unsafe impl<T> Sync for UnsafeSendSync<T> {}

impl<T> UnsafeSendSync<T> {
	pub fn new( internal: T ) -> Self {
		Self { i: internal }
	}

	pub fn unwrap( self ) -> T { self.i }
}

impl<T> Default for UnsafeSendSync<T> where T: Default {
	fn default() -> Self {
		Self { i: T::default() }
	}
}

impl<T> Deref for UnsafeSendSync<T> {
	type Target = T;

	fn deref( &self ) -> &Self::Target  { &self.i }
}

impl<T> DerefMut for UnsafeSendSync<T> {
	fn deref_mut( &mut self ) -> &mut Self::Target  { &mut self.i }
}

impl<T> Clone for UnsafeSendSync<T> where
	T: Clone
{
	fn clone( &self ) -> Self {
		Self::new( self.i.clone() )
	}
}
