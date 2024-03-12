use crate::Type;
use crate::Ref;
use crate::Children;
use crate::StackFrame;
use crate::Context;

pub trait Analyzable {
    fn analyze(&mut self, children: &mut Children, stackframe: Ref<StackFrame>, context: &mut Context) -> Type;
}
