use yew::prelude::*;
use std::ops::Deref;
use crate::piece::{Piece, PieceColor, PieceType};

#[derive(Properties, PartialEq)]
pub struct BoardProps {}

#[function_component(Board)]
pub fn board(_props: &BoardProps) -> Html {
    let board_state: UseStateHandle<[[Option<Piece>; 8]; 8]> = use_state(|| initialize_board());
    let selected_square = use_state(|| None);

    let on_square_click = {
        let board_state = board_state.clone();
        let selected_square = selected_square.clone();

        Callback::from(move |(x, y): (usize, usize)| {
            if let Some((prev_x, prev_y)) = *selected_square {
                let current_board = (*board_state).clone();
                let prev_x_index: Option<&[Option<Piece>; 8]> = current_board.get(prev_x);
                if let Some(prev_x_index) = prev_x_index {
                    let prev_y_index: Option<&Option<Piece>> = prev_x_index.get(prev_y);
                    if let Some(prev_y_index) = prev_y_index {
                        if let Some(piece) = prev_y_index {
                            if piece.is_valid_move((prev_x, prev_y), (x, y), &current_board) {
                                let mut new_board = current_board;
                                new_board[x][y] = new_board[prev_x][prev_y];
                                new_board[prev_x][prev_y] = None;
                                board_state.set(new_board);
                            }
                        }
                    }
                }
                selected_square.set(None);
            } else {
                let current_board = (*board_state).clone();
                if current_board[x][y].is_some() {
                    selected_square.set(Some((x, y)));
                }
            }
        })
    };

    html! {
        <div class="board">
            { for (0..8).map(|x| {
                html! {
                    <div class="row">
                        { for (0..8).map(|y| {
                            let is_selected = selected_square
                                .as_ref()
                                .map_or(false, |&(sx, sy)| sx == x && sy == y);

                            let square_color = if (x + y) % 2 == 0 {
                                "white-square"
                            } else {
                                "black-square"
                            };

                            let piece = board_state[x][y];
                            let piece_symbol = piece.map_or("".to_string(), |p| p.get_unicode().to_string());

                            html! {
                                <div
                                    class={classes!(
                                        "square",
                                        square_color,
                                        if is_selected { "selected" } else { "" }
                                    )}
                                    onclick={let on_square_click = on_square_click.clone();
                                            Callback::from(move |_| on_square_click.emit((x, y)))}
                                >
                                    { piece_symbol }
                                </div>
                            }
                        }) }
                    </div>
                }
            }) }
        </div>
    }
}

fn initialize_board() -> [[Option<Piece>; 8]; 8] {
    let mut board = [[None; 8]; 8];

    // Initialize pawns
    for i in 0..8 {
        board[1][i] = Some(Piece {
            piece_type: PieceType::Pawn,
            color: PieceColor::Black,
        });
        board[6][i] = Some(Piece {
            piece_type: PieceType::Pawn,
            color: PieceColor::White,
        });
    }

    // Initialize other pieces
    let pieces = [
        PieceType::Rook,
        PieceType::Knight,
        PieceType::Bishop,
        PieceType::Queen,
        PieceType::King,
        PieceType::Bishop,
        PieceType::Knight,
        PieceType::Rook,
    ];

    for (i, &piece_type) in pieces.iter().enumerate() {
        board[0][i] = Some(Piece {
            piece_type,
            color: PieceColor::Black,
        });
        board[7][i] = Some(Piece {
            piece_type,
            color: PieceColor::White,
        });
    }

    board
}