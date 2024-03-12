use std::marker::PhantomData;

pub struct Ref<T:?Sized>(pub u64, pub PhantomData<T>);

impl<T:?Sized> Clone for Ref<T> {
    fn clone(&self) -> Self {
        Self(self.0, PhantomData)
    }
}

impl<T:?Sized> Copy for Ref<T> {}

impl<T:?Sized> PartialEq for Ref<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T:?Sized> Eq for Ref<T> {}

impl<T:?Sized> PartialOrd for Ref<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T:?Sized> Ord for Ref<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

