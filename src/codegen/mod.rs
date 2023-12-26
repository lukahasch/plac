use crate::utils::Map;

pub type Location = usize;

pub enum Type {
    I32,
    I64,
    F32,
    F64,
    U32,
    U64,
    Bool,
    Char,
    Ptr(Box<Type>),
    Array(Box<Type>, usize),
    Struct(Struct),
    FnPtr(Box<Function>),
    Lookup(String),
    TypeDef(TypeDef),
}

pub struct Struct {
    pub name: String,
    pub type_def: TypeDef,
    pub methods: Map<String,KnownFunction>,
}

pub struct TypeDef {
    pub fields: Map<String,Type>,
}

pub struct Function {
    pub convention: Convention,
    pub args: Vec<Type>,
    pub ret: Type,
}

pub enum Convention {
    Default,
}

pub struct KnownFunction {
    pub function: Function,
    pub code_block: Location,
}

pub enum Value {
    Ref(String),
    Copy(String),
    Deref(String),
    Constant(String),
    Function(Function, Location),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    U32(u32),
    U64(u64),
    Bool(bool),
    Char(char),
    Ptr(Box<Value>),
    Array(Vec<Value>),
}

pub struct CodeBlock {
    pub statements: Vec<Statement>,
}

pub enum ConstantType {
    Type(Type),
    KnownFunction(KnownFunction),
}

pub enum ConstantValue {
    KnownFunction(KnownFunction),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    U32(u32),
    U64(u64),
    Bool(bool),
    Char(char),
    Ptr(Value),
    Array(Vec<Value>),
    Function(Function, Location),
}

pub enum Statement {
    CreateConstant(String, ConstantType, ConstantValue),
    CreateVariable(String, Type, Value),
    Assign(String, Value),
    AssignCall(String, Value, Vec<Value>),
    Call(Value, Vec<Value>),
    CreateCall(String, Value, Vec<Value>),
    Return(Value),
    If(Value, Location, Location),
    While(Value, Location),
}
