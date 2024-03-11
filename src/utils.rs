use std::marker::PhantomData;

pub struct Ref<T:?Sized>(pub u64, pub PhantomData<T>);
