use std::{any::{Any, TypeId}, collections::HashSet, ops::{Deref, DerefMut}};
use atomic_refcell::{AtomicRef, AtomicRefCell, AtomicRefMut};

/// The Resource trait to annotate which structs are.
pub trait Resource: Any + Send + Sync {
   fn as_any(&self) -> &dyn Any;

   fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// Struct to represent the actual borrowing state of the world resources.
pub struct ResourceBorrowState {
   pub immutable_borrows: HashSet<TypeId>,
   pub mutable_borrows: HashSet<TypeId>
}

impl ResourceBorrowState {
   pub fn new() -> Self {
      return Self {
         immutable_borrows: HashSet::new(),
         mutable_borrows: HashSet::new()
      };
   }

   pub fn try_borrow_immutable(&mut self, type_id: TypeId) -> bool {
      if self.mutable_borrows.contains(&type_id) {
         return false;
      } else {
         self.immutable_borrows.insert(type_id);
         return true;
      }
   }

   pub fn try_borrow_mutable(&mut self, type_id: TypeId) -> bool {
      if self.immutable_borrows.contains(&type_id) || self.mutable_borrows.contains(&type_id) {
         return false;
      } else {
         self.mutable_borrows.insert(type_id);
         return true;
      }
   }

   pub fn release_immutable(&mut self, type_id: TypeId) {
      self.immutable_borrows.remove(&type_id);
   }

   pub fn release_mutable(&mut self, type_id: TypeId) {
      self.mutable_borrows.remove(&type_id);
   }
}

/// Struct to represent the immutable reference of a resource.
pub struct ResourceRef<'a, T: Resource + 'static> {
   pub(crate) inner: AtomicRef<'a, Box<dyn Resource>>,
   pub(crate) type_id: TypeId,
   pub(crate) resource_borrow_state: &'a AtomicRefCell<ResourceBorrowState>,
   pub(crate) phantom_data: std::marker::PhantomData<T>
}

impl<'a, T: Resource + 'static> Deref for ResourceRef<'a, T> {
   type Target = T;

   fn deref(&self) -> &T {
      return self.inner.as_any().downcast_ref::<T>().unwrap();
   }
}

impl<'a, T: Resource + 'static> Drop for ResourceRef<'a, T> {
   fn drop(&mut self) {
      self.resource_borrow_state.borrow_mut().release_immutable(self.type_id);
   }
}

/// Struct to represent a mutable reference of a resource.
pub struct ResourceRefMut<'a, T: Resource + 'static> {
   pub(crate) inner: AtomicRefMut<'a, Box<dyn Resource>>,
   pub(crate) type_id: TypeId,
   pub(crate) resource_borrow_state: &'a AtomicRefCell<ResourceBorrowState>,
   pub(crate) phantom_data: std::marker::PhantomData<T>
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
      self.resource_borrow_state.borrow_mut().release_mutable(self.type_id);
   }
}
