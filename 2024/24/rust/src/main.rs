use std::{collections::HashMap, fs::read_to_string, rc::Rc};

struct MultiIndexCounter {
    inner: Vec<usize>,
    size: usize,
    overflow: bool,
    first_call: bool,
}

impl MultiIndexCounter {
    fn new(pairs: usize, size: usize) -> Self {
        MultiIndexCounter {
            inner: vec![0; pairs * 2],
            size,
            overflow: false,
            first_call: true,
        }
    }

    fn next(&mut self) -> Option<&Vec<usize>> {
        if self.first_call {
            self.first_call = false;
            return Some(&self.inner);
        }
        if self.overflow {
            return None;
        }
        for i in 0..self.inner.len() {
            if self.inner[i] + 1 < self.size {
                self.inner[i] += 1;
                break;
            }
            self.inner[i] = 0;
            if i + 1 == self.inner.len() {
                self.overflow = true;
            }
        }
        if self.overflow {
            return None;
        }
        Some(&self.inner)
    }
}

trait Source {
    fn evaluate(&mut self) -> bool;
    fn name(&self) -> &str;
}

#[derive(Clone)]
enum Operation {
    XOR,
    OR,
    AND,
}

struct Gate {
    name: String,
    operation: Operation,
    cache: Option<bool>,
    left: Rc<dyn Source>,
    right: Rc<dyn Source>,
}

impl Source for Gate {
    fn evaluate(&mut self) -> bool {
        match self.cache {
            Some(value) => value,
            None => {
                let left = self.left.evaluate();
                let right = self.right.evaluate();
                let result = match self.operation {
                    Operation::XOR => left ^ right,
                    Operation::OR => left || right,
                    Operation::AND => left && right,
                };
                self.cache = Some(result);
                result
            }
        }
    }

    fn name(&self) -> &str {
        &self.name
    }
}

struct Input {
    name: String,
    value: bool,
}

impl Source for Input {
    fn evaluate(&self) -> bool {
        self.value
    }

    fn name(&self) -> &str {
        &self.name
    }
}

enum CircuitElement {
    Gate {
        operation: Operation,
        left: String,
        right: String,
    },
    Wire {
        value: bool,
    },
}

struct CircuitRegistry {
    registry: HashMap<String, CircuitElement>,
}

impl CircuitRegistry {
    fn new() -> Self {
        CircuitRegistry {
            registry: HashMap::new(),
        }
    }
    fn add_wire(&mut self, name: String, state: &str) {
        fn to_bool(input: &str) -> bool {
            match input.parse::<u8>() {
                Ok(0u8) => false,
                Ok(1u8) => true,
                _ => panic!("Unexpected input {input}"),
            }
        }
        self.registry.insert(
            name,
            CircuitElement::Wire {
                value: to_bool(state),
            },
        );
    }

    fn add_gate(&mut self, name: String, operation: &str, left: String, right: String) {
        self.registry.insert(
            name,
            CircuitElement::Gate {
                operation: match operation {
                    "XOR" => Operation::XOR,
                    "AND" => Operation::AND,
                    "OR" => Operation::OR,
                    _ => panic!("Unexpected operation {operation}"),
                },
                left,
                right,
            },
        );
    }

    fn build(&self, name: &str) -> Box<dyn Source> {
        match &self.registry[name] {
            CircuitElement::Gate {
                operation,
                left,
                right,
            } => Box::new(Gate {
                name: name.to_string(),
                operation: operation.clone(),
                left: self.build(left),
                right: self.build(right),
            }),
            CircuitElement::Wire { value } => Box::new(Input {
                name: name.to_string(),
                value: *value,
            }),
        }
    }

    fn combine_bits(&self) -> u64 {
        let mut ending_wires = self
            .registry
            .keys()
            .filter(|name| name.starts_with("z"))
            .collect::<Vec<&String>>();

        ending_wires.sort_by(|left, right| {
            let left = left.replace("z", "").parse::<u8>().unwrap();
            let right = right.replace("z", "").parse::<u8>().unwrap();
            left.cmp(&right)
        });
        let sum = ending_wires
            .iter()
            .map(|ending_wire| self.build(&ending_wire).evaluate())
            .enumerate()
            .map(|(bit, resut)| (resut as u64) << (bit as u64))
            .sum::<u64>();
        sum
    }

    fn find_addition_mistakes(&self) -> Vec<String> {
        todo!()
    }
}

fn puzzle_1(registry: &CircuitRegistry) {
    let result = registry.combine_bits();
    println!("Combining all bits give a result {result}");
}

fn puzzle_2(registry: &CircuitRegistry) {
    let erronous = registry.find_addition_mistakes();
}

fn main() {
    let file_content = read_to_string("input.txt").unwrap();
    let mut input_wires = true;
    let mut registry = CircuitRegistry::new();
    file_content.lines().for_each(|line| {
        if line.is_empty() {
            input_wires = false;
            return;
        }
        if input_wires {
            let (name, state) = line.split_once(": ").unwrap();
            registry.add_wire(name.to_string(), state);
        } else {
            let (expression, name) = line.split_once("->").unwrap();
            let mut expression_parts = expression.trim().split(" ");
            let left = expression_parts.next().unwrap();
            let operator = expression_parts.next().unwrap();
            let right = expression_parts.next().unwrap();
            registry.add_gate(
                name.trim().to_string(),
                operator,
                left.to_string(),
                right.to_string(),
            );
        }
    });
    puzzle_1(&registry);
    puzzle_2(&registry);
}

#[cfg(test)]
mod test {
    use crate::MultiIndexCounter;

    #[test]
    fn should_iterate_over_multiple_indexes() {
        let mut iterator = MultiIndexCounter::new(1, 3);

        assert_eq!(iterator.next(), Some(&vec![0, 0]));
        assert_eq!(iterator.next(), Some(&vec![1, 0]));
        assert_eq!(iterator.next(), Some(&vec![2, 0]));

        assert_eq!(iterator.next(), Some(&vec![0, 1]));
        assert_eq!(iterator.next(), Some(&vec![1, 1]));
        assert_eq!(iterator.next(), Some(&vec![2, 1]));

        assert_eq!(iterator.next(), Some(&vec![0, 2]));
        assert_eq!(iterator.next(), Some(&vec![1, 2]));
        assert_eq!(iterator.next(), Some(&vec![2, 2]));

        assert_eq!(iterator.next(), None);
    }
}
