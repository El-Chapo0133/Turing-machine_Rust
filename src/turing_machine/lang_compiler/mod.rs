
extern crate regex;

mod file_reader;
mod file_writer;
mod data {
    pub const EXECUTABLEFILE: &'static str  = "./_resources/executable.json";
    pub const KEYS: [&'static str; 4] = ["write", "move", "goto", "base_tape"];
}


use serde_json;
use serde::{Deserialize, Serialize};

#[derive(serde::Serialize, Deserialize, Debug)]
pub struct Output {
    pub executable: Vec<ExecutableStep>,
    pub base_tape: String
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ExecutableStep {
    pub name: String,
    pub todo: Vec<Step>
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Step {
    pub read: String,
    pub write: String,
    pub r#move: u8, // need the "r#", but will be identified as "move"
    pub goto: String
}
impl Output {
    pub fn new() -> Output {
        Output {
            executable: Vec::<ExecutableStep>::new(),
            base_tape: String::from(""),
        }
    }
}
impl ExecutableStep {
    pub fn new() -> ExecutableStep {
        ExecutableStep {
            name: String::new(),
            todo: Vec::<Step>::new(),
        }
    }
}
impl Step {
    pub fn new() -> Step {
        Step {
            read: String::new(),
            write: String::new(),
            r#move: 0,
            goto: String::new(),
        }
    }
}



pub struct Compiler {

}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {

        }
    }

    pub fn compile(&self, filename: String) {
        let file_content = match file_reader::read_file(&filename) {
            Ok(r) => r,
            Err(why) => panic!("Could not read the file {0}: {1}", filename, why),
        };
        let content = remove_useless(&file_content);

        // println!("{:?}", content);

        match prevent_error(&content) {
            Ok(_) => println!("No error found, starting transpilling"),
            Err(_) => panic!("Got errors, try debugging your code before running it ^^"),
        }

        let mut output = Output::new();
        let mut index_step: usize = 0;
        let mut index_todo: usize = 0;
        let mut step_values_assigned = 0;
        let mut unclosed_bracket = 0;
        let mut in_step = false;
        let mut got_todo_read = false;
        let mut skip_it = false;
        for index in 0..content.len() {
            if skip_it {
                skip_it = false;
                continue;
            }
            // println!("{}", content[index]);
            if content[index] == "{" {
                unclosed_bracket += 1;
                continue;
            }
            if content[index] == "}" {
                unclosed_bracket -= 1;
                continue;
            } else {
                if content[index] == "base_tape" {
                    // println!("Base tape key found");
                    output.base_tape = remove_quotation_mark(String::from(content[index + 1]));
                    skip_it = true;
                    continue;
                } else if content[index] == "<endstep>;" {
                    in_step = false;
                    index_step += 1;
                    got_todo_read = false;
                } else {
                    if !in_step {
                        if unclosed_quotation_mark(content[index]) {
                            panic!("Expected a step name at {}", content[index]);
                        }
                        output.executable.push(ExecutableStep::new());
                        output.executable[index_step].name = String::from(content[index]);
                        output.executable[index_step].todo = Vec::<Step>::new();
                        in_step = true;
                        index_todo = 0;
                    } else {
                        if !got_todo_read {
                            if !is_a_string(content[index]) {
                                panic!("Expected {} to be a string instead of something else", content[index]);
                            }
                            output.executable[index_step].todo.push(Step::new());
                            output.executable[index_step].todo[index_todo].read = remove_first_and_last(String::from(content[index]));
                            // println!("to read: {}", content[index]);
                            // println!("{:?}", output);
                            step_values_assigned += 1;
                            got_todo_read = true;
                            continue;
                        }
                        let (key, _) = keys_contains(content[index]);
                        // println!("key: {}", key);
                        // output.executable[index_step].todo[index_todo][key] == ;
                        if key == "write" {
                            if !is_a_string(content[index + 1]) {
                                panic!("Expected {} to be a string instead of something else", content[index + 1]);
                            }
                            output.executable[index_step].todo[index_todo].write = remove_quotation_mark(String::from(content[index + 1]));
                            step_values_assigned += 1;
                            skip_it = true;
                        } else if key == "goto" {
                            if !is_a_string(content[index + 1]) {
                                panic!("Expected {} to be a string instead of something else", content[index + 1]);
                            }
                            // println!("{}", content[index + 1]);
                            output.executable[index_step].todo[index_todo].goto = remove_quotation_mark(String::from(content[index + 1]));
                            step_values_assigned += 1;
                            skip_it = true;
                        } else if key == "move" {
                            if is_a_string(content[index + 1]) {
                                panic!("Expected {} to be a number instead of something else", content[index + 1]);
                            }
                            output.executable[index_step].todo[index_todo].r#move = remove_last_char(String::from(content[index + 1])).parse::<u8>().unwrap();
                            step_values_assigned += 1;
                            skip_it = true;
                        } else {
                            panic!("undefined key: {}.", key);
                        }
                        if step_values_assigned == 4 {
                            step_values_assigned = 0;
                            got_todo_read = false;
                            index_todo += 1;
                        }
                    }
                }
            }
        }

        if unclosed_bracket != 0 {
            panic!("Unclosed bracket somewhere ^^'");
        }

        let jsonified: String = match serde_json::to_string(&output) {
            Ok(r) => r,
            Err(why) => panic!("Could not serialize the struct Output: {}", why),
        };

        match file_writer::write_file(String::from(data::EXECUTABLEFILE), jsonified) {
            Ok(()) => return,
            Err(why) => panic!("Could not write the executable file: {}", why)
        }

        // println!("{:?}", output);
    }
}

fn prevent_error<'a>(content: &'a Vec<&str>) -> core::result::Result<(), ()> {
    let mut unclosed_bracket = 0;
    let mut got_errors = false;
    for (index, part) in content.iter().enumerate() {
        let (key, result) = keys_contains(part);
        if result {
            if get_last_char(content[index + 1]) != ';' {
                got_errors = true;
                println!("The value of \"{}\" must end with a ';'", key);
            }
            continue;
        }

        if is_a_string(part) {
            if !unclosed_quotation_mark(part) {
                got_errors = true;
                println!("{} quonotation mark are unclosed", part);
            }
        }

        if part == &"{" {
            unclosed_bracket += 1;
        } else if part == &"}" {
            unclosed_bracket -= 1;
        }
        if unclosed_bracket == 1 && part == &"}" && content[index + 1].chars().nth(0).unwrap() == '}' {
            panic!("Missing the <endstep>");
        }
    }


    if !got_errors {
        return Ok(())
    } else {
        return Err(())
    }
}

