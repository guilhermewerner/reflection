use std::any::Any;

pub trait Reflect: Any + Send + Sync + 'static {}
