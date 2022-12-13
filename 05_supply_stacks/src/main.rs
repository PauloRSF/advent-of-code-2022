use std::{collections::HashMap, error::Error, fs::read_to_string, str::FromStr};

type CrateStackId = String;
type CrateStack = Vec<char>;

#[derive(Debug, Clone)]
struct CrateStacks {
    state: HashMap<CrateStackId, CrateStack>,
    order: Vec<CrateStackId>,
}

impl FromStr for CrateStacks {
    type Err = Box<dyn Error>;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut lines = value.split('\n').collect::<Vec<_>>();

        let stacks_order = lines
            .pop()
            .unwrap()
            .trim()
            .split("   ")
            .map(String::from)
            .collect::<Vec<_>>();

        let number_of_stacks = stacks_order.len();

        let all_crates = lines
            .join(" ")
            .chars()
            .collect::<Vec<_>>()
            .chunks(4)
            .map(|crate_box| crate_box.get(1).copied().unwrap_or(' '))
            .collect::<Vec<char>>();

        let stacks_state = stacks_order
            .iter()
            .enumerate()
            .map(|(index, stack_id)| {
                (
                    stack_id.clone(),
                    all_crates
                        .iter()
                        .skip(index)
                        .step_by(number_of_stacks)
                        .rev()
                        .copied()
                        .filter(|&crate_marker| crate_marker != ' ')
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<HashMap<_, _>>();

        Ok(Self {
            order: stacks_order,
            state: stacks_state,
        })
    }
}

impl CrateStacks {
    fn get_target_crate_stacks(
        &self,
        instruction: &MoveInstruction,
    ) -> Result<(CrateStack, CrateStack), Box<dyn Error>> {
        let origin_stack = self
            .state
            .get(&instruction.origin_stack_id)
            .ok_or(format!(
                "Tried to move crate from stack {} to stack {} but the origin stack does not exist",
                instruction.origin_stack_id, instruction.destination_stack_id
            ))?
            .clone();

        let destination_stack = self
            .state
            .get(&instruction.destination_stack_id)
            .ok_or(format!(
                "Tried to move crate from stack {} to stack {} but the destination stack does not exist",
                instruction.origin_stack_id, instruction.destination_stack_id
            ))?
            .clone();

        Ok((origin_stack, destination_stack))
    }

    fn move_crates_with_cratemover_9000(
        &self,
        instruction: &MoveInstruction,
    ) -> Result<Self, Box<dyn Error>> {
        let (mut origin_stack, mut destination_stack) =
            self.get_target_crate_stacks(instruction)?;

        for _ in 0..instruction.amount {
            let crate_to_move = origin_stack.pop().ok_or(format!(
                "Tried to move crate from stack {}, but it has no crates",
                instruction.origin_stack_id
            ))?;

            destination_stack.push(crate_to_move);
        }

        let mut stacks_state = self.state.clone();

        stacks_state.insert(instruction.origin_stack_id.clone(), origin_stack);
        stacks_state.insert(instruction.destination_stack_id.clone(), destination_stack);

        Ok(CrateStacks {
            state: stacks_state,
            order: self.order.clone(),
        })
    }

    fn move_crates_with_cratemover_9001(
        &self,
        instruction: &MoveInstruction,
    ) -> Result<Self, Box<dyn Error>> {
        let (mut origin_stack, mut destination_stack) =
            self.get_target_crate_stacks(instruction)?;

        let crates_to_move =
            origin_stack.split_off(origin_stack.len() - instruction.amount as usize);

        destination_stack.extend_from_slice(&crates_to_move[..]);

        let mut stacks_state = self.state.clone();

        stacks_state.insert(instruction.origin_stack_id.clone(), origin_stack);
        stacks_state.insert(instruction.destination_stack_id.clone(), destination_stack);

        Ok(CrateStacks {
            state: stacks_state,
            order: self.order.clone(),
        })
    }
}

#[derive(Debug)]
struct MoveInstruction {
    amount: u8,
    origin_stack_id: CrateStackId,
    destination_stack_id: CrateStackId,
}

impl FromStr for MoveInstruction {
    type Err = Box<dyn Error>;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut segments = value.split(' ').skip(1);

        let amount = segments
            .next()
            .ok_or(format!("Could not get amount from instruction \"{}\"", value))?
            .parse::<u8>()
            .map_err(|_| format!("The amount of crates to move in an instruction should be an integer. Got: \"{}\"", value))?;

        segments.next();

        let origin_stack_id = segments
            .next()
            .ok_or(format!(
                "Could not get the origin stack index from instruction \"{}\"",
                value
            ))?
            .to_string();

        segments.next();

        let destination_stack_id = segments
            .next()
            .ok_or(format!(
                "Could not get the destination stack index from instruction \"{}\"",
                value
            ))?
            .to_string();

        Ok(Self {
            amount,
            origin_stack_id,
            destination_stack_id,
        })
    }
}

fn get_message_from_crate_stacks(crate_stacks: &CrateStacks) -> String {
    crate_stacks
        .order
        .iter()
        .map(|stack_id| {
            crate_stacks
                .state
                .get(stack_id)
                .expect(
                    "Crate stacks order vector should only contain existing stack IDs as elements",
                )
                .iter()
                .last()
        })
        .collect::<Option<String>>()
        .unwrap_or_default()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("./input.txt")?;

    let (initial_stacks, all_instructions) = input.trim_end().split_once("\n\n").ok_or("The input should contain the initial stacks stateuration and the moving instructions separated by a double newline")?;

    let crate_stacks = initial_stacks.parse::<CrateStacks>()?;

    let instructions = all_instructions
        .split('\n')
        .map(str::parse)
        .collect::<Result<Vec<MoveInstruction>, _>>()?;

    let cratemover_9000_rearranged_stacks = instructions
        .iter()
        .try_fold(crate_stacks.clone(), |stacks, instruction| {
            stacks.move_crates_with_cratemover_9000(instruction)
        })?;

    println!(
        "After reorganizing with the CrateMover 9000, the top crates give the message \"{}\"",
        get_message_from_crate_stacks(&cratemover_9000_rearranged_stacks)
    );

    let cratemover_9001_rearranged_stacks = instructions
        .iter()
        .try_fold(crate_stacks, |stacks, instruction| {
            stacks.move_crates_with_cratemover_9001(instruction)
        })?;

    println!(
        "After reorganizing with the CrateMover 9001, the top crates give the message \"{}\"",
        get_message_from_crate_stacks(&cratemover_9001_rearranged_stacks)
    );

    Ok(())
}
