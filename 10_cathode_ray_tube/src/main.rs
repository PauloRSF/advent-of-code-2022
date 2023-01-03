use std::{collections::HashSet, error::Error, fmt, fs::read_to_string, str::FromStr};

#[derive(Debug)]
enum Instruction {
    Addx(i32),
    Noop,
}

impl FromStr for Instruction {
    type Err = Box<dyn Error>;

    fn from_str(instruction_str: &str) -> Result<Self, Self::Err> {
        match instruction_str {
            "noop" => Ok(Self::Noop),
            instruction if instruction.starts_with("addx") => {
                let (_, value_str) = instruction.split_once(' ').ok_or(format!(
                    "An ADDX instruction should have a value. Got: {}",
                    instruction
                ))?;

                let value = value_str.parse::<i32>().map_err(|_| {
                    format!(
                        "An ADDX instruction value should be an integer. Got: {}",
                        instruction
                    )
                })?;

                Ok(Instruction::Addx(value))
            }
            _ => Err(format!("Invalid instruction: {}", instruction_str).into()),
        }
    }
}

#[derive(Hash, PartialEq, Eq)]
struct Point {
    x: u16,
    y: u16,
}

#[derive(Default)]
struct Crt {
    lit_pixels: HashSet<Point>,
}

impl fmt::Display for Crt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let screen = (0..6)
            .map(|y| {
                (0..40)
                    .map(|x| {
                        if self.lit_pixels.contains(&Point { x, y }) {
                            '#'
                        } else {
                            '.'
                        }
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");

        write!(f, "{}", screen)
    }
}

impl Crt {
    fn draw_pixel(&mut self, cycle: u16, sprite_position: i32) {
        let current_pixel = Point {
            x: cycle % 40,
            y: (cycle as f32 / 40_f32).floor() as u16,
        };

        if sprite_position < 0 {
            return;
        }

        let sprite_range = match sprite_position {
            0 => 0..2,
            39 => 38..40,
            x => x - 1..x + 2,
        };

        if sprite_range.contains(&(current_pixel.x as i32)) {
            self.lit_pixels.insert(current_pixel);
        }
    }
}

struct Cpu {
    x: i32,
    crt: Crt,
    current_cycle: u16,
    signal_strengths: Vec<i32>,
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            x: 1,
            current_cycle: 0,
            crt: Crt::default(),
            signal_strengths: vec![],
        }
    }
}

impl Cpu {
    fn tick(&mut self) {
        self.crt.draw_pixel(self.current_cycle, self.x);

        self.current_cycle += 1;

        let next_signal_saving_cycle = self.signal_strengths.len() as u16 * 40 + 20;

        if self.current_cycle == next_signal_saving_cycle {
            self.signal_strengths
                .push(self.x * self.current_cycle as i32);
        }
    }

    fn run(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Noop => self.tick(),
            Instruction::Addx(value) => {
                self.tick();
                self.tick();
                self.x += value;
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("./input.txt")?;

    let instructions = input
        .trim()
        .split('\n')
        .map(str::parse::<Instruction>)
        .collect::<Result<Vec<_>, _>>()?;

    let mut cpu = Cpu::default();

    for instruction in instructions {
        cpu.run(&instruction)
    }

    println!(
        "The product of all saved signal strenghts is {}",
        cpu.signal_strengths.iter().sum::<i32>(),
    );

    println!("\nFinal image produced by the CRT:\n\n{}", cpu.crt);

    Ok(())
}
