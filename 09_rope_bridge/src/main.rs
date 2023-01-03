use std::{collections::HashSet, error::Error, fs::read_to_string, str::FromStr};

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn step(&self, direction: &Direction) -> Point {
        match direction {
            Direction::Up => self.translated(0, -1),
            Direction::Down => self.translated(0, 1),
            Direction::Left => self.translated(-1, 0),
            Direction::Right => self.translated(1, 0),
        }
    }

    fn translated(&self, x: i32, y: i32) -> Self {
        Self {
            x: self.x + x,
            y: self.y + y,
        }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = Box<dyn Error>;

    fn from_str(direction_str: &str) -> Result<Self, Self::Err> {
        match direction_str {
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            _ => Err(format!("A direction should be U, D, L or R. Got: {}", direction_str).into()),
        }
    }
}

struct Motion {
    direction: Direction,
    amount: i32,
}

impl FromStr for Motion {
    type Err = Box<dyn Error>;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let (direction_str, amount_str) = value.split_once(' ').ok_or(format!(
            "A motion's direction and amount should be separated by a space. Got: {}",
            value
        ))?;

        let amount = amount_str
            .parse::<i32>()
            .map_err(|_| format!("A motion amount should be an integer. Got {}", amount_str))?;

        Ok(Self {
            direction: direction_str.parse()?,
            amount,
        })
    }
}

struct Rope {
    knots: Vec<Point>,
}

impl Rope {
    fn new(knot_count: usize, initial_position: &Point) -> Self {
        Self {
            knots: [*initial_position].repeat(knot_count),
        }
    }

    fn tail(&self) -> &Point {
        self.knots.last().unwrap()
    }

    fn move_rope_head(&mut self, direction: &Direction) {
        let head_knot_position = self.knots[0].step(direction);

        let mut new_knot_positions = vec![head_knot_position];
        let mut last_knot_moved = head_knot_position;

        for knot in self.knots[1..].iter() {
            last_knot_moved = Rope::get_new_knot_position(knot, &last_knot_moved);

            new_knot_positions.push(last_knot_moved);
        }

        self.knots = new_knot_positions;
    }

    fn get_new_knot_position(current_position: &Point, previous_knot_position: &Point) -> Point {
        match (
            previous_knot_position.x - current_position.x,
            previous_knot_position.y - current_position.y,
        ) {
            (x, y) if x.abs() <= 1 && y.abs() <= 1 => *current_position,
            (x, y) => Point {
                x: current_position.x + x.clamp(-1, 1),
                y: current_position.y + y.clamp(-1, 1),
            },
        }
    }
}

fn compute_unique_tail_positions_count(rope: &mut Rope, motions: &[Motion]) -> usize {
    let mut unique_tail_positions = HashSet::<Point>::new();

    for motion in motions {
        for _ in 0..motion.amount {
            rope.move_rope_head(&motion.direction);

            unique_tail_positions.insert(*rope.tail());
        }
    }

    unique_tail_positions.len()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("./input.txt")?;

    let motions = input
        .trim()
        .split('\n')
        .map(str::parse::<Motion>)
        .collect::<Result<Vec<_>, _>>()?;

    let mut two_knots_rope = Rope::new(2, &Point::default());

    println!(
        "Following the motions with 2 knots, the tail would end up in {} unique positions",
        compute_unique_tail_positions_count(&mut two_knots_rope, &motions)
    );

    let mut ten_knots_rope = Rope::new(10, &Point::default());

    println!(
        "Following the motions with 10 knots, the tail would end up in {} unique positions",
        compute_unique_tail_positions_count(&mut ten_knots_rope, &motions)
    );

    Ok(())
}
