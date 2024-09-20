use cpu::Cpu;
use cpu::State;
use serde::Deserialize;
use std::env;
use std::fs;

mod bus;
mod cpu;

const TEST_PATH: &str = "../sm83/v1/";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!(":(");
    }

    if args[1] == "all" {
        let mut failed = false;
        for file in fs::read_dir("../sm83/v1/").unwrap() {
            failed = failed || run_tests(file.unwrap().path().to_str().unwrap());
        }
        if !failed {
            println!("all tests passed! wow!")
        }
    } else {
        for arg in args.iter().skip(1) {
            run_tests(&(TEST_PATH.to_owned() + arg));
        }
    }
}

fn run_tests(path: &str) -> bool {
    let mut failed = false;
    let data = fs::read_to_string(path).unwrap();
    let tests: Vec<Test> = serde_json::from_str(&data).unwrap();
    println!("running test: {}", path);

    let mut cpu = Cpu::new();

    for test in tests {
        cpu.set_state(&test.initial);
        // println!("test name: {}", test.name);
        cpu.execute_instruction();
        let final_state = cpu.to_state();
        if test.end != final_state {
            failed = true;
            println!("----------------------------------------------------------------");
            println!(
                "test name: {}\ninitial: {:#?}\nexpected: {:#?}\nactual: {:#?}",
                test.name, test.initial, test.end, final_state
            );
            println!("----------------------------------------------------------------");
        }
        cpu.reset();
    }

    if !failed {
        println!("all tests passed! yay!");
    }
    failed
}

#[derive(Debug, Deserialize)]
struct Test {
    name: String,
    initial: State,
    #[serde(alias = "final")]
    end: State,
}
