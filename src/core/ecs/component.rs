use std::{any::{Any, TypeId}, collections::HashSet, ops::{Deref, DerefMut}};
use atomic_refcell::{AtomicRef, AtomicRefCell, AtomicRefMut};
use uuid::Uuid;

/// Struct to represent the actual borrowing state of the entities components.
pub struct ComponentBorrowState {
   pub immutable_borrows: HashSet<(TypeId, Uuid)>,
   pub mutable_borrows: HashSet<(TypeId, Uuid)>
}

impl ComponentBorrowState {
   pub fn new() -> Self {
      return Self {
         immutable_borrows: HashSet::new(),
         mutable_borrows: HashSet::new()
      };
   }

   pub fn try_borrow_immutable(&mut self, type_id: TypeId, entity_id: Uuid) -> bool {
      let key: (TypeId, Uuid) = (type_id, entity_id);
      
      if self.mutable_borrows.contains(&key) {
         return false;
      } else {
         self.immutable_borrows.insert(key);
         return true;
      }
   }

   pub fn try_borrow_mutable(&mut self, type_id: TypeId, entity_id: Uuid) -> bool {
      let key: (TypeId, Uuid) = (type_id, entity_id);

      if self.immutable_borrows.contains(&key) || self.mutable_borrows.contains(&key) {
         return false;
      } else {
         self.mutable_borrows.insert(key);
         return true;
      }
   }

   pub fn release_immutable(&mut self, type_id: TypeId, entity_id: Uuid) {
      self.immutable_borrows.remove(&(type_id, entity_id));
   }

   pub fn release_mutable(&mut self, type_id: TypeId, entity_id: Uuid) {
      self.mutable_borrows.remove(&(type_id, entity_id));
   }
}

/// The Component trait to annotate which structs are.
pub trait Component: Any + Send + Sync {
   fn as_any(&self) -> &dyn Any;

   fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// Struct to represent the immutable reference of a component.
pub struct ComponentRef<'a, T: Component + 'static> {
   pub inner: AtomicRef<'a, Box<dyn Component>>,
   pub type_id: TypeId,
   pub entity_id: Uuid,
   pub component_borrow_state: &'a AtomicRefCell<ComponentBorrowState>,
   pub phantom_data: std::marker::PhantomData<T>
}

impl<'a, T: Component + 'static> Deref for ComponentRef<'a, T> {
   type Target = T;

   fn deref(&self) -> &T {
      return self.inner.as_any().downcast_ref::<T>().unwrap();
   }
}

impl<'a, T: Component + 'static> Drop for ComponentRef<'a, T> {
   fn drop(&mut self) {
      self.component_borrow_state.borrow_mut().release_immutable(self.type_id, self.entity_id);
   }
}

/// Struct to represent a mutable reference of a component.
pub struct ComponentRefMut<'a, T: Component + 'static> {
   pub inner: AtomicRefMut<'a, Box<dyn Component>>,
   pub type_id: TypeId,
   pub entity_id: Uuid,
   pub component_borrow_state: &'a AtomicRefCell<ComponentBorrowState>,
   pub phantom_data: std::marker::PhantomData<T>
}

impl<'a, T: Component + 'static> Deref for ComponentRefMut<'a, T> {
   type Target = T;

   fn deref(&self) -> &T {
      return self.inner.as_any().downcast_ref::<T>().unwrap();
   }
}

impl<'a, T: Component + 'static> DerefMut for ComponentRefMut<'a, T> {
   fn deref_mut(&mut self) -> &mut T {
      return self.inner.as_any_mut().downcast_mut::<T>().unwrap();
   }
}

impl<'a, T: Component + 'static> Drop for ComponentRefMut<'a, T> {
   fn drop(&mut self) {
      self.component_borrow_state.borrow_mut().release_mutable(self.type_id, self.entity_id);
   }
}
