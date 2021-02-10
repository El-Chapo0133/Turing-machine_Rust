mod turing_machine;



fn main() {

    let compiler = turing_machine::lang_compiler::Compiler::new();

    compiler.compile(String::from("./test-language.txt"));

    let mut turing_machine = turing_machine::turing_machine::TuringMachine::new();

    turing_machine.load_executable();

    turing_machine.print_tape();

    turing_machine.execute();

    turing_machine.print_tape();

    println!("Code me daddy!");
}
