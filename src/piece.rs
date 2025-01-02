use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: PieceColor,
}

impl Piece {
    pub fn get_unicode(&self) -> &'static str {
        match (self.color, self.piece_type) {
            (PieceColor::White, PieceType::King) => "♔",
            (PieceColor::White, PieceType::Queen) => "♕",
            (PieceColor::White, PieceType::Rook) => "♖",
            (PieceColor::White, PieceType::Bishop) => "♗",
            (PieceColor::White, PieceType::Knight) => "♘",
            (PieceColor::White, PieceType::Pawn) => "♙",
            (PieceColor::Black, PieceType::King) => "♚",
            (PieceColor::Black, PieceType::Queen) => "♛",
            (PieceColor::Black, PieceType::Rook) => "♜",
            (PieceColor::Black, PieceType::Bishop) => "♝",
            (PieceColor::Black, PieceType::Knight) => "♞",
            (PieceColor::Black, PieceType::Pawn) => "♟",
        }
    }

    pub fn is_valid_move(&self, from: (usize, usize), to: (usize, usize), board: &[[Option<Piece>; 8]; 8]) -> bool {
        let (from_x, from_y) = from;
        let (to_x, to_y) = to;

        // Basic movement validation for each piece type
        match self.piece_type {
            PieceType::Pawn => {
                let direction = if self.color == PieceColor::White { -1i32 } else { 1i32 };
                let start_rank = if self.color == PieceColor::White { 6 } else { 1 };

                // Normal move
                if to_x as i32 == from_x as i32 + direction && to_y == from_y {
                    return board[to_x][to_y].is_none();
                }

                // Initial two-square move
                if from_x == start_rank && to_x as i32 == from_x as i32 + 2 * direction && to_y == from_y {
                    return board[to_x][to_y].is_none() &&
                        board[(from_x as i32 + direction) as usize][from_y].is_none();
                }

                // Capture
                if to_x as i32 == from_x as i32 + direction && (to_y as i32 - from_y as i32).abs() == 1 {
                    if let Some(piece) = board[to_x][to_y] {
                        return piece.color != self.color;
                    }
                }

                false
            },
            // Add other piece movement validation as needed
            _ => true, // Temporarily allow all moves for other pieces
        }
    }
}