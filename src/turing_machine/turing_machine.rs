
#[path = "json_reader/mod.rs"] mod json_reader;


pub struct TuringMachine {
    tape: Vec<u64>,
    executable: json_reader::Output,
}


impl TuringMachine {
    pub fn new() -> TuringMachine {
        TuringMachine {
            tape: Vec::new(),
            executable: json_reader::Output::new(),
        }
    }

    pub fn print_tape(&self) {
        for tape_value in &self.tape {
            if tape_value == &0 {
                continue;
            }
            print!("{}", *tape_value as u8 as char)
        }
        print!("\n");
    }

    pub fn load_executable(&mut self) {
        self.executable = json_reader::get_executable().expect("Could not load the executable");
        self.load_base_tape();

        println!("Executable and base_tape successfully loaded, step number: {}", self.executable.executable.len());
    }

    fn load_base_tape(&mut self) {
        self.tape.push(0 as u64); // means "<empty>"
        for value in self.executable.base_tape.as_bytes() {
            self.tape.push(*value as u64); // base_tape will be dereferenced
        }
        self.tape.push(0 as u64); // means "<empty>"
    }

    pub fn execute(&mut self) {
        let move_index = |index: usize, moving: u8| -> usize {
            if moving == 1 {
                return index + 1;
            } else if moving == 0 {
                return index - 1;
            } else {
                panic!("Value {} isn't supported", moving);
            }
        };
        let mut which_step: String = String::from("step1");
        let mut current_index: usize = 0;
        loop {
            let step_all_stages = get_step_all_stages(&self.executable.executable, &which_step);
            if self.tape.len() == current_index {
                self.tape.push(0 as u64); // means "<empty>"
            }
            let current_value = self.tape[current_index];
            let mut stage_found = false;

            for step_stage in step_all_stages {
                let value_parsed = check_empty_or_parse(step_stage.read.clone());
                // println!("Value parsed: {}", value_parsed);
                if value_parsed == current_value {
                    stage_found = true;
                    let value_to_write_parsed = check_empty_or_parse(step_stage.write.clone());
                    // println!("Value to write parsed: {}", value_to_write_parsed);
                    self.tape[current_index] = value_to_write_parsed;
                    current_index = move_index(current_index, step_stage.r#move);
                    which_step = step_stage.goto;
                    // println!("current_index is {}", current_index);
                    // println!("Step name moved to {}", which_step);
                    break;
                }
            }
            if !stage_found {
                panic!("Stage for \"{}\" is not assigned", current_value as u8 as char);
            }

            if which_step == "<end>" {
                println!("<end> key found");
                return;
            }
        }
    }
}

fn get_step_all_stages(steps: &Vec<json_reader::ExecutableStep>, input: &str) -> Vec<json_reader::Step> {
    for step in steps {
        if step.name == input {
            return step.todo.clone();
        }
    }
    panic!("Could not find the step {}", input)
}

fn check_empty_or_parse(input: String) -> u64 {
    if input == "<empty>" {
        // println!("<empty> key found");
        return 0;
    } else {
        return input.chars().collect::<Vec<char>>()[0] as u64;
    }
}