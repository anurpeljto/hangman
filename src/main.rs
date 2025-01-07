use std::collections::HashSet;
use std::fs;
use std::io::{self, Write};
use serde::{Deserialize, Serialize};
use rand::seq::SliceRandom;
use colored::*; 

const STAGES: [&str; 7] = [
    "\n  +---+\n  |   |\n  O   |\n /|\\  |\n / \\\n      |\n=========\n",
    "\n  +---+\n  |   |\n  O   |\n /|\\  |\n /    |\n      |\n=========\n",
    "\n  +---+\n  |   |\n  O   |\n /|\\  |\n      |\n      |\n=========\n",
    "\n  +---+\n  |   |\n  O   |\n /|   |\n      |\n      |\n=========\n",
    "\n  +---+\n  |   |\n  O   |\n  |   |\n      |\n      |\n=========\n",
    "\n  +---+\n  |   |\n  O   |\n      |\n      |\n      |\n=========\n",
    "\n  +---+\n  |   |\n      |\n      |\n      |\n      |\n=========\n",
];

#[derive(Serialize, Deserialize)]
struct GameState {
    player1: String,
    player2: Option<String>,
    turn: usize,
    chosen_word: String,
    placeholder: String,
    correct_letters: HashSet<char>,
    guessed_letters: HashSet<char>,
    lives: usize,
    is_player1_choosing: bool,
}

#[derive(Serialize, Deserialize)]
struct PlayerScore {
    name: String,
    wins: usize,
}

#[derive(Serialize, Deserialize)]
struct Scoreboard {
    players: Vec<PlayerScore>,
}

impl Scoreboard {
    fn load() -> Scoreboard {
        match fs::read_to_string("scoreboard.json") {
            Ok(data) => serde_json::from_str(&data).unwrap_or(Scoreboard { players: Vec::new() }),
            Err(_) => Scoreboard { players: Vec::new() },
        }
    }

    fn save(&self) {
        let data = serde_json::to_string(self).expect("Failed to serialize scoreboard.");
        fs::write("scoreboard.json", data).expect("Failed to save scoreboard.");
    }

    fn update(&mut self, player_name: &str) {
        if let Some(player) = self.players.iter_mut().find(|p| p.name == player_name) {
            player.wins += 1;
        } else {
            self.players.push(PlayerScore {
                name: player_name.to_string(),
                wins: 1,
            });
        }
        self.players.sort_by(|a, b| b.wins.cmp(&a.wins));
        self.save();
    }

    fn update_lost(&mut self, player_name: &str) {
        if let Some(player) = self.players.iter_mut().find(|p| p.name == player_name) {
            return;
        } else {
            self.players.push(PlayerScore {
                name: player_name.to_string(),
                wins: 0,
            });
        }
        self.players.sort_by(|a, b| b.wins.cmp(&a.wins));
        self.save();
    }

    fn display(&self) {
        println!("\n---- {} ----", "Scoreboard".bold().green());
        for player in &self.players {
            println!("{}: {} wins", player.name.green(), player.wins.to_string().yellow());
        }
    }
}

fn main() {
    println!("{}", "Welcome to Hangman!".bold().cyan());
    println!("1. New Game\n2. Load Game\n3. View Scoreboard\n");

    let choice = get_input("Choose an option: ").trim().to_string();

    match choice.as_str() {
        "1" => new_game(),
        "2" => load_game(),
        "3" => view_scoreboard(),
        _ => println!("{}", "Invalid choice! Exiting.".red()),
    }
}

fn new_game() {
    let mode = get_input("Multiplayer? (yes/no): ").to_lowercase();
    let player1 = get_input("Enter Player 1's name: ");
    let player2 = if mode == "yes" {
        Some(get_input("Enter Player 2's name: "))
    } else {
        None
    };

    let (chosen_word, is_player1_choosing) = if mode == "yes" {
        let choose_word = get_input("Player 1, do you want to choose a word for Player 2 to guess? (yes/no): ").to_lowercase();
        if choose_word == "yes" {
            println!("{}", format!("{} (Player 1), enter a word for Player 2 to guess.", player1).yellow());
            let word = get_input_hidden();
            println!("{}", "Game starting!".bold().green());
            (word.to_lowercase(), true)
        } else {
            let word_list = vec!["bicycle", "hangman", "elephant", "laptop", "programming"];
            (word_list.choose(&mut rand::thread_rng()).unwrap().to_string(), false)
        }
    } else {
        let word_list = vec!["bicycle", "hangman", "elephant", "laptop", "programming"];
        (word_list.choose(&mut rand::thread_rng()).unwrap().to_string(), false)
    };

    let placeholder = "_".repeat(chosen_word.len());
    let state = GameState {
        player1,
        player2,
        turn: 1,
        chosen_word: chosen_word.to_lowercase(),
        placeholder,
        correct_letters: HashSet::new(),
        guessed_letters: HashSet::new(),
        lives: 6,
        is_player1_choosing,
    };

    play_game(state);
}

fn load_game() {
    match fs::read_to_string("save.json") {
        Ok(data) => {
            let state: GameState = serde_json::from_str(&data).expect("Failed to parse save file.");
            println!("{}", "Game loaded. Resuming...".green());
            play_game(state);
        }
        Err(_) => println!("{}", "No save file found. Starting a new game.".red()),
    }
}
