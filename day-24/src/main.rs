#![allow(dead_code)]

use anyhow::{Result, anyhow};

use cli::{Part, get_part};

fn main() {
    match get_part("inputs/day-24.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{error:?}"),
    }
}

fn part_1(input: &str) -> Result<u64> {
    // This is one where I cannot solve and have to look for a solution.
    //
    // See https://github.com/mebeim/aoc/blob/master/2021/README.md#day-24---arithmetic-logic-unit.
    //
    // Each input block either results in:
    //   - z = z * 26 + w + constant; (this is the closest I've got) or
    //   - z = z / 26 if z % 26 - w + constant = 0
    // where constant is a value present in the input block.
    //
    // You can simply throw in some starting values through each input block to observe the outcome.
    // There are 7 of the first type, and 7 of the second type.
    //
    // So broadly speaking, z is used as a storage for the constant values modulo 26. In order for z
    // to be 0 at the end of the program, we must satisfy all
    // the second criteria above, thus defining the relationships between the values of w.
    //
    // The relative order of the first- and second-type input blocks, affects which digits are
    // related to each other, and what the constants in that relationship are:
    //      prev_w + prev_constant - curr_w + curr_constant = 0
    //   => prev_w + prev_constant + curr_constant = curr_w
    //   => store the relationship (prev_w, curr_w, (prev_constant + curr_constant))
    //
    // For max model number, we set either prev_w or w to 9, whichever yields larger digits.
    // For min model number, we set either prev_w or w to 1, whichever yields smaller digits.
    //
    // Note that (prev_constant + curr_constant) should be in the range -8 to 8.

    let max = max_model_number(&digits_relationships(input)?)?;

    let mut alu = Alu::try_new(&max.to_string(), input)?;

    while alu.step().is_ok() {}

    if alu.z() == 0 {
        Ok(max)
    } else {
        Err(anyhow!("Incorrect model number calculated: {}", max))
    }
}

fn part_2(input: &str) -> Result<u64> {
    let min = min_model_number(&digits_relationships(input)?)?;

    let mut alu = Alu::try_new(&min.to_string(), input)?;

    while alu.step().is_ok() {}

    if alu.z() == 0 {
        Ok(min)
    } else {
        Err(anyhow!("Incorrect model number calculated: {}", min))
    }
}

/// Determines the relationships between digits in the program.
///
/// Returns the set of relationships (prev_w, w, constant).
fn digits_relationships(program: &str) -> Result<Vec<(usize, usize, i32)>> {
    let mut first_type_constants: Vec<(usize, i32)> = Vec::new();

    let mut relationships = Vec::new();

    let lines = program.lines().collect::<Vec<_>>();
    if lines.len() != 14 * 18 {
        return Err(anyhow!("Unexpected program len: {}", program));
    }

    for w in 0..14 {
        let input_block = &lines[w * 18..(w + 1) * 18];
        if input_block[0] != "inp w" {
            return Err(anyhow!("Invalid input block: {:?}", input_block));
        }

        if input_block[4] == "div z 1" {
            // First type input block

            if !input_block[15].starts_with("add y ") {
                return Err(anyhow!(
                    "Unexpected first type input_block (cannot find constant)"
                ));
            }

            let [_, _, constant_part, ..] =
                input_block[15].split_ascii_whitespace().collect::<Vec<_>>()[..]
            else {
                return Err(anyhow!(
                    "Cannot get constant_part for first type input_block"
                ));
            };

            first_type_constants.push((w, constant_part.parse()?));
        } else {
            // Second type input block

            if !input_block[5].starts_with("add x") {
                return Err(anyhow!(
                    "Unexpected second type input_block (cannot find constant)"
                ));
            }

            let [_, _, constant_part, ..] =
                input_block[5].split_ascii_whitespace().collect::<Vec<_>>()[..]
            else {
                return Err(anyhow!(
                    "Cannot get constant_part for second type input_block"
                ));
            };

            let Some((stored_w, stored_constant)) = first_type_constants.pop() else {
                return Err(anyhow!("Popping an empty stack"));
            };

            relationships.push((stored_w, w, stored_constant + constant_part.parse::<i32>()?));
        }
    }

    assert_eq!(first_type_constants.len(), 0);
    assert_eq!(relationships.len(), 7);
    assert!(
        relationships
            .iter()
            .all(|relationship| relationship.2 > -9 && relationship.2 < 9)
    );

    Ok(relationships)
}

