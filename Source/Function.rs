pub trait Function<Args = ()>: Send + Sync + 'static {
    type Result;
    fn Invoke(&self, args: Args) -> Self::Result;
}
