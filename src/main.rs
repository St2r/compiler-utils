use compiler_utils::grammar;

use std::collections::{HashMap, HashSet};
use std::collections::VecDeque;
use std::env::args;

fn main() {
    let mut handlers: HashMap<&str, &dyn Fn(&[String]) -> ()> = HashMap::new();
    handlers.insert("opg", &process_opg);

    let args: Vec<String> = args().collect();
    let cmd: &str = args
        .get(1)
        .unwrap_or_else(|| { panic!("无效的命令") });
    let para: &[String] = &args[1..];
    handlers[cmd](para);
}

fn process_opg(para: &[String]) {
    println!("opg start");
    let input: String = (&para[0]).to_string();
    let mut token_t: VecDeque<char> = VecDeque::new();
    let mut token_n: VecDeque<char> = VecDeque::new();
    let set_t: HashSet<char> = init_set_t();
    let set_n: HashSet<char> = init_set_n();
    token_t.push_back('#');
}

fn init_set_t() -> HashSet<char> {
    ['+', '*', '(', ')', 'i'].iter().cloned().collect::<HashSet<char>>()
}

fn init_set_n() -> HashSet<char> {
    ['E', 'F', 'T'].iter().cloned().collect::<HashSet<char>>()
}
