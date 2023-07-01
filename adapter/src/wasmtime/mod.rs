pub mod component;

pub struct Store;

pub trait AsContextMut {
    type Data;

    fn as_context_mut(&mut self) -> Store { Store{} }
}

pub use anyhow::Result;

//struct SomeContext<T> ;

impl AsContextMut for Store {
    type Data = ();
}
