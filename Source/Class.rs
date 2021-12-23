use std::any::{type_name, TypeId};

pub struct Class {
    id: TypeId,
    name: &'static str,
}

impl Class {
    pub fn New<T: 'static>() -> Self {
        Self {
            id: TypeId::of::<T>(),
            name: type_name::<T>().into(),
        }
    }

    pub fn GetId(&self) -> TypeId {
        self.id
    }

    pub fn GetName(&self) -> &'static str {
        self.name
    }
}
