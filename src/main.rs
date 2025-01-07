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
