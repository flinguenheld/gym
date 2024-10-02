use rand::Rng;
use std::collections::VecDeque;

const ADD: i16 = i16::MAX;
const SUB: i16 = i16::MAX - 1;
const MUL: i16 = i16::MAX - 2;

#[derive(Clone, Copy)]
pub enum OperationType {
    Addition,
    Subtraction,
    Multiplication,
    All,
}

pub struct OperationNEWGEN {
    pub to_string: String,
    pub result: String,

    operation_type: OperationType,
    min: i16,
    max: i16,
    nb_terms: u16,
}

impl OperationNEWGEN {
    pub fn new(operation_type: OperationType, mut nb_terms: u16, min: i16, mut max: i16) -> Self {
        if min > max {
            max = min + 5;
        }
        if nb_terms < 2 {
            nb_terms = 2;
        }

        Self {
            to_string: "0 + 0".to_string(),
            result: "0".to_string(),
            operation_type,
            min,
            max,
            nb_terms,
        }
    }

    pub fn generate(&self) -> Self {
        let nb_numbers = rand::thread_rng().gen_range(2..self.nb_terms + 1);

        // Create --
        let mut operations = VecDeque::new();
        for i in 0..(nb_numbers * 2 - 1) {
            operations.push_back(if i % 2 == 0 {
                rand::thread_rng().gen_range(self.min..=self.max)
            } else {
                match self.operation_type {
                    OperationType::Addition => ADD,
                    OperationType::Subtraction => SUB,
                    OperationType::Multiplication => MUL,
                    OperationType::All => rand::thread_rng().gen_range(MUL..=ADD),
                }
            });
        }
        let value_str = to_string(&operations);

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

        let value_digit = *operations.front().unwrap();

        OperationNEWGEN {
            to_string: value_str,
            result: value_digit.to_string(),
            ..*self
        }
    }
}

fn to_string(numbers: &VecDeque<i16>) -> String {
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
