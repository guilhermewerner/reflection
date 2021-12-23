use std::any::TypeId;
use std::collections::HashMap;

#[derive(Default)]
pub struct TypeRegistry {
    types: HashMap<TypeId, ()>,
}
