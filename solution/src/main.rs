use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let mut game = Game::new();
    let mut inputs = stdin.lock().lines();
    while let Some(Ok(line)) = inputs.next() {
        match line.split_whitespace().next() {
            Some("$$$") | Some("exec") => {
                game.update_player_info(&line);
            }
            Some("Anfield") => {
                game.parse_grid(&mut inputs, &line);
            }
            Some("Piece") => {
                game.parse_piece(&mut inputs, &line);
                game.find_best_piece_position();
            }
            _ => (),
        }
    }
}

struct Game {
    grid: Vec<Vec<char>>,
    piece: Vec<Vec<char>>,
    player_id: String,
    player_command: String,
}

impl Game {
    fn new() -> Self {
        Self {
            grid: Vec::new(),
            piece: Vec::new(),
            player_id: String::new(),
            player_command: String::new(),
        }
    }

    fn update_player_info(&mut self, line: &str) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        self.player_id = parts[2].to_string();
        self.player_command = line.to_string();
    }

    fn parse_grid(&mut self, inputs: &mut io::Lines<io::StdinLock>, line: &str) {
        let grid_height: usize = line
            .split_whitespace()
            .find(|&word| word.ends_with(':'))
            .and_then(|word| word.trim_end_matches(':').parse().ok())
            .unwrap();
        inputs.next();
        self.grid.clear();
        for _ in 0..grid_height {
            let grid_line = inputs.next().unwrap().unwrap();
            let row: Vec<char> = grid_line.chars().skip(4).collect();
            self.grid.push(row);
        }
    }

    fn parse_piece(&mut self, inputs: &mut io::Lines<io::StdinLock>, line: &str) {
        let piece_height: usize = line
            .split_whitespace()
            .find(|&word| word.ends_with(':'))
            .and_then(|word| word.trim_end_matches(':').parse().ok())
            .unwrap();
        self.piece.clear();
        for _ in 0..piece_height {
            let piece_line = inputs.next().unwrap().unwrap();
            let row: Vec<char> = piece_line.chars().collect();
            self.piece.push(row);
        }
    }

    fn find_best_piece_position(&self) {
        let (enemy, enemy2) = self.get_enemy_chars();
        let position = self.get_best_position(enemy, enemy2);
        if let Some((x, y)) = position {
            writeln!(io::stdout(), "{} {}", y, x).unwrap();
        } else {
            writeln!(io::stdout(), "0 0").unwrap();
        }
        io::stdout().flush().unwrap();
    }

    fn get_enemy_chars(&self) -> (char, char) {
        match (self.player_id.as_str(), self.player_command.contains("solution")) {
            ("p1", true) => ('s', '$'),
            ("p2", true) => ('a', '@'),
            ("p1", false) => ('a', '@'),
            ("p2", false) => ('s', '$'),
            _ => (' ', ' '),
        }
    }

    fn find_best_positions(&self, enemy: char, enemy2: char) -> Vec<(usize, usize)> {
        let mut best_positions = Vec::new();
        for x in 0..self.grid.len() {
            for y in 0..self.grid[0].len() {
                let mut count = 0;
                let mut valid = true;
                for (i, piece_row) in self.piece.iter().enumerate() {
                    for (j, &piece_char) in piece_row.iter().enumerate() {
                        let grid_x = x + i;
                        let grid_y = y + j;
                        if grid_x >= self.grid.len()
                            || grid_y >= self.grid[0].len()
                            || (piece_char != '.'
                                && (self.grid[grid_x][grid_y] == enemy
                                    || self.grid[grid_x][grid_y] == enemy2))
                        {
                            valid = false;
                            break;
                        }
                        if piece_char != '.' && self.grid[grid_x][grid_y] != '.' {
                            count += 1;
                        }
                    }
                    if !valid {
                        break;
                    }
                }
                if valid && count == 1 {
                    best_positions.push((x, y));
                }
            }
        }
        best_positions
    }

    fn get_best_position(&self, enemy: char, enemy2: char) -> Option<(usize, usize)> {
        let best_positions = self.find_best_positions(enemy, enemy2);
        let mut enemy_cells = Vec::new();
        for (i, row) in self.grid.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                if cell == enemy || cell == enemy2 {
                    enemy_cells.push((i, j));
                }
            }
        }
        let mut nearest_position = None;
        let mut min_distance = f64::INFINITY;
        for &best_position in &best_positions {
            let mut current_distance = f64::INFINITY;
            for &enemy_position in &enemy_cells {
                let dist = {
                    let (x1, y1) = best_position;
                    let (x2, y2) = enemy_position;
                    let diff_x = ((x2 as isize) - (x1 as isize)) as f64;
                    let diff_y = ((y2 as isize) - (y1 as isize)) as f64;
                    (diff_x.powi(2) + diff_y.powi(2)).sqrt()
                };
                if dist < current_distance {
                    current_distance = dist;
                }
            }
            if current_distance < min_distance {
                min_distance = current_distance;
                nearest_position = Some(best_position);
            }
        }
        nearest_position
    }
}