/// Finds the max model number given the relationships between digits.
fn max_model_number(digits_relationships: &[(usize, usize, i32)]) -> Result<u64> {
    // digits[prev_w] + constant = digits[w].
    // To maximize the model number, either prev_w or w should be 9.

    let mut digits = [0; 14];

    for relationship in digits_relationships {
        if relationship.2 > 0 {
            digits[relationship.1] = 9;
            digits[relationship.0] = 9 - relationship.2;
        } else {
            digits[relationship.0] = 9;
            digits[relationship.1] = 9 + relationship.2;
        }
    }

    if !digits.iter().all(|digit| *digit > 0 && *digit < 10) {
        Err(anyhow!("Invalid digit (out of range) after calculation"))
    } else {
        digits
            .iter()
            .map(|digit| *digit as u64)
            .reduce(|acc, digit| acc * 10 + digit)
            .ok_or(anyhow!("Cannot collapse digits into single value"))
    }
}

/// Finds the min model number given the relationships between digits.
fn min_model_number(digits_relationships: &[(usize, usize, i32)]) -> Result<u64> {
    // digits[prev_w] + constant = digits[w].
    // To minimize the model number, either prev_w or w should be 9.

    let mut digits = [0; 14];

    for relationship in digits_relationships {
        if relationship.2 > 0 {
            digits[relationship.0] = 1;
            digits[relationship.1] = 1 + relationship.2;
        } else {
            digits[relationship.1] = 1;
            digits[relationship.0] = 1 - relationship.2;
        }
    }

    if !digits.iter().all(|digit| *digit > 0 && *digit < 10) {
        Err(anyhow!("Invalid digit (out of range) after calculation"))
    } else {
        digits
            .iter()
            .map(|digit| *digit as u64)
            .reduce(|acc, digit| acc * 10 + digit)
            .ok_or(anyhow!("Cannot collapse digits into single value"))
    }
}

struct Alu {
    registers: [i32; 4],
    input: [u8; 14],
    program: Vec<String>,
    ip: usize,
    pc: usize,
}

impl Alu {
    fn try_new(model_number: &str, program: &str) -> Result<Self> {
        let model_number_vec = model_number
            .bytes()
            .map(|byte| byte - b'0')
            .collect::<Vec<_>>();
        if model_number_vec.len() != 14 {
            return Err(anyhow!(
                "Invalid model number (len must be 14): {}",
                model_number
            ));
        };
        model_number_vec.iter().try_for_each(|byte| {
            if *byte < 1 || *byte > 9 {
                return Err(anyhow!(
                    "Invalid model number (must be 1-9): {}",
                    model_number
                ));
            }

            Ok(())
        })?;

        let mut input = [0; 14];
        input.copy_from_slice(&model_number_vec);

        let program = program.lines().map(String::from).collect::<Vec<_>>();

        Ok(Alu {
            registers: [0; 4],
            input,
            program,
            ip: 0,
            pc: 0,
        })
    }

