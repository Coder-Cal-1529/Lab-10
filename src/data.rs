// Loading and saving game data.
use crate::board::Board;
use std::fs;
use crate::board::{Cell,SIZE, new_board};
pub const MIN_SHIP: usize = 2;
pub const MAX_SHIP: usize = 5;

#[derive(Debug, Clone, PartialEq)]
pub struct ShipSpec {
    pub name: String,
    pub length: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScoreRecord {
    pub name: String,
    pub score: u32,
}

// WORKED EXAMPLE (provided). Study this before writing parse_ship_line.
// One line of scores.txt looks like:  Ana,83
// Returns None for any line that is not "name,number".
pub fn parse_score(line: &str) -> Option<ScoreRecord> {
    let parts: Vec<&str> = line.trim().split(',').collect();
    if parts.len() != 2 {
        return None;
    }
    let name = parts[0].trim();
    if name.is_empty() {
        return None;
    }
    match parts[1].trim().parse::<u32>() {
        Ok(score) => Some(ScoreRecord {
            name: name.to_string(),
            score,
        }),
        Err(_) => None,
    }
}

// PROVIDED: sort highest score first (selection sort).
pub fn sort_scores_desc(records: &mut Vec<ScoreRecord>) {
    for i in 0..records.len() {
        let mut best = i;
        for j in (i + 1)..records.len() {
            if records[j].score > records[best].score {
                best = j;
            }
        }
        records.swap(i, best);
    }
}

// ---------------------------------------------------------------------
// TASKS: write the six functions below (README has the task order).
// Each has a stub that compiles.let mut valid_ships: Vec<ShipSpec>;
// You will need to add `use` lines at the top as you go.
// ---------------------------------------------------------------------

// Task 1.
// One line of fleet.txt looks like:  Carrier,5
// Return None unless the line has exactly one comma, a non-empty trimmed name,
// and a length that parses as a number from MIN_SHIP to MAX_SHIP.
// Model it on parse_score above.
pub fn parse_ship_line(line: &str) -> Option<ShipSpec> {
	let ship: Vec<&str> = line.trim().split(',').collect();
	if ship.len() != 2 {
        	return None;
    	}
	let name = ship[0].trim();
	if name.is_empty() {
        	return None;
	}
	match ship[1].trim().parse::<usize>() {
        	Ok(length) => Some(ShipSpec {
        		    name: name.to_string(),
        		    length,
      				}),
        	Err(_) => return None,
	};
let length: usize = ship[1].parse::<usize>().unwrap();
if length>=MIN_SHIP && length<=MAX_SHIP	{
return Some(ShipSpec{
name: ship[0].trim().to_string(),
length,
})
}else{
return None
}
}
// Task 2.
// Read path and return every valid ship. Blank lines and lines starting with
// '#' are ignored silently. Any other line parse_ship_line rejects is counted;
// if the count is above 0, print Skipped N invalid line(s) in <path>
// to stderr (eprintln!). If the file cannot be read, print the error to stderr and return an empty Vec.
pub fn load_fleet(path: &str) -> Vec<ShipSpec> {
	let mut ships = Vec::new();
	let mut inv_line: i32 = 0;
	let fleet = match fs::read_to_string(path){
		Ok(fleet) => fleet,
		Err(err) =>{
			eprintln!("Unable to read file due to Error: {}", err);
       			return vec![]
        	}
	};
	for line in fleet.lines(){
	let skip = line.trim();
                if skip.is_empty()||skip.starts_with('#'){
                        continue;
                }
		match parse_ship_line(line){
			Some(ship_spec) => ships.push(ship_spec),
			None => inv_line += 1,
		};
	}
	if inv_line > 0{ 
		eprintln!("Skipped {} invalid line(s) in {}", inv_line, path);
	};
	ships
}

     
// Task 3.
// Turn the text of a board file into a Board (a 2D array). The text has SI// rows of SIZE characters: '.' is water, 'S' is a ship. Blank lines and lines
// starting with '#' are ignored. Return None for too few rows, too many rows,
// a row of the wrong width, or any other character.
// Start from board::new_board() and fill in board[row][col].
pub fn parse_board(text: &str) -> Option<Board> {
	let mut board = new_board();
	let mut row_count: usize =0;
	for row in text.lines(){
		let line = row.trim();
		if line.is_empty()||line.starts_with('#'){
			continue;
		}
		if row_count >= SIZE{
			return None;
		}
		if line.chars().count() != SIZE{
			return None
		}
		for (col,ch) in line.chars().enumerate(){ 		
		match ch{
		'.' => board[row_count][col] = Cell::Water,
		'S' => board[row_count][col] = Cell::Ship,
		_ => return None,
		}		
		}
	
	row_count +=1;
	}
	if row_count != SIZE{
	return None;
	}
Some(board)
}
// Task 4.
// Read the file at path and parse it with parse_board. On a read error, or if
// parse_board returns None, print a message to stderr and return None.
pub fn load_board(path: &str) -> Option<Board> {
	let board_file = match fs::read_to_string(path) {   
		Ok(board) => board,
		Err(err) => {
			eprintln!("File could not be read due to Error: {}", err);
			return None;
		}
	};
	match parse_board(&board_file){
		Some(board)=> Some(board),
		None => {
			eprintln!("<{}> is not a valid board file", path);
			None
		}
	}
}
// Task 6.
// Append "name,score" as one line to path, creating the file if needed.
// Replace commas in the name with spaces so the line stays parseable.
// Return true if the write succeeded; on failure print the error to stderr
// and return false.
pub fn append_score(_path: &str, _record: &ScoreRecord) -> bool {
    todo!("Task 6: append_score")
}

// Task 7.
// Read every valid score line from path. A missing file is normal on the
// first run: return an empty Vec and print nothing. Skip lines that
// parse_score rejects.
pub fn load_scores(_path: &str) -> Vec<ScoreRecord> {
    todo!("Task 7: load_scores")
}

