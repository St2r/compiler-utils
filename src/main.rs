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
        if char.is_ascii_whitespace() {
            continue;
        }

        // 判断是否是终结符
        if !set_t.contains(&char) && char != '#' {
            println!("E");
            return;
        }

        // 计算优先级并分情况
        let mut p: Priority = pb.compare_priority(token_stack.back().unwrap(), &char);
        if p == Priority::None {
            // 无法计算优先级或者两个终结符不能相邻
            println!("E");
            return;
        } else if p == Priority::More {
            // 出现大于，开始规约
            while p == Priority::More {
                // 规约
                while !priority_stack.is_empty() && *priority_stack.back().unwrap() == Priority::Equal {
                    priority_stack.pop_back();
                    token_stack.pop_back();
                }
                if priority_stack.is_empty() || *priority_stack.back().unwrap() != Priority::Less {
                    println!("RE");
                    return;
                }

                priority_stack.pop_back();
                let s = token_stack.pop_back().unwrap();
                p = pb.compare_priority(token_stack.back().unwrap(), &char);

                // todo 规约
                match s {
                    '(' => {
                        if n_stack.len() < 3
                            || n_stack.pop_back().unwrap() != ')'
                            || n_stack.pop_back().unwrap() != 'N'
                            || n_stack.pop_back().unwrap() != '(' {
                            println!("RE");
                            return;
                        }
                    }
                    '+' => {
                        if n_stack.len() < 3
                            || n_stack.pop_back().unwrap() != 'N'
                            || n_stack.pop_back().unwrap() != '+'
                            || n_stack.pop_back().unwrap() != 'N' {
                            println!("RE");
                            return;
                        }
                    }
                    '*' => {
                        if n_stack.len() < 3
                            || n_stack.pop_back().unwrap() != 'N'
                            || n_stack.pop_back().unwrap() != '*'
                            || n_stack.pop_back().unwrap() != 'N' {
                            println!("RE");
                            return;
                        }
                    }
                    'i' => {
                        if n_stack.len() < 1
                            || n_stack.pop_back().unwrap() != 'i' {
                            println!("RE");
                            return;
                        }
                    }
                    _ => {
                        println!("RE");
                        return;
                    }
                }

                n_stack.push_back('N');

                println!("R");

                if p == Priority::None && *token_stack.back().unwrap() != '#' {
                    println!("E");
                    return;
                }
            }
            if char == '#' {
                if !token_stack.is_empty() && n_stack.len() != 1 {
                    println!("RE");
                }
                return;
            }
            println!("I{}", char);
            n_stack.push_back(char);
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
            n_stack.push_back(char);
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

