use anyhow::{anyhow, Ok, Result};
use rand::Rng;
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq)]
enum Field {
    Term(i64),
    Operator(char),
}

pub struct Operation {
    pub to_string: String,
    pub result: String,
    operators: Vec<char>,
    min: i64,
    max: i64,
    nb_terms: u32,
    cons_div: u8,
}

impl Operation {
    pub fn new(options: String, nb_terms: u32, min: i64, max: i64, cons_div: bool) -> Option<Self> {
        let mut operators = Vec::new(); // In a vec to allow random choice
        if options.contains("D") {
            operators.push('/');
        }
        if options.contains("A") {
            operators.push('+');
        }
        if options.contains("S") {
            operators.push('-');
        }
        if options.contains("M") {
            operators.push('*');
        }

        if operators.is_empty() {
            None
        } else {
            Some(Self {
                to_string: "0 + 0".to_string(),
                result: "0".to_string(),
                operators,
                min,
                max: if min > max { min + 5 } else { max },
                nb_terms: match nb_terms {
                    _ if nb_terms < 2 => 2,
                    _ if nb_terms > 30 => 30,
                    _ => nb_terms,
                },
                cons_div: match cons_div {
                    false => 1,
                    true => 2,
                },
            })
        }
    }

    /// Update the instance with a new operation
    pub fn generate(&mut self) -> Result<()> {
        let nb_numbers = rand::thread_rng().gen_range(2..=self.nb_terms);

        let (mut previous, mut div_count) = (1, 0);
        for i in 0..nb_numbers {
            let op = if div_count == self.cons_div {
                div_count = 0;
                previous = 1;
                if self.operators.len() > 1 {
                    self.operators[rand::thread_rng().gen_range(1..self.operators.len())]
                } else {
                    ['+', '-', '*'][rand::thread_rng().gen_range(0..3)]
                }
            } else {
                self.operators[rand::thread_rng().gen_range(0..self.operators.len())]
            };

            let num = if op == '/' {
                calculate(previous, rand::thread_rng().gen_range(2..=12), '*')?
            } else {
                rand::thread_rng().gen_range(self.min..=self.max)
            };

            // Prevent more than 'cons_div' consecutives divisions
            if op == '/' {
                div_count += 1;
                previous = calculate(previous, num, '*')?;
            } else {
                previous = num;
            }

            // Add them from the back for divisions
            if i == 0 {
                self.to_string = format!("{}", num);
            } else {
                self.to_string = format!("{} {} {}", num, op, self.to_string);
            }
        }

        let operations = convert(self.to_string.as_str())?;
        self.result = resolve(operations)?;

        Ok(())
    }
}

/// Clean spaces between operators
pub fn clean_operation(txt: &str) -> String {
    let p: String = txt.chars().filter(|c| !c.is_whitespace()).collect();

    let mut output = String::new();
    let mut previous = '+';

    for c in p.chars() {
        println!("-> {} -> {}", previous, c);
        if c == '+' || c == '*' || c == '/' || (c == '-' && previous.is_ascii_digit()) {
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
        .filter(|c| c.is_ascii_digit() || *c == '+' || *c == '-' || *c == '*' || *c == '/')
        .collect();

    // Create a Field container
    let mut current = String::new();
    for (i, c) in txt.iter().enumerate() {
        if (!c.is_ascii_digit() && !current.is_empty()) || i == txt.len() - 1 {
            if i == txt.len() - 1 {
                current.push(*c);
            }

            let num = current.parse::<i64>()?;
            operations.push_back(Field::Term(num));
            current.clear();

            if *c == '+' || *c == '*' || *c == '-' || *c == '/' {
                operations.push_back(Field::Operator(*c));
            }
        } else {
            current.push(*c);
        }
    }

    Ok(operations)
}

fn resolve(mut operations: VecDeque<Field>) -> Result<String> {
    if !operations.is_empty() {
        while let Some(position) = operations
            .iter()
            .position(|v| *v == Field::Operator('*') || *v == Field::Operator('/'))
        {
            // Multiplication first --
            let b = extract_term(&operations.remove(position + 1).unwrap())?;
            let a = extract_term(&operations.remove(position - 1).unwrap())?;

            if let Some(field) = operations.get_mut(position - 1) {
                let op = extract_operator(field)?;
                *field = Field::Term(calculate(a, b, op)?);
            }
        }

        // Rest --
        while operations.len() > 1 {
            let a = extract_term(&operations.pop_front().unwrap())?;
            let op = extract_operator(&operations.pop_front().unwrap())?;
            let b = extract_term(&operations.pop_front().unwrap())?;

            operations.push_front(Field::Term(calculate(a, b, op)?));
        }

        Ok(extract_term(operations.front().unwrap())?.to_string())
    } else {
        Err(anyhow!("Resolve - Empty"))
    }
}

fn extract_term(field: &Field) -> Result<i64> {
    match field {
        Field::Term(v) => Ok(*v),
        _ => Err(anyhow!("Incorrect operation, expected a term")),
    }
}
fn extract_operator(field: &Field) -> Result<char> {
    match field {
        Field::Operator(op) => Ok(*op),
        _ => Err(anyhow!("Incorrect operation, expected an operator")),
    }
}
fn calculate(a: i64, b: i64, operator: char) -> Result<i64> {
    if let Some(val) = match operator {
        '/' => a.checked_div(b),
        '*' => a.checked_mul(b),
        '-' => a.checked_sub(b),
        _ => a.checked_add(b),
    } {
        Ok(val)
    } else {
        Err(anyhow!("Overflow !"))
    }
}
