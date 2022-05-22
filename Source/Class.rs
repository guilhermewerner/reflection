use crate::Property;
use std::any::{type_name, TypeId};
use std::borrow::Cow;
use std::collections::HashMap;

pub struct Class {
    id: TypeId,
    name: Cow<'static, str>,
    properties: HashMap<Cow<'static, str>, Property>,
}

impl Class {
    pub fn New<T: 'static>() -> Self {
        Self {
            id: TypeId::of::<T>(),
            name: type_name::<T>().into(),
            properties: HashMap::new(),
        }
    }

    pub fn GetId(&self) -> TypeId {
        self.id
    }

    pub fn GetName(&self) -> &str {
        &self.name
    }
}
