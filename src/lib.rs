#![feature(tuple_trait)]
#![warn(clippy::all)]

pub mod codegen;
pub mod ast;
pub mod utils;
pub mod parser;
pub mod error;

pub use utils::Ref;

pub type Variable = String;

pub mod traits {
    use super::Type;

    pub enum Component {
        Type(Type),
        Constant(u64),
    }

    pub trait Limit {
        fn id(&self) -> (u32, Vec<Component>);
    }

    pub trait Node {
    }
}

pub struct Limit;

pub struct Generic {
    pub name: String,
    pub limits: Vec<Limit>,
}

pub enum Type {
    Type(Vec<Limit>),
    TypeOf(Variable),
    Generic(Ref<Generic>),
}

pub enum Children {
    None,
    Block(Vec<Node>),
}
