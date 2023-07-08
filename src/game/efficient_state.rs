use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
};

use fnv::FnvHashMap;
use smallvec::SmallVec;

use mill_playfield::{EfficientPlayField,FieldPos,MoveDirection,DirectionToCheck};

use super::PlayerColor;

/// Calculates the possible moves of color, the amount of moves wich lead to a mill for color
/// and the amount of stones of the other players color, which can be beaten
///
/// It is possibly used as a judging function for the player agent
//#[inline]
pub fn get_move_triple(pf: &mut EfficientPlayField, color: PlayerColor) -> (u32, u32, u32) {
    let mut moves_possible_counter: u32 = 0;
    let mut moves_into_mill_counter: u32 = 0;
    let mut stones_to_take_counter: u32 = 0;

    // Used for the extreme case when all stones of the opponent are in a mill
    let mut overall_stones_of_opposite_color_counter = 0;

    let (color_positions, not_color_positions) = get_positions(&pf, color);
    if color_positions.len() == 3 {
        moves_possible_counter = 3 * not_color_positions.len() as u32;
        stones_to_take_counter = not_color_positions.len() as u32;

        for position in not_color_positions {
            if 0 < pf.get_mill_count(
                position,
                DirectionToCheck::OnAndAcrossRings { player_color: (!color).into() },
            ) {
                stones_to_take_counter -= 1;
            }
        }

        if (color_positions[0].ring_index == color_positions[1].ring_index
            && color_positions[1].ring_index != color_positions[2].ring_index)
            || (color_positions[0].ring_index != color_positions[1].ring_index
                && color_positions[1].ring_index == color_positions[2].ring_index)
            || (color_positions[0].ring_index == color_positions[2].ring_index
                && color_positions[0].ring_index != color_positions[1].ring_index)
            || (color_positions[0].index == color_positions[1].index
                && color_positions[1].index != color_positions[2].index)
            || (color_positions[0].index != color_positions[1].index
                && color_positions[1].index == color_positions[2].index)
            || (color_positions[0].index == color_positions[2].index
                && color_positions[0].index != color_positions[1].index)
        {
            moves_into_mill_counter += 1;
        }
    } else {
        calculate_move_tupel(
            pf,
            color,
            &mut moves_possible_counter,
            &mut moves_into_mill_counter,
            &mut overall_stones_of_opposite_color_counter,
            &mut stones_to_take_counter,
        );

        if stones_to_take_counter == 0 {
            // All stones of the opposite color are in a mill:
            stones_to_take_counter = overall_stones_of_opposite_color_counter;
        }
    }

    (moves_possible_counter, moves_into_mill_counter, stones_to_take_counter)
}

fn calculate_move_tupel(
    pf: &mut EfficientPlayField,
    color: PlayerColor,
    moves_possible_counter: &mut u32,
    moves_into_mill_counter: &mut u32,
    overall_stones_of_opposite_color_counter: &mut u32,
    stones_to_take_counter: &mut u32,
) {
    for ring_index in 0..3 {
        for field_index in 0..8 {
            let current_field = FieldPos {ring_index, index: field_index};
            // Current field state sifted to the LSB
            let current_field_state = pf.get_field_state_at(current_field);

            // If the current field is empty, we wont make any adjustments to the return values
            if current_field_state == 0 {
                continue;
            }

            // If the current field == color && the on-ring neighbors are empty => movements into a mill possible
            if current_field_state == (<PlayerColor as Into<u16>>::into(color) << (field_index * 2)) {
                for (neighbor_index, neighbor_state) in pf.get_neighbor_field_states(current_field) {
                    if neighbor_state == 0 {
                        *moves_possible_counter += 1;

                        *moves_into_mill_counter += pf.simulate_move_get_mill_count(
                            current_field,
                            MoveDirection::OnRing { target_field_index: neighbor_index },
                            color.into(),
                        );
                    }
                }
                // Check for possible over-ring moves
                if (field_index % 2) == 0 {
                    let (a, b) = calculate_tupels_across_rings(pf, current_field, color);

                    *moves_possible_counter += a;
                    *moves_into_mill_counter += b;
                }
            }
            // The opposite colors amount of stones which can be taken should be counted, which is if the stone
            // Isn't inside a mill!
            else {
                *overall_stones_of_opposite_color_counter += 1;

                if pf.get_mill_count(
                    current_field,
                    DirectionToCheck::OnAndAcrossRings { player_color: (!color).into() },
                ) == 0
                {
                    *stones_to_take_counter += 1;
                }
            }
        }
    }
}

