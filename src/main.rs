use coordination::GameCoordinator;
use mill::game::efficient_state::win_decider::compare_to_reference_data;

mod coordination;

fn main() {
    let mut coordinator = GameCoordinator::setup();
    coordinator.start_game();

    // For release testing of assignment 6:
    //compare_to_reference_data("input_felder_5vs5.txt", "output_5vs5.txt", 5);
}
