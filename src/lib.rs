#![feature(tuple_trait)]
#![warn(clippy::all)]

pub mod codegen;
pub mod ast;
pub mod utils;
pub mod parser;
pub mod error;
pub mod limits;

use std::collections::HashMap;
use crate::limits::LimitGenerator;
pub use utils::Ref;
use limits::Limit;

pub type Name = String;

pub struct Generic {
    pub name: String,
    pub limits: Vec<Limit>,
}

#[derive(Clone, PartialEq)]
pub enum Type {
    Type(Vec<Limit>),
    TypeOf(Box<Variable>),
    Generic(Ref<Generic>),
}

pub enum Children {
    None,
    Block(Vec<Node>),
}

#[derive(Clone, PartialEq)]
pub struct Variable(pub Type, pub Option<u64>);

pub struct StackFrame {
    pub module: Ref<Module>,
    pub variables: HashMap<Name, Variable>,
}

pub struct Context {
    pub stackframes: HashMap<Ref<StackFrame>, StackFrame>,
    pub modules: HashMap<Ref<Module>, Module>,
    pub functions: HashMap<Ref<Function>, Function>,
    pub generics: HashMap<Ref<Generic>, Generic>,
    pub limit_generators: HashMap<Ref<LimitGenerator>, LimitGenerator>,
}

pub struct Module {
    pub parent: Option<Ref<Module>>,
    pub components: Vec<Component>,
    pub limits: HashMap<Name, Ref<LimitGenerator>>,
    pub name: String,
}

pub enum Component {
    Function(Ref<Function>),
    Variable(Variable),
}

pub struct Function {
    pub name: String,
    pub generics: Vec<Ref<Generic>>,
    pub args: Vec<Variable>,
    pub ret: Type,
    pub body: Vec<Node>,
}
