use std::{any::Any, collections::HashMap, fmt::Debug, fs::read_to_string, rc::Rc};

trait Source: Any + Debug {
    fn evaluate(&self) -> bool;
    fn name(&self) -> &str;
    fn as_any(&self) -> &dyn Any;
}

#[derive(Clone, PartialEq, Debug)]
enum Operation {
    XOR,
    OR,
    AND,
}

#[derive(Debug)]
struct Gate {
    name: String,
    operation: Operation,
    left: Rc<dyn Source>,
    right: Rc<dyn Source>,
}

impl Source for Gate {
    fn evaluate(&self) -> bool {
        let left = self.left.evaluate();
        let right = self.right.evaluate();
        let result = match self.operation {
            Operation::XOR => left ^ right,
            Operation::OR => left || right,
            Operation::AND => left && right,
        };
        result
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug)]
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

    fn as_any(&self) -> &dyn Any {
        self
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

#[derive(Debug)]
enum FullAdderGates {
    FirstHalfAdderXor,
    FirstHalfAdderAnd,
    SecondHalfAdderXor,
    SecondHalfAdderAnd,
    OrCarry,
}

#[derive(Debug)]
struct CircuitError {
    bit: usize,
    gate: FullAdderGates,
    gate_name: String,
    explenation: String,
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

    fn build_circuit(&self) -> (Vec<Rc<dyn Source>>, HashMap<String, Rc<dyn Source>>) {
        let mut gates_and_wires = HashMap::new();
        let ending_wires = self
            .ending_wires()
            .iter()
            .map(|wire| self.build(wire, &mut gates_and_wires))
            .collect();
        (ending_wires, gates_and_wires)
    }

    fn build(&self, name: &str, cache: &mut HashMap<String, Rc<dyn Source>>) -> Rc<dyn Source> {
        if cache.contains_key(name) {
            return Rc::clone(&cache[name]);
        }
        let source: Rc<dyn Source> = match &self.registry[name] {
            CircuitElement::Gate {
                operation,
                left,
                right,
            } => Rc::new(Gate {
                name: name.to_string(),
                operation: operation.clone(),
                left: self.build(left, cache),
                right: self.build(right, cache),
            }),
            CircuitElement::Wire { value } => Rc::new(Input {
                name: name.to_string(),
                value: *value,
            }),
        };
        cache.insert(name.to_string(), Rc::clone(&source));
        source
    }

    fn ending_wires(&self) -> Vec<&str> {
        let mut ending_wires = self
            .registry
            .keys()
            .filter(|name| name.starts_with("z"))
            .map(|name| name.as_str())
            .collect::<Vec<&str>>();

        ending_wires.sort_by(|left, right| {
            let left = left.replace("z", "").parse::<u8>().unwrap();
            let right = right.replace("z", "").parse::<u8>().unwrap();
            left.cmp(&right)
        });

        ending_wires
    }

    fn combine_bits(&self) -> u64 {
        let (ending_wires, _) = self.build_circuit();

        let sum = ending_wires
            .into_iter()
            .map(|wire| wire.evaluate())
            .enumerate()
            .map(|(bit, resut)| (resut as u64) << (bit as u64))
            .sum::<u64>();
        sum
    }

    fn validate_addition_mistakes(&self) -> Option<CircuitError> {
        let mut known_carries: HashMap<usize, String> = HashMap::new();
        let (ending_wires, gates) = self.build_circuit();

        let find_gate_by_child_and_type = |child: &str, gate_type| {
            gates
                .iter()
                .filter(|(_, gate)| match gate.as_any().downcast_ref::<Gate>() {
                    Some(gate) => {
                        gate.operation == gate_type
                            && (gate.left.name() == child || gate.right.name() == child)
                    }
                    None => false,
                })
                .map(|(_, v)| v.as_any().downcast_ref::<Gate>().unwrap())
                .next()
        };

        for (i, gate) in ending_wires.into_iter().enumerate() {
            let gate = gate.as_any().downcast_ref::<Gate>().unwrap();
            if i == 0 {
                if gate.operation != Operation::XOR {
                    return Some(CircuitError {
                        bit: i,
                        gate: FullAdderGates::FirstHalfAdderXor,
                        gate_name: gate.name.clone(),
                        explenation: "Gate with input wires in half adders is not a XOR"
                            .to_string(),
                    });
                }
                let and_gate =
                    find_gate_by_child_and_type(&gate.left.name(), Operation::AND).unwrap();
                known_carries.insert(i, and_gate.name.clone());
            } else if i == 45 {
                // unimplemented!()
            } else {
                if gate.operation != Operation::XOR {
                    return Some(CircuitError {
                        bit: i,
                        gate: FullAdderGates::SecondHalfAdderXor,
                        gate_name: gate.name.clone(),
                        explenation: "Wire outputting Z signal should be a XOR".to_string(),
                    });
                }

                let x_wire_name = format!("x{i:0>2}");
                let xor_first_gate =
                    find_gate_by_child_and_type(&x_wire_name, Operation::XOR).unwrap();

                if gate.left.name() != xor_first_gate.name
                    && gate.right.name() != xor_first_gate.name
                {
                    return Some(CircuitError {
                        bit: i,
                        gate: FullAdderGates::FirstHalfAdderXor,
                        gate_name: xor_first_gate.name.clone(),
                        explenation:
                            "Wire outputting Z signal is not connected to XOR with x and y bits"
                                .to_string(),
                    });
                }
            }
        }
        None
    }
}

fn puzzle_1(registry: &CircuitRegistry) {
    let result = registry.combine_bits();
    println!("Combining all bits gives a result {result}");
}

/// This solution was created semi-automatically.
/// When this method panics, it prints the incorrect gate. Than I manually searched the place
/// where it should belong, improved the input file and rerun the program as long as entire validatoin passed.
fn puzzle_2(registry: &CircuitRegistry) {
    if let Some(error) = registry.validate_addition_mistakes() {
        panic!("There was an error {:?}", error);
    }
    let mut erronous = vec!["dbp", "fdv", "z15", "ckj", "z23", "kdf", "z39", "rpp"];
    erronous.sort();
    println!("List of miswires gates: {}", erronous.join(","));
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

    // This iterator was created when I tried to bruteforece the solution.
    // Iterating over all possible pair would take forever, so the idea was abandoned.
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
