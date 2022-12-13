use std::{collections::HashSet, error::Error, fs::read_to_string, slice::Windows};

fn find_first_marker_start_index(
    marker_windows: Windows<'_, char>,
    marker_size: usize,
) -> Option<usize> {
    marker_windows
        .enumerate()
        .map(|(index, window)| (index + marker_size, window))
        .find(|(_, window)| HashSet::<&char>::from_iter(window.iter()).len() == marker_size)
        .map(|(index, _)| index)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("./input.txt")?;

    let buffer_chars = input.chars().collect::<Vec<char>>();

    let packet_marker_size = 4;

    let packet_marker_windows = buffer_chars.windows(packet_marker_size);

    let first_packet_start_marker_index =
        find_first_marker_start_index(packet_marker_windows, packet_marker_size)
            .ok_or("Could not find a start-of-packet sequence")?;

    println!(
        "The first start-of-packet marker starts at character {}",
        first_packet_start_marker_index
    );

    let message_marker_size = 14;

    let message_marker_windows = buffer_chars.windows(message_marker_size);

    let first_message_start_marker_index =
        find_first_marker_start_index(message_marker_windows, message_marker_size)
            .ok_or("Could not find a start-of-message sequence")?;

    println!(
        "The first start-of-message marker starts at character {}",
        first_message_start_marker_index
    );

    Ok(())
}
