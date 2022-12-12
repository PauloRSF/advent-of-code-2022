use std::{cmp::Ordering, error::Error, fs::read_to_string};

struct SectionRange {
    start: u32,
    end: u32,
}

impl TryFrom<&str> for SectionRange {
    type Error = Box<dyn Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (start_section, end_section) = value
            .split_once('-')
            .ok_or(format!("A range should be dash separated. Got: {}", value))?;

        Ok(Self {
            start: start_section.parse::<u32>().map_err(|_| {
                format!(
                    "A range start value should be an integer. Got: {}",
                    start_section
                )
            })?,
            end: end_section.parse::<u32>().map_err(|_| {
                format!(
                    "A range end value should be an integer. Got: {}",
                    end_section
                )
            })?,
        })
    }
}

struct ElfPair {
    first_elf_range: SectionRange,
    second_elf_range: SectionRange,
}

impl TryFrom<&str> for ElfPair {
    type Error = Box<dyn Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (first_elf_range, second_elf_range) = value
            .split_once(',')
            .ok_or("An elf pair should be comma separated")?;

        Ok(Self {
            first_elf_range: SectionRange::try_from(first_elf_range)?,
            second_elf_range: SectionRange::try_from(second_elf_range)?,
        })
    }
}

impl ElfPair {
    fn has_redundant_range(&self) -> bool {
        let range_start_comparison = self.first_elf_range.start.cmp(&self.second_elf_range.start);
        let range_end_comparison = self.first_elf_range.end.cmp(&self.second_elf_range.end);

        matches!(
            (range_start_comparison, range_end_comparison),
            (Ordering::Greater, Ordering::Less)
                | (Ordering::Less, Ordering::Greater)
                | (Ordering::Greater, Ordering::Equal)
                | (Ordering::Equal, Ordering::Greater)
                | (Ordering::Equal, Ordering::Less)
                | (Ordering::Less, Ordering::Equal)
                | (Ordering::Equal, Ordering::Equal)
        )
    }

    fn has_overlapping_ranges(&self) -> bool {
        if self.first_elf_range.end < self.second_elf_range.start {
            return false;
        }

        if self.first_elf_range.start > self.second_elf_range.end {
            return false;
        }

        true
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("./input.txt")?;

    let pairs = input
        .trim()
        .split('\n')
        .map(ElfPair::try_from)
        .collect::<Result<Vec<_>, _>>()?;

    let amount_of_pairs_with_redundant_range = pairs
        .iter()
        .filter(|pair| pair.has_redundant_range())
        .count();

    println!(
        "There are {} elf pairs where one range fully contains the other",
        amount_of_pairs_with_redundant_range
    );

    let amount_of_pairs_with_overlapping_ranges = pairs
        .iter()
        .filter(|pair| pair.has_overlapping_ranges())
        .count();

    println!(
        "There are {} elf pairs that have overlapping ranges",
        amount_of_pairs_with_overlapping_ranges
    );

    Ok(())
}
