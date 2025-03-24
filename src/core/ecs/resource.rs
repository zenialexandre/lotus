use std::any::Any;

/// The Resource trait to annotate which structs are.
pub trait Resource: Any + Send + Sync {
   fn as_any(&self) -> &dyn Any;

   fn as_any_mut(&mut self) -> &mut dyn Any;
}
