use std::error::Error;

mod ascii_art;
mod game_of_life;

fn main() -> Result<(), Box<dyn Error>> {
    
    // let __ = ascii_art::__gen_nika_art();
    game_of_life::__game_of_life();
    Ok(())
}
