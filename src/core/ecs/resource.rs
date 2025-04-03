use std::{any::{Any, TypeId}, ops::{Deref, DerefMut}};
use atomic_refcell::{AtomicRef, AtomicRefCell, AtomicRefMut};
use super::world::BorrowState;

/// The Resource trait to annotate which structs are.
pub trait Resource: Any + Send + Sync {
   fn as_any(&self) -> &dyn Any;

   fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// Struct to represent the immutable reference of a resource.
pub struct ResourceRef<'a, T: Resource + 'static> {
   pub inner: AtomicRef<'a, Box<dyn Resource>>,
   pub type_id: TypeId,
   pub borrow_state: &'a AtomicRefCell<BorrowState>,
   pub phantom_data: std::marker::PhantomData<T>
}

impl<'a, T: Resource + 'static> Deref for ResourceRef<'a, T> {
   type Target = T;

   fn deref(&self) -> &T {
      return self.inner.as_any().downcast_ref::<T>().unwrap();
   }
}

impl<'a, T: Resource + 'static> Drop for ResourceRef<'a, T> {
   fn drop(&mut self) {
      self.borrow_state.borrow_mut().release_immutable(self.type_id);
   }
}

/// Struct to represent a mutable reference of a resource.
pub struct ResourceRefMut<'a, T: Resource + 'static> {
   pub inner: AtomicRefMut<'a, Box<dyn Resource>>,
   pub type_id: TypeId,
   pub borrow_state: &'a AtomicRefCell<BorrowState>,
   pub phantom_data: std::marker::PhantomData<T>
}

impl<'a, T: Resource + 'static> Deref for ResourceRefMut<'a, T> {
   type Target = T;

   fn deref(&self) -> &T {
      return self.inner.as_any().downcast_ref::<T>().unwrap();
   }
}

impl<'a, T: Resource + 'static> DerefMut for ResourceRefMut<'a, T> {
   fn deref_mut(&mut self) -> &mut T {
      return self.inner.as_any_mut().downcast_mut::<T>().unwrap();
   }
}

impl<'a, T: Resource + 'static> Drop for ResourceRefMut<'a, T> {
   fn drop(&mut self) {
      self.borrow_state.borrow_mut().release_mutable(self.type_id);
   }
}
