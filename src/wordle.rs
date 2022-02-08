use crate::words::all_words;
use rand::prelude::SliceRandom;

#[derive(Copy, Clone, Debug)]
pub enum Tile {
    Gray(char),
    Yellow(char),
    Green(char),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Done,
}

// Just for cleaner code
type Row = [Tile; 5];

#[derive(Debug)]
pub struct Game {
    word: [char; 5],
    pub rows: Vec<Row>,
    pub game_state: GameState,
}

pub fn new_game() -> Game {
    Game {
        game_state: GameState::InProgress,
        rows: vec![],
        word: all_words().choose(&mut rand::thread_rng()).unwrap().clone(),
    }
}

pub fn guess(game: &mut Game, guess: [char; 5]) -> &mut Game {
    if !all_words().contains(&guess) {
        // invalid guess
        return game;
    }

    if game.game_state == GameState::Done {
        return game;
    }

    game.rows.push(calculate_tiles(game.word, guess));

    let last_guess_right = game
        .rows
        .last()
        .unwrap()
        .into_iter()
        .all(|tile| match tile {
            Tile::Green(_) => true,
            _ => false,
        });

    if game.rows.len() == 6 || last_guess_right {
        game.game_state = GameState::Done;
    }

    return game;
}

fn calculate_tiles(word: [char; 5], guess: [char; 5]) -> [Tile; 5] {
    let mut tiles: [Tile; 5] = guess.map(|guessed_char| Tile::Gray(guessed_char));

    // First do greens, remove matches
    guess
        .into_iter()
        .zip(word)
        .enumerate()
        .for_each(|(ix, (guessed_char, word_char))| {
            if guessed_char == word_char {
                tiles[ix] = Tile::Green(guessed_char);
            }
        });

    let mut used_yellow_word_chars: Vec<usize> = tiles
        .into_iter()
        .enumerate()
        .filter_map(|(ix, tile)| match tile {
            Tile::Green(_) => Some(ix),
            _ => None,
        })
        .collect();

    // Then to yellows, remove matches
    guess
        .into_iter()
        .enumerate()
        .for_each(|(ix, guessed_char)| {
            // Where are all the occurrences
            let positions_in_word: Vec<usize> = word
                .into_iter()
                .enumerate()
                .filter_map(|(ix, word_char)| {
                    if word_char == guessed_char {
                        return Some(ix);
                    }
                    None
                })
                .collect();

            // Remove the already used ones
            let valid_positions_in_word: Vec<usize> = positions_in_word
                .into_iter()
                .filter(|pos| !used_yellow_word_chars.contains(pos))
                .collect();

            if valid_positions_in_word.len() > 0 {
                tiles[ix] = Tile::Yellow(guessed_char);
                used_yellow_word_chars.push(valid_positions_in_word.first().unwrap().clone());
            }
        });

    return tiles;
}
