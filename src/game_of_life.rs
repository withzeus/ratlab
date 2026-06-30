use std::{thread::sleep, time::Duration};

use rand::{self, random_range};

type Board = Vec<Vec<u8>>;

const BOARD_WIDTH: u8 = 60;
const BOARD_HEIGHT: u8 = 10;

const ALIVE : u8 = 1;
const DEAD : u8 = 0;

const ALIVE_STR : &str = "◉";
const DEAD_STR : &str = " ";

pub fn __game_of_life() {
    let mut state = random_state(BOARD_WIDTH, BOARD_HEIGHT);

    loop {
        render(&state);
        state = next_board_state(&state);

        sleep(
            Duration::from_millis(500)
        );
    }
}

fn render(state : &Board){
    for (_, row) in state.iter().enumerate() {
        for (_, c) in row.iter().enumerate(){
            print!("{}", if *c == 1 { ALIVE_STR } else { DEAD_STR } );
        } 
    }
}

fn next_cell_value(c : (usize, usize), state : &Board) -> u8 {
    let x = c.0;
    let y = c.1;

    let mut n_live_neighbors = 0;

    let start_x = x.saturating_sub(1);
    let end_x = std::cmp::min(x + 1, (BOARD_WIDTH - 1) as usize);

    let start_y = y.saturating_sub(1);
    let end_y = std::cmp::min(y + 1, (BOARD_HEIGHT - 1) as usize);

    for x1 in start_x..=end_x{
        for y1 in start_y..=end_y{
            if x1 == x && y1 == y {
                continue;
            }
            if state[y1][x1] == ALIVE {
                n_live_neighbors += 1;
            }

        }
    }

    if state[y][x] == ALIVE {
        if n_live_neighbors <= 1 {
            return DEAD
        }
        else if n_live_neighbors <= 3 {
            return ALIVE
        }
        else {
            return DEAD
        }
    } else {
        if n_live_neighbors == 3 {
            return ALIVE
        }
        return DEAD
    }
}

fn next_board_state(state : &Board) -> Board {
    let mut new_state = dead_state(BOARD_WIDTH, BOARD_HEIGHT);
    
    for (y, row) in state.iter().enumerate(){
        for (x, _) in row.iter().enumerate(){
            new_state[y][x] = next_cell_value((x, y), &state);
        }
    }

    new_state
}   

fn random_state(w: u8, h: u8) -> Board {
    let mut board: Board = Vec::new();

    for _ in 0..h {
        let mut row: Vec<u8> = Vec::new();
        for _ in 0..w {
            let random : u8 = random_range(0..=1);
            row.push(random);
        }

        board.push(row);
    }

    board
}

fn dead_state(w: u8, h: u8) -> Board {
    let mut board: Board = Vec::new();

    for _ in 0..h {
        let mut row: Vec<u8> = Vec::new();
        for _ in 0..w {
            row.push(0);
        }

        board.push(row);
    }

    board
}
