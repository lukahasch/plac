#![warn(clippy::all)]

pub mod codegen;

pub mod node {
    pub use node_trait::Node as Trait;

    pub struct Node {}

    pub mod node_trait {
        pub trait Node {}
    }
}
