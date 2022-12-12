use std::{collections::HashSet, error::Error, fs::read_to_string};

#[derive(Debug)]
struct Rucksack {
    left: HashSet<char>,
    right: HashSet<char>,
}

impl Rucksack {
    fn unique_items(&self) -> HashSet<char> {
        self.left.union(&self.right).copied().collect()
    }

    fn common_items_in_compartments(&self) -> HashSet<&char> {
        self.left.intersection(&self.right).collect()
    }
}

impl TryFrom<&str> for Rucksack {
    type Error = String;

    fn try_from(contents: &str) -> Result<Self, Self::Error> {
        match contents.len() % 2 {
            0 => {
                let (left_contents, right_contents) = contents.split_at(contents.len() / 2);

                Ok(Self {
                    left: HashSet::from_iter(left_contents.chars()),
                    right: HashSet::from_iter(right_contents.chars()),
                })
            }
            _ => Err(format!(
                "A rucksack should have the same number of elements on both compartments. Got: {}",
                contents
            )),
        }
    }
}

fn get_item_priority(item_char: &char) -> u32 {
    match item_char {
        'A'..='Z' => *item_char as u32 - 38,
        'a'..='z' => *item_char as u32 - 96,
        _ => 0,
    }
}

fn get_elf_group_badge_item(rucksacks: &[Rucksack]) -> Option<char> {
    rucksacks
        .iter()
        .map(|rucksack| rucksack.unique_items())
        .reduce(|common_items, rucksack_items| {
            common_items
                .intersection(&rucksack_items)
                .copied()
                .collect::<HashSet<_>>()
        })
        .expect("Should provide at least one rucksack to get the badge item")
        .iter()
        .last()
        .copied()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("./input.txt")?;

    let rucksacks = input
        .trim()
        .split('\n')
        .map(Rucksack::try_from)
        .collect::<Result<Vec<_>, _>>()?;

    let repeated_items_total_priority: u32 = rucksacks
        .iter()
        .filter_map(|rucksack| {
            rucksack
                .common_items_in_compartments()
                .iter()
                .last()
                .copied()
        })
        .map(get_item_priority)
        .sum();

    println!(
        "The sum of the repeated item's priority for all rucksacks is {:?}",
        repeated_items_total_priority
    );

    let badge_items_total_priority: u32 = rucksacks
        .chunks(3)
        .filter_map(get_elf_group_badge_item)
        .map(|v| get_item_priority(&v))
        .sum();

    println!(
        "The sum of all badge items' priorities is {}",
        badge_items_total_priority
    );

    Ok(())
}
