use std::any::Any;

pub unsafe trait Reflect: Any + Send + Sync + 'static {
    fn TypeName(&self) -> &'static str;
}
