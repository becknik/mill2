# Mill in Rust

This is a [mill](https://en.wikipedia.org/wiki/Nine_men%27s_morris) implementation in Rust for the "Programmierprojekt: Mühlespiel in Rust" course in the University of Stuttgart in the summer semester of 2023.

The course is held by the FMI [FMI](https://fmi.uni-stuttgart.de/ti/teaching/s23/progproj/) and is initially taking place this semester.

Due to git LFS destroying the last repo, we had to make a new one... The old repo with its lastest branch is archived [here](https://github.com/becknik/mill/tree/who_won).

## Open TODOs

Besides the TODOs in the programs text, the following parts/ rules of the game are atm not fully implemented yet:

Rules:

- If a player can't move stones any more, he has lost the party
- If a player only has stones in a closed mill, a stone can be beaten out of one closed mill

## Assignments

### Assignment 3

Just `cargo run` it :^)

### Assignment 4

Execution:

```bash
cd mill2
cargo test -- assignment4
diff output.txt ../blatt_4_test_data_large/output.txt
```

Example for `input_felder.txt` © FMI Uni Stuttgart:
```
BBEEEEEBEEEEWEWWBWWEEEBE
BBEEEWEBBEWEBEEEEEEEEEEE
BEEEWWBEWEWEEEEWEEEEEBBB
BWEWEEWEBEBBEBWEWEEBEWWB
EBBBEEEWEEBEWEBEEEEEEEEE
EBEEWBWWEBBEBEWBEWEWBEWE
EEBEBWWEWEWWEEEEEEEBBBEE

```

### Assignment 5

```bash
cd mill2
cargo test -- assignment5 --nocapture
```

Or do a `cargo bench` & take a look into the `perf-opti` branch to see my waste of time due two stupid bugs... :'(

### Assignment 6

> Moved to library

```bash
cd mill2
# release is necessary, else it might take forever...

# Tests for plain run without any output:
cargo test --release --lib -- game::efficient_state::win_decider::unit_tests::t5vs5_run_won_loose_set_generation --exact
cargo test --release --lib -- game::efficient_state::win_decider::unit_tests::t9vs9_run_won_loose_set_generation --exact

# Tests agains reference files in the same directory:
# `input_felder_5vs5_large.txt` & `output_5vs5_large.txt`, `input_felder_5vs5.txt` & `output_3vs3.txt` and `input_felder_3vs3.txt` & `output_3vs3.txt`
cargo test --release --lib -- game::efficient_state::win_decider::unit_tests::t3vs3_all_won_loose_set_correct --exact --nocapture
cargo test --release --lib -- game::efficient_state::win_decider::unit_tests::t5vs5_all_won_loose_set_correct --exact --nocapture
cargo test --release --lib -- game::efficient_state::win_decider::unit_tests::t5vs5_all_won_loose_set_correct_large --exact --nocapture
```
