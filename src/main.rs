use crate::wordle::GameState;
use std::io;

mod wordle;
mod words;

fn main() {
    let mut game = wordle::new_game();

    let mut user_input = String::new();

    while game.game_state != GameState::Done {
        io::stdin()
            .read_line(&mut user_input)
            .expect("Couldn't read from stdin");

        let guess: [char; 5] = user_input
            .trim()
            .chars()
            .collect::<Vec<char>>()
            .try_into()
            .expect("Your input must be five lower-case characters only");

        wordle::guess(&mut game, guess);

        user_input.truncate(0);

        println!("{:?}", game.rows.last());
    }

    println!("Done!");
}
