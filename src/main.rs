use plac::codegen::asm::*;
use plac::codegen::interpreter::*;

fn main() {
    let instructions:Vec<Instruction> = vec![
        "add R2 0 0 U",
        "add R0 0 1 U",
        "add R1 0 2 U",
        "swap R0 R1", 
        "add R1 R0 R1 U",
        "print R1",
        "add R2 R2 1 U",
        "cmp R2 15",
        "jmp E 40", 
        "jmp A 12",
        "halt"
    ].into_iter().map(|x| Instruction::try_from(x.to_string()).unwrap_or_else(|_| {panic!("{}", x)})).collect();
    println!("{:#?}", instructions);
    println!("====================");
    let mut interpreter = Interpreter::new();
    interpreter.load(instructions);
    interpreter.set_register(Register::StackPointer, 1000);
    let err = interpreter.execute();
    println!("====================");
    println!("{:?}", err);
    println!("{}", interpreter.debug());
}