/// Calculates the moves possible and moves into a mill when a stones is movable across EfficientPlayField rings.
/// Returns (moves_possible_counter, moves_into_mill_counter)
fn calculate_tupels_across_rings(pf: &mut EfficientPlayField, field: FieldPos, color: PlayerColor) -> (u32, u32) {
    let mut moves_possible_counter = 0;
    let mut moves_into_mill_counter = 0;

    let (next_rings_field_state, previous_rings_field_state) = pf.get_neigbor_rings_field_states(field);


    match field.ring_index {
        // Inner Ring
        0 if next_rings_field_state == 0 => {
            moves_possible_counter += 1;

            moves_into_mill_counter += pf.simulate_move_get_mill_count(
                FieldPos { ring_index: 0, index: field.index },
                MoveDirection::AcrossRings { target_ring_index: 1 },
                color.into(),
            )
        }
        // Mid Ring
        1 => {
            if previous_rings_field_state == 0 {
                moves_possible_counter += 1;

                moves_into_mill_counter += pf.simulate_move_get_mill_count(
                    FieldPos { ring_index: 1, index: field.index },
                    MoveDirection::AcrossRings { target_ring_index: 0 },
                    color.into(),
                )
            }

            if next_rings_field_state == 0 {
                moves_possible_counter += 1;

                moves_into_mill_counter += pf.simulate_move_get_mill_count(
                    FieldPos { ring_index: 1, index: field.index },
                    MoveDirection::AcrossRings { target_ring_index: 2 },
                    color.into(),
                )
            }
        }
        // Outer Ring
        2 if previous_rings_field_state == 0 => {
            moves_possible_counter += 1;

            moves_into_mill_counter += pf.simulate_move_get_mill_count(
                FieldPos { ring_index: 2, index: field.index },
                MoveDirection::AcrossRings { target_ring_index: 1 },
                color.into(),
            )
        }
        _ => {}
    }

    (moves_possible_counter, moves_into_mill_counter)
}

fn get_positions(pf: &EfficientPlayField, color: PlayerColor) -> (SmallVec<[FieldPos; 9]>, SmallVec<[FieldPos; 9]>) {
    let mut color_positions = SmallVec::<[FieldPos; 9]>::default();
    let mut not_color_positions = SmallVec::<[FieldPos; 9]>::default();

    for ring_index in 0..3 {
        for field_index in 0..8 {
            let current_field = FieldPos{ring_index, index: field_index};
            let state = pf.get_field_state_at(current_field);

            if state == (<PlayerColor as Into<u16>>::into(color) << (field_index * 2)) {
                color_positions.push(current_field)
            } else if state == (<PlayerColor as Into<u16>>::into(!color) << (field_index * 2)) {
                not_color_positions.push(current_field )
            }
        }
    }

    (color_positions, not_color_positions)
}

fn process_input_fields_canonical() {
    let (reader, mut writer) = init_writer_reader("input_felder_4.txt");
    let mut output_map = FnvHashMap::<EfficientPlayField, usize>::default();

    for (line_index, line_content) in reader.lines().enumerate() {
        // Idk why but the reference output.txt starts counting on 1...
        let line_index = line_index + 1;

        let mut playfield = EfficientPlayField::from_coded(&line_content.unwrap());
        println!("{playfield}");
        let canonical_form = playfield.get_canon_form();
        println!("{canonical_form}");

        match output_map.get(&canonical_form) {
            Some(identical_canonical_index) => writeln!(writer, "{}", identical_canonical_index).unwrap(),
            None => {
                output_map.insert(canonical_form, line_index);
                writeln!(writer, "{}", line_index).unwrap();
            }
        }
    }
}

fn process_input_fields_tuple() {
    let (reader, mut writer) = init_writer_reader("input_felder_5.txt");

    for (line_index, line_content) in reader.lines().enumerate() {
        let line_content = line_content.unwrap();
        let mut playfield = EfficientPlayField::from_coded(&line_content);

        let (x, y, z) = get_move_triple(&mut playfield, PlayerColor::White);

        assert!({
            println!("Input {line_index}: {line_content}\n{playfield}");
            println!("Moves: {x}\nMoves->Mill: {y}\nTo Take: {z}");
            true
        });

        writeln!(writer, "{x} {y} {z}").unwrap();
    }
}

/// Inits the reader and writer on the default files `input_felder.txt` and `output.txt`
fn init_writer_reader(input: &str) -> (BufReader<File>, BufWriter<File>) {
    let input_felder_txt =
        File::open(input).expect("The 'input_felder.txt' file was not found in the projects root...");
    let reader = BufReader::new(input_felder_txt);

    let output_text = File::create("output.txt").expect("Could not create ro 'output.txt' to write results into");
    let writer = BufWriter::new(output_text);

    (reader, writer)
}

#[cfg(test)]
mod tests {
    use super::EfficientPlayField;

    #[test]
    fn assignment4() {
        super::process_input_fields_canonical();
    }

    #[test]
    fn assignment5() {
        super::process_input_fields_tuple();
    }

    #[test]
    fn assignment5_dbg() {
        //let mut test_epf = EfficientPlayField::from_coded("BEEEWEWBEEWWEEWEWEEWWWBB");
        let mut test_epf = EfficientPlayField::from_coded("EWWBBEEEEEWBBEEEEEEEBEEB");
        println!("{test_epf}");

        let (x, y, z) = test_epf.get_move_triple(crate::game::PlayerColor::White);
        println!("{x} {y} {z}")
    }
}
