use anyhow::{anyhow, Result};
use rand::Rng;
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq)]
enum Field {
    Term(i32),
    Operator(char),
}

pub struct Operation {
    pub to_string: String,
    pub result: String,

    operators: Vec<char>,
    min: i32,
    max: i32,
    nb_terms: u32,
}

impl Operation {
    pub fn new(options: String, mut nb_terms: u32, min: i32, mut max: i32) -> Self {
        if min > max {
            max = min + 5;
        }
        if nb_terms < 2 {
            nb_terms = 2;
        }

        let mut operators = Vec::new(); // In a vec to allow random choice
        if options.contains("A") {
            operators.push('+');
        }
        if options.contains("S") {
            operators.push('-');
        }
        if options.contains("M") || operators.is_empty() {
            operators.push('*');
        }

        Self {
            to_string: "0 + 0".to_string(),
            result: "0".to_string(),
            operators,
            min,
            max,
            nb_terms,
        }
    }

    /// Update the instance with a new operation
    pub fn generate(&mut self) {
        let nb_numbers = rand::thread_rng().gen_range(2..=self.nb_terms);

        for i in 0..nb_numbers {
            let num = rand::thread_rng().gen_range(self.min..=self.max);
            let op = self.operators[rand::thread_rng().gen_range(0..self.operators.len())];

            if i == 0 {
                self.to_string = format!("{}", num);
            } else {
                self.to_string = format!("{} {} {}", self.to_string, op, num);
            }
        }

        if let Ok(operations) = convert(self.to_string.as_str()) {
            self.result = resolve(operations).unwrap();
        }
    }
}

/// Clean spaces between operators
pub fn clean_operation(txt: &str) -> String {
    let p: String = txt.chars().filter(|c| !c.is_whitespace()).collect();

    let mut output = String::new();
    let mut previous = '+';

    for c in p.chars() {
        println!("-> {} -> {}", previous, c);
        if c == '+' || c == '*' || (c == '-' && previous.is_ascii_digit()) {
            output = format!("{} {} ", output, c)
        } else if c == '-' && previous.is_ascii_digit() {
            output = format!("{} {}", output, c)
        } else {
            output = format!("{}{}", output, c)
        }
        previous = c;
    }

    output
}
pub fn convert_and_resolve(txt: &str) -> Result<String> {
    resolve(convert(txt)?)
}

// --
fn convert(txt: &str) -> Result<VecDeque<Field>> {
    let mut operations: VecDeque<Field> = VecDeque::new();

    // Clean
    let txt: Vec<char> = txt
        .chars()
        .filter(|c| c.is_ascii_digit() || *c == '+' || *c == '-' || *c == '*')
        .collect();

    // Create a Field container
    let mut current = String::new();
    for (i, c) in txt.iter().enumerate() {
        if (!c.is_ascii_digit() && !current.is_empty()) || i == txt.len() - 1 {
            if i == txt.len() - 1 {
                current.push(*c);
            }

            let num = current.parse::<i32>()?;
            operations.push_back(Field::Term(num));
            current.clear();

            if *c == '+' || *c == '*' || *c == '-' {
                operations.push_back(Field::Operator(*c));
            }
        } else {
            current.push(*c);
        }
    }

    Ok(operations)
}

fn resolve(mut operations: VecDeque<Field>) -> Result<String> {
    while let Some(position) = operations.iter().position(|v| *v == Field::Operator('*')) {
        // Multiplication first --
        let b = match operations.remove(position + 1).unwrap() {
            Field::Term(v) => v,
            _ => return Err(anyhow!("Incorrect operation, expected a term")),
        };
        let a = match operations.remove(position - 1).unwrap() {
            Field::Term(v) => v,
            _ => return Err(anyhow!("Incorrect operation, expected a term")),
        };
        operations[position - 1] = Field::Term(a * b);
    }

    // Rest --
    while operations.len() > 1 {
        let a = match operations.pop_front().unwrap() {
            Field::Term(v) => v,
            _ => return Err(anyhow!("Incorrect operation, expected a term")),
        };
        let op = operations.pop_front().unwrap();
        let b = match operations.pop_front().unwrap() {
            Field::Term(v) => v,
            _ => return Err(anyhow!("Incorrect operation, expected a term")),
        };

        match op {
            Field::Operator(o) => operations.push_front(Field::Term(match o {
                '+' => a + b,
                _ => a - b,
            })),
            _ => return Err(anyhow!("Incorrect operation, expected an operator")),
        };
    }

    match operations.front().unwrap() {
        Field::Term(v) => Ok(v.to_string()),
        _ => return Err(anyhow!("Resolve failed")),
    }
}
