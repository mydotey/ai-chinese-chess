#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Color {
    Red,
    Black,
}

impl Color {
    pub fn opposite(&self) -> Self {
        match self {
            Color::Red => Color::Black,
            Color::Black => Color::Red,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PieceType {
    General,  // King/Shuai/Jiang
    Advisor,  // Guard/Shi
    Elephant, // Xiang/Xiang
    Horse,    // Ma
    Chariot,  // Rook/Ju
    Cannon,   // Pao
    Soldier,  // Pawn/Bing/Zu
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Piece {
    pub color: Color,
    pub piece_type: PieceType,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

impl Pos {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GameState {
    Playing,
    Won(Color),
}

pub struct Board {
    pub grid: [[Option<Piece>; 9]; 10],
    pub turn: Color,
    pub selected: Option<Pos>,
    pub state: GameState,
}

impl Board {
    pub fn new() -> Self {
        let mut grid = [[None; 9]; 10];

        let setup_row = |grid: &mut [[Option<Piece>; 9]; 10], y: usize, color: Color| {
            let pieces = [
                PieceType::Chariot,
                PieceType::Horse,
                PieceType::Elephant,
                PieceType::Advisor,
                PieceType::General,
                PieceType::Advisor,
                PieceType::Elephant,
                PieceType::Horse,
                PieceType::Chariot,
            ];
            for (x, &pt) in pieces.iter().enumerate() {
                grid[y][x] = Some(Piece {
                    color,
                    piece_type: pt,
                });
            }
        };

        // Black pieces (top)
        setup_row(&mut grid, 0, Color::Black);
        grid[2][1] = Some(Piece {
            color: Color::Black,
            piece_type: PieceType::Cannon,
        });
        grid[2][7] = Some(Piece {
            color: Color::Black,
            piece_type: PieceType::Cannon,
        });
        for x in (0..9).step_by(2) {
            grid[3][x] = Some(Piece {
                color: Color::Black,
                piece_type: PieceType::Soldier,
            });
        }

        // Red pieces (bottom)
        setup_row(&mut grid, 9, Color::Red);
        grid[7][1] = Some(Piece {
            color: Color::Red,
            piece_type: PieceType::Cannon,
        });
        grid[7][7] = Some(Piece {
            color: Color::Red,
            piece_type: PieceType::Cannon,
        });
        for x in (0..9).step_by(2) {
            grid[6][x] = Some(Piece {
                color: Color::Red,
                piece_type: PieceType::Soldier,
            });
        }

        Self {
            grid,
            turn: Color::Red,
            selected: None,
            state: GameState::Playing,
        }
    }

    pub fn get_piece(&self, pos: Pos) -> Option<Piece> {
        if pos.x < 9 && pos.y < 10 {
            self.grid[pos.y][pos.x]
        } else {
            None
        }
    }

    pub fn move_piece(&mut self, from: Pos, to: Pos) -> bool {
        if self.state != GameState::Playing {
            return false;
        }
        if let Some(piece) = self.get_piece(from) {
            if piece.color != self.turn {
                return false;
            }
            if self.is_valid_move(from, to) {
                if let Some(target) = self.get_piece(to) {
                    if target.piece_type == PieceType::General {
                        self.state = GameState::Won(self.turn);
                    }
                }

                self.grid[to.y][to.x] = self.grid[from.y][from.x];
                self.grid[from.y][from.x] = None;

                if self.state == GameState::Playing {
                    self.turn = self.turn.opposite();
                }
                return true;
            }
        }
        false
    }

    fn is_valid_move(&self, from: Pos, to: Pos) -> bool {
        if from == to {
            return false;
        }
        if to.x >= 9 || to.y >= 10 {
            return false;
        }

        let piece = match self.get_piece(from) {
            Some(p) => p,
            None => return false,
        };

        // Cannot capture own piece
        if let Some(target) = self.get_piece(to) {
            if target.color == piece.color {
                return false;
            }
        }

        let dx = (to.x as i32 - from.x as i32).abs();
        let dy = (to.y as i32 - from.y as i32).abs();

        match piece.piece_type {
            PieceType::General => {
                // Must stay in palace and move 1 step orthogonally
                if dx + dy != 1 {
                    return false;
                }
                if to.x < 3 || to.x > 5 {
                    return false;
                }
                match piece.color {
                    Color::Red => {
                        if to.y < 7 {
                            return false;
                        }
                    }
                    Color::Black => {
                        if to.y > 2 {
                            return false;
                        }
                    }
                }
                true
            }
            PieceType::Advisor => {
                // Must stay in palace and move 1 step diagonally
                if dx != 1 || dy != 1 {
                    return false;
                }
                if to.x < 3 || to.x > 5 {
                    return false;
                }
                match piece.color {
                    Color::Red => {
                        if to.y < 7 {
                            return false;
                        }
                    }
                    Color::Black => {
                        if to.y > 2 {
                            return false;
                        }
                    }
                }
                true
            }
            PieceType::Elephant => {
                // Move 2 steps diagonally, cannot cross river, eye cannot be blocked
                if dx != 2 || dy != 2 {
                    return false;
                }
                match piece.color {
                    Color::Red => {
                        if to.y < 5 {
                            return false;
                        }
                    }
                    Color::Black => {
                        if to.y > 4 {
                            return false;
                        }
                    }
                }
                // Check eye
                let eye_x = (from.x + to.x) / 2;
                let eye_y = (from.y + to.y) / 2;
                if self.grid[eye_y][eye_x].is_some() {
                    return false;
                }
                true
            }
            PieceType::Horse => {
                // Move "L" shape (1 orthogonal + 1 diagonal), check for blocking leg
                if !((dx == 1 && dy == 2) || (dx == 2 && dy == 1)) {
                    return false;
                }
                // Check leg
                let leg_x = if dx == 2 { (from.x + to.x) / 2 } else { from.x };
                let leg_y = if dy == 2 { (from.y + to.y) / 2 } else { from.y };
                if self.grid[leg_y][leg_x].is_some() {
                    return false;
                }
                true
            }
            PieceType::Chariot => {
                // Move any distance orthogonally, cannot jump
                if dx != 0 && dy != 0 {
                    return false;
                }
                self.count_obstacles(from, to) == 0
            }
            PieceType::Cannon => {
                // Move like Chariot, capture by jumping over exactly one piece
                if dx != 0 && dy != 0 {
                    return false;
                }
                let obstacles = self.count_obstacles(from, to);
                if self.get_piece(to).is_some() {
                    obstacles == 1
                } else {
                    obstacles == 0
                }
            }
            PieceType::Soldier => {
                // Move 1 step forward. After crossing river, can also move sideways.
                if dx + dy != 1 {
                    return false;
                }
                match piece.color {
                    Color::Red => {
                        if (to.y as i32) > (from.y as i32) {
                            return false;
                        } // Cannot move back
                        if from.y >= 5 && dx != 0 {
                            return false;
                        } // Before river, only forward
                    }
                    Color::Black => {
                        if (to.y as i32) < (from.y as i32) {
                            return false;
                        } // Cannot move back
                        if from.y <= 4 && dx != 0 {
                            return false;
                        } // Before river, only forward
                    }
                }
                true
            }
        }
    }

    fn count_obstacles(&self, from: Pos, to: Pos) -> i32 {
        let mut count = 0;
        if from.x == to.x {
            let (min_y, max_y) = if from.y < to.y {
                (from.y, to.y)
            } else {
                (to.y, from.y)
            };
            for y in (min_y + 1)..max_y {
                if self.grid[y][from.x].is_some() {
                    count += 1;
                }
            }
        } else {
            let (min_x, max_x) = if from.x < to.x {
                (from.x, to.x)
            } else {
                (to.x, from.x)
            };
            for x in (min_x + 1)..max_x {
                if self.grid[from.y][x].is_some() {
                    count += 1;
                }
            }
        }
        count
    }
}
