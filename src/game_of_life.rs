use rand::{self, random_range};

type Board = Vec<Vec<u8>>;

pub fn __game_of_life() {
    let width: u8 = 120;
    let height: u8 = 10;

    let state = random_state(width, height);

    println!("Rendered:");
    print!("{:?}", render(state));
}

fn render(state : Board)
{
    let alive : &str = "+";
    let dead : &str = " ";
    for (_, row) in state.iter().enumerate() {
        for (_, c) in row.iter().enumerate(){
            print!("{}", if *c == 1 { alive } else { dead } );
        } 
        println!("");
    }
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
