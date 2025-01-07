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
    fn load() -> Scoreboard { /* ... */ }
    fn save(&self) { /* ... */ }
    fn update(&mut self, player_name: &str) { /* ... */ }
    fn update_lost(&mut self, player_name: &str) { /* ... */ }
    fn display(&self) { /* ... */ }
}
