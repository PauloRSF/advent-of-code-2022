use std::{fs::read_to_string, panic};

fn main() {
    let input = read_to_string("./input.txt").expect("Input file should be readable");

    let calorie_count_by_elf = input.split("\n\n").map(|elf_calories| {
        elf_calories
            .trim()
            .split('\n')
            .map(|calories| {
                calories
                    .parse::<u32>()
                    .expect("Calories should be an integer")
            })
            .sum::<u32>()
    });

    let ordered_calorie_counts_by_elf = {
        let mut calories_list = calorie_count_by_elf.collect::<Vec<_>>();
        calories_list.sort_unstable();
        calories_list.reverse();
        calories_list
    };

    let biggest_calorie_count = ordered_calorie_counts_by_elf
        .get(0)
        .expect("Some elf should have the max calories");

    println!(
        "The elf carrying the most calories is carrying {} calories.",
        biggest_calorie_count
    );

    if ordered_calorie_counts_by_elf.len() < 3 {
        panic!("We should have at least 3 elves for the second part of the puzzle")
    }

    let combined_top_three_calorie_counts = ordered_calorie_counts_by_elf[..3].iter().sum::<u32>();

    println!(
        "The 3 elves carrying the most calories are carrying a combined amount of {} calories.",
        combined_top_three_calorie_counts
    );
}