    /// Executes one instruction.
    fn step(&mut self) -> Result<()> {
        if self.pc >= self.program.len() {
            return Err(anyhow!("pc >= program len"));
        }

        match &self.program[self.pc]
            .split_ascii_whitespace()
            .collect::<Vec<_>>()[..]
        {
            ["inp", r, ..] if Self::is_register(r) => {
                if self.ip >= self.input.len() {
                    return Err(anyhow!("ip >= input len"));
                }

                let Some(register_index) = Self::register_index(r) else {
                    return Err(anyhow!("Cannot get index for register: {}", r));
                };

                self.registers[register_index] = self.input[self.ip] as i32;

                self.ip += 1;
            }
            ["add", r, a, ..] if Self::is_register(r) => {
                let Some(register_index) = Self::register_index(r) else {
                    return Err(anyhow!("Cannot get index for register: {}", r));
                };

                self.registers[register_index] +=
                    if let Some(argument_index) = Self::register_index(a) {
                        self.registers[argument_index]
                    } else {
                        a.parse::<i32>()?
                    };
            }
            ["mul", r, a, ..] if Self::is_register(r) => {
                let Some(register_index) = Self::register_index(r) else {
                    return Err(anyhow!("Cannot get index for register: {}", r));
                };

                self.registers[register_index] *=
                    if let Some(argument_index) = Self::register_index(a) {
                        self.registers[argument_index]
                    } else {
                        a.parse::<i32>()?
                    };
            }
            ["div", r, a, ..] if Self::is_register(r) => {
                let Some(register_index) = Self::register_index(r) else {
                    return Err(anyhow!("Cannot get index for register: {}", r));
                };

                self.registers[register_index] /=
                    if let Some(argument_index) = Self::register_index(a) {
                        self.registers[argument_index]
                    } else {
                        a.parse::<i32>()?
                    };
            }
            ["mod", r, a, ..] if Self::is_register(r) => {
                let Some(register_index) = Self::register_index(r) else {
                    return Err(anyhow!("Cannot get index for register: {}", r));
                };

                self.registers[register_index] %=
                    if let Some(argument_index) = Self::register_index(a) {
                        self.registers[argument_index]
                    } else {
                        a.parse::<i32>()?
                    };
            }
            ["eql", r, a, ..] if Self::is_register(r) => {
                let Some(register_index) = Self::register_index(r) else {
                    return Err(anyhow!("Cannot get index for register: {}", r));
                };

                let argument = if let Some(argument_index) = Self::register_index(a) {
                    self.registers[argument_index]
                } else {
                    a.parse::<i32>()?
                };

                self.registers[register_index] =
                    (self.registers[register_index] == argument) as i32;
            }

            _ => return Err(anyhow!("Unhandled instruction: {}", self.program[self.pc])),
        };

        self.pc += 1;

        Ok(())
    }

    /// Checks that the given symbol is a register name.
    fn is_register(symbol: &str) -> bool {
        matches!(symbol, "w" | "x" | "y" | "z")
    }

    /// Maps the given register name to an index into registers.
    fn register_index(symbol: &str) -> Option<usize> {
        match symbol {
            "w" => Some(0),
            "x" => Some(1),
            "y" => Some(2),
            "z" => Some(3),
            _ => None,
        }
    }

    /// Reads the value of register w.
    fn w(&self) -> i32 {
        self.registers[0]
    }

    /// Reads the value of register x.
    fn x(&self) -> i32 {
        self.registers[1]
    }

    /// Reads the value of register y.
    fn y(&self) -> i32 {
        self.registers[2]
    }

    /// Reads the value of register z.
    fn z(&self) -> i32 {
        self.registers[3]
    }

    /// Resets the [Alu] to initial state.
    fn reset(&mut self) {
        self.ip = 0;
        self.pc = 0;
        self.registers = [0, 0, 0, 0];
    }

    /// Runs the input block that corresponds to the digit at input_index.
    ///
    /// Uses z as the cumulated register z value before running the block. Runs the block for w in
    /// the range of 1 to 9.
    fn run_input_block(&mut self, input_index: usize, z: i32) -> Vec<i32> {
        let mut output = Vec::new();

        for w in 1..10 {
            self.pc = input_index * 18 + 1;
            self.registers = [w, 0, 0, z];

            for _ in 0..18 {
                let _ = self.step();
            }

            output.push(self.z());
        }

        output
    }
}
