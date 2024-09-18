use cpu::Cpu;
use cpu::State;
use serde::Deserialize;
use std::env;
use std::fs;

mod bus;
mod cpu;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!(":(");
    }

    run_tests(&args[1]);
}

fn run_tests(path: &str) {
    let mut failed = false;
    let data = fs::read_to_string(path).unwrap();
    let tests: Vec<Test> = serde_json::from_str(&data).unwrap();
    println!("running test: {}", path);

    for test in tests {
        let mut cpu = Cpu::from_state(&test.initial);
        cpu.execute_instruction();
        let final_state = cpu.to_state();
        if test.end != final_state {
            failed = true;
            println!(
                "test name: {}\ninitial: {:#?}\nexpected: {:#?}\nactual: {:#?}",
                test.name, test.initial, test.end, final_state
            );
        }
    }

    if !failed {
        println!("all tests passed! yay!");
    }
}

#[derive(Debug, Deserialize)]
struct Test {
    name: String,
    initial: State,
    #[serde(alias = "final")]
    end: State,
}