fn remove_useless<'a>(content: &'a String) -> Vec<&'a str> {
    let removed = remove_spaces_and_rf(&content);
    let mut to_return = Vec::<&'a str>::new();
    for line in removed {
        for word in line {
            to_return.push(word);
        }
    }
    to_return
}
fn remove_spaces_and_rf<'a>(content: &'a str) -> Vec<Vec<&'a str>> {
    content.lines().map(|line| {
        line.split_whitespace().collect()
    }).collect()
}
fn keys_contains(input: &str) -> (&str, bool) {
    for key in data::KEYS.iter() {
        if key == &input {
            return (key, true);
        }
    }
    ("", false)
}
fn get_last_char(input: &str) -> char {
    input.chars().last().unwrap()
}
fn get_first_char(input: &str) -> char {
    input.chars().next().unwrap()
}
fn is_a_string(input: &str) -> bool {
    let first_char = get_first_char(input);
    let last_char = get_last_char(input);

    first_char == '"' || last_char == '"'
}
fn unclosed_quotation_mark(input: &str) -> bool {
    let re = regex::Regex::new(r";").unwrap();
    let removed = re.split(input).collect::<Vec<&str>>()[0];
    get_first_char(removed) == get_last_char(removed)
}
fn remove_first_and_last(input: String) -> String {
    let mut to_return = String::new();
    for index in 1..input.len() - 1 {
        to_return.push(input.chars().nth(index).unwrap());
    }
    to_return
}
fn remove_last_char(input: String) -> String {
    let mut to_return = String::new();
    for index in 0..input.len() - 1 {
        to_return.push(input.chars().nth(index).unwrap());
    }
    to_return
}
fn remove_quotation_mark(input: String) -> String {
    let mut to_return = String::new();
    for index in 1..input.len() - 2 {
        to_return.push(input.chars().nth(index).unwrap());
    }
    to_return
}