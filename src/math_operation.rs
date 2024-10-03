use rand::Rng;
use std::collections::VecDeque;

pub const ADD: i32 = i32::MAX;
pub const SUB: i32 = i32::MAX - 1;
pub const MUL: i32 = i32::MAX - 2;

pub struct Operation {
    pub to_string: String,
    pub result: String,

    operation_type: Vec<i32>,
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
            operators.push(ADD);
        }
        if options.contains("S") {
            operators.push(SUB);
        }
        if options.contains("M") || operators.is_empty() {
            operators.push(MUL);
        }

        Self {
            to_string: "0 + 0".to_string(),
            result: "0".to_string(),
            operation_type: operators,
            min,
            max,
            nb_terms,
        }
    }

    /// Update the instance with a new operation
    pub fn generate(&mut self) {
        let nb_numbers = rand::thread_rng().gen_range(2..self.nb_terms + 1);

        // Create --
        let mut operations = VecDeque::new();
        for i in 0..(nb_numbers * 2 - 1) {
            operations.push_back(if i % 2 == 0 {
                rand::thread_rng().gen_range(self.min..=self.max)
            } else {
                *self
                    .operation_type
                    .get(rand::thread_rng().gen_range(0..self.operation_type.len()))
                    .unwrap_or(&ADD)
            });
        }
        self.to_string = to_string(&operations);

        // Resolve
        while let Some(position) = operations.iter().position(|v| *v == MUL) {
            let b = operations.remove(position + 1).unwrap();
            let a = operations.remove(position - 1).unwrap();

            operations[position - 1] = a * b;
        }

        while operations.len() > 1 {
            let a = operations.pop_front().unwrap();
            let op = operations.pop_front().unwrap();
            let b = operations.pop_front().unwrap();

            operations.push_front(match op {
                ADD => a + b,
                _ => a - b,
            });
        }

        self.result = (*operations.front().unwrap()).to_string();
    }
}

fn to_string(numbers: &VecDeque<i32>) -> String {
    numbers
        .iter()
        .enumerate()
        .map(|(i, v)| {
            if i % 2 == 0 {
                v.to_string()
            } else {
                match *v {
                    ADD => " + ".to_string(),
                    SUB => " - ".to_string(),
                    MUL => " * ".to_string(),
                    _ => "Error ".to_string(),
                }
            }
        })
        .collect()
}
