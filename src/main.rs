use battleship::board::{self, Board, Shot};
use battleship::data::{self, ShipSpec};
use battleship::ui;
use rand::SeedableRng;
use rand::rngs::StdRng;
use crate::data::ScoreRecord;
const FLEET_PATH: &str = "data/fleet.txt";
const SCORES_PATH: &str ="data/scores.txt";
const PRACTICE_PATH: &str = "data/boards/practice.txt";
const TOP_N: usize = 5;

fn main() {
    let fleet = data::load_fleet(FLEET_PATH);
    if fleet.is_empty() {
        println!("No fleet loaded. Check {FLEET_PATH}.");
        return;
    }

    let menu = vec![
        String::from("Play"),
        String::from("Practice board"),
        String::from("Quit"),
    ];
    loop {
        println!("\n=== BATTLESHIP ===");
        show_top_scores(TOP_N);
        match ui::menu_choice("\nChoose:", &menu) {
            1 => play_game(&fleet),
            2 => practice_game(),
            _ => {
                println!("Goodbye.");
                break;
            }
        }
    }
}

// Print the best scores, highest first, or a message if there are none.
fn show_top_scores(n: usize) {
	println!("TOP {n}");
	let mut score_records =  data::load_scores(SCORES_PATH);
	if score_records.is_empty(){
		println!("	no scores yet");
		return
	}
	data::sort_scores_desc(&mut score_records);
	let limit = std::cmp::min(n, score_records.len());
	for i in 0..limit{
		let records = &score_records[i];
		println!("{:>2}. {:<15} {:>4}", i + 1, records.name, records.score);

	}
}

// A scored game against a randomly placed fleet.
fn play_game(fleet: &[ShipSpec]) {
    let mut board = board::new_board();
    let mut rng = StdRng::seed_from_u64(rand::random::<u64>());
    if !board::place_fleet(&mut board, fleet, &mut rng) {
        println!("The fleet does not fit on the board. Check {FLEET_PATH}.");
        return;
    }
    if let Some(shots) = play_round(&mut board) {
        let score = board::score_for(shots);
        println!("All ships sunk in {shots} shots. Score: {score}");
        // TODO (Task 10): if score > 0, ask for a name with ui::prompt, build a
        // ScoreRecord (add it to the `use` line), and save it with
        // data::append_score. Print "Score saved." only when it returns true

if score >0 {
let given_name = ui::prompt("Name for the high-score list: ");
if given_name.is_empty(){
return
}else{
let mut success: bool = false;
while success == false{
success = data::append_score(SCORES_PATH, &ScoreRecord{
name: given_name.clone(),
score:  score,
});
}
println!("Score Saved");
}
}
}
}
// An unscored game on the fixed layout in the practice file.
fn practice_game() {
let mut board = data::load_board(PRACTICE_PATH);
match play_round(board.as_mut().unwrap()){
	Some(shots) =>{
		println!("Practice boad cleard in {} shots", shots);
	},
	None => {
println!("Practice board unavailable");
	},
}
}
// Fire until every ship is sunk. Returns the number of shots, or None if the
// player types "quit". PROVIDED.
fn play_round(board: &mut Board) -> Option<u32> {
    let mut shots = 0;
    while !board::all_sunk(board) {
        println!();
        print!("{}", board::render(board, false));
        let text = ui::prompt("Fire at (like B7, or 'quit'): ");
        if text.eq_ignore_ascii_case("quit") {
            return None;
        }
        match board::parse_coord(&text) {
            None => println!("Enter a column A-J and a row 1-10, like B7."),
            Some((row, col)) => match board::fire(board, row, col) {
                Shot::Hit => {
                    shots += 1;
                    println!("Hit!");
                }
                Shot::Miss => {
                    shots += 1;
                    println!("Miss.");
                }
                Shot::AlreadyShot => println!("You already fired there."),
            },
        }
    }
    print!("\n{}", board::render(board, false));
    Some(shots)
}
