use std::any::TypeId;
use crate::Type;

#[derive(Clone, PartialEq)]
pub enum Component {
    Type(Type),
    Constant(u64),
}

pub trait Limitable: PartialOrd + PartialEq + 'static + Clone {
    fn id(&self) -> (u32, Vec<Component>);
    fn string(&self) -> String;
}

pub struct Limit {
    ptr: *mut u8,
    pub type_id: TypeId,
    id: unsafe fn(*mut u8) -> (u32, Vec<Component>),
    string: unsafe fn(*mut u8) -> String,
    drop: unsafe fn(*mut u8),
    partial_cmp: unsafe fn(*mut u8, *mut u8) -> Option<std::cmp::Ordering>,
    clone: unsafe fn(*mut u8) -> *mut u8,
}

impl Limit {
    pub fn new<T: Limitable>(data: T) -> Self {
        Self {
            ptr: Box::into_raw(Box::new(data)) as *mut u8,
            type_id: std::any::TypeId::of::<T>(),
            id: |ptr| unsafe { (*(ptr as *mut T)).id() },
            string: |ptr| unsafe { (*(ptr as *mut T)).string() },
            drop: |ptr| unsafe { Box::from_raw(ptr as *mut T); },
            partial_cmp: |ptr1, ptr2| unsafe { (*(ptr1 as *mut T)).partial_cmp(&*(ptr2 as *mut T)) },
            clone: |ptr| unsafe { Box::into_raw(Box::new((*(ptr as *mut T)).clone())) as *mut u8 },
        }
    }

    pub fn id(&self) -> (u32, Vec<Component>) {
        unsafe { (self.id)(self.ptr) }
    }

    pub fn string(&self) -> String {
        unsafe { (self.string)(self.ptr) }
    }

    pub fn type_id(&self) -> TypeId {
        self.type_id
    }

    pub fn cast<T:Limitable>(&mut self) -> Option<&mut T> {
        if self.type_id == std::any::TypeId::of::<T>() {
            Some(unsafe { &mut *(self.ptr as *mut T) })
        } else {
            None
        }
    }

    pub fn cast_ref<T:Limitable>(&self) -> Option<&T> {
        if self.type_id == std::any::TypeId::of::<T>() {
            Some(unsafe { &*(self.ptr as *const T) })
        } else {
            None
        }
    }
}

impl Drop for Limit {
    fn drop(&mut self) {
        unsafe { (self.drop)(self.ptr) }
    }
}

impl PartialEq for Limit {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl PartialOrd for Limit {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.type_id != other.type_id {
            return None;
        }
        unsafe { (self.partial_cmp)(self.ptr, other.ptr) }
    }
}

impl Clone for Limit {
    fn clone(&self) -> Self {
        Self {
            ptr: unsafe { (self.clone)(self.ptr) },
            type_id: self.type_id,
            id: self.id,
            string: self.string,
            drop: self.drop,
            partial_cmp: self.partial_cmp,
            clone: self.clone,
        }
    }
}

pub trait CanGenerate { 
    fn generate(&self, components: Vec<Component>) -> Limit;
}

pub type LimitGenerator = Box<dyn CanGenerate>;
