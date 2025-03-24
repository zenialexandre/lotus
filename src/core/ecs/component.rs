use std::any::Any;

/// The Component trait to annotate which structs are.
pub trait Component: Any + Send + Sync {
   fn as_any(&self) -> &dyn Any;

   fn as_any_mut(&mut self) -> &mut dyn Any;
}
