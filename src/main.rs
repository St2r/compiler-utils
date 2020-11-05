use compiler_utils::grammar;

use std::collections::{HashMap, HashSet};
use std::collections::VecDeque;
use std::env::args;
use crate::Priority::{Less, More, Equal};
use std::u32::MAX;
use std::ops::Index;
use std::fs;

fn main() {
    let mut handlers: HashMap<&str, &dyn Fn(&[String]) -> ()> = HashMap::new();
    handlers.insert("opg", &process_opg);

    let args: Vec<String> = args().collect();
    let cmd: &str = args
        .get(1)
        .unwrap_or_else(|| { panic!("无效的命令") });
    let para: &[String] = &args[2..];
    handlers[cmd](para);
}

#[derive(PartialEq, Clone, Copy)]
enum Priority {
    Less,
    More,
    Equal,
    None,
}

// https://oj.karenia.cc/suite/1ft03dkbb240y
fn process_opg(para: &[String]) {
    let path: String = para[0].to_string();
    let mut input = fs::read_to_string(path).unwrap();
    input.push('#');
    let mut token_stack: VecDeque<char> = VecDeque::new();
    let mut n_stack: VecDeque<char> = VecDeque::new();
    let mut priority_stack: VecDeque<Priority> = VecDeque::new();
    let set_t: HashSet<char> = init_set_t();
    // let set_n: HashSet<char> = init_set_n();
    token_stack.push_back('#');
    let pb = PriorityBuilder::new();

    for char in input.chars() {
        if !set_t.contains(&char) && char != '#' {
            println!("E");
            return;
        }
        let mut p: Priority = pb.compare_priority(token_stack.back().unwrap(), &char);
        if p == Priority::None {
            println!("E");
            return;
        } else if p == Priority::More {
            while p == Priority::More {
                // 规约
                let mut s: VecDeque<char> = VecDeque::new();
                while !priority_stack.is_empty() && *priority_stack.back().unwrap() == Priority::Equal {
                    priority_stack.pop_back();
                    s.push_front(token_stack.pop_back().unwrap());
                }
                if priority_stack.is_empty() || *priority_stack.back().unwrap() != Priority::Less {
                    println!("RE");
                    return;
                }

                priority_stack.pop_back();
                s.push_front(token_stack.pop_back().unwrap());
                p = pb.compare_priority(token_stack.back().unwrap(), &char);

                // todo 规约
                match s.pop_front().unwrap() {
                    '(' => {
                        if s.pop_back().unwrap() != ')' || !s.is_empty() {
                            println!("RE");
                            return;
                        }
                        n_stack.pop_back();
                    },
                    '+' | '*'=> {
                        if !s.is_empty()  || n_stack.len() < 2 {
                            println!("RE");
                            return;
                        }
                        n_stack.pop_back();
                        n_stack.pop_back();
                    },
                    'i' => {
                        if !s.is_empty() {
                            println!("RE");
                            return;
                        }
                    },
                    _ => {
                        println!("RE");
                        return;
                    }
                }

                n_stack.push_back('N');

                println!("R");
            }
            if char == '#' {
                if !token_stack.is_empty() && n_stack.len() != 1 {
                    println!("RE");
                }
                return;
            }
            println!("I{}", char);
            priority_stack.push_back(p);
            token_stack.push_back(char);
        } else {
            if char == '#' {
                if !token_stack.is_empty() && n_stack.len() != 1 {
                    println!("RE");
                }
                return;
            }
            println!("I{}", char);
            priority_stack.push_back(p);
            token_stack.push_back(char);
        }
    }
}

fn init_set_t() -> HashSet<char> {
    ['+', '*', '(', ')', 'i'].iter().cloned().collect::<HashSet<char>>()
}

fn init_set_n() -> HashSet<char> {
    ['E', 'F', 'T'].iter().cloned().collect::<HashSet<char>>()
}

struct PriorityBuilder {
    priority_table: [[Priority; 6]; 6],
}

impl PriorityBuilder {
    fn new() -> PriorityBuilder {
        PriorityBuilder {
            priority_table: [
                [Priority::More, Priority::Less, Priority::Less, Priority::Less, Priority::More, Priority::More],
                [Priority::More, Priority::More, Priority::Less, Priority::Less, Priority::More, Priority::More],
                [Priority::More, Priority::More, Priority::None, Priority::None, Priority::More, Priority::More],
                [Priority::Less, Priority::Less, Priority::Less, Priority::Less, Priority::Equal, Priority::None],
                [Priority::More, Priority::More, Priority::None, Priority::None, Priority::More, Priority::More],
                [Priority::Less, Priority::Less, Priority::Less, Priority::Less, Priority::Less, Priority::None],
            ]
        }
    }

    fn token_to_int(ch: &char) -> usize {
        match ch {
            '+' => 0,
            '*' => 1,
            'i' => 2,
            '(' => 3,
            ')' => 4,
            '#' => 5,
            _ => 999,
        }
    }

    fn compare_priority(&self, x: &char, y: &char) -> Priority {
        self.priority_table[PriorityBuilder::token_to_int(&x)][PriorityBuilder::token_to_int(&y)]
    }
}

