// Adding shakmaty as the main library for the chess game
use shakmaty::{Chess, Color, Position, Role, Square};
use shakmaty::uci::UciMove;
use std::io::{self, Write};

fn main() {
    // Start a new game from the standard starting position
    let mut pos = Chess::default();

    println!("=== Simple Console Chess ===");
    println!("Enter moves in UCI format, e.g. e2e4  or  g1f3");
    println!("Type 'quit' to exit.\n");

    loop {
        // Print the current board
        print_board(&pos);

        // Show whose turn it is
        let turn = if pos.turn() == Color::White {
            "White"
        } else {
            "Black"
        };
        println!("\n{} to move.", turn);

        // Check for game over
        if pos.is_checkmate() {
            println!("Checkmate! {} wins.", if pos.turn() == Color::White { "Black" } else { "White" });
            break;
        }
        if pos.is_stalemate() {
            println!("Stalemate! It's a draw.");
            break;
        }
        if pos.is_insufficient_material() {
            println!("Draw by insufficient material.");
            break;
        }

        // Ask for a move
        print!("Your move: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let input = input.trim();

        if input.eq_ignore_ascii_case("quit") || input.eq_ignore_ascii_case("exit") {
            println!("Goodbye!");
            break;
        }

        // Try to parse the move as UCI (e.g. "e2e4")
        match input.parse::<UciMove>() {
            Ok(uci) => {
                // Convert UCI to a legal Move in the current position
                match uci.to_move(&pos) {
                    Ok(m) => {
                        // Apply the move
                        pos = pos.play(m).expect("Move was already validated as legal");
                        println!("Move played: {}\n", input);
                    }
                    Err(_) => {
                        println!("Illegal move. Try again.\n");
                    }
                }
            }
            Err(_) => {
                println!("Invalid format. Use UCI like e2e4 or g1f3.\n");
            }
        }
    }
}

/// Simple function that prints the board with Unicode chess pieces
fn print_board(pos: &Chess) {
    println!("\n  a b c d e f g h");
    println!("  ---------------");

    for rank in (0..8).rev() {
        print!("{}|", rank + 1);

        for file in 0..8 {
            let square = Square::new(file + rank * 8);
            let piece = pos.board().piece_at(square);

            let symbol = match piece {
                Some(p) => match (p.color, p.role) {
                    (Color::White, Role::King)   => "♔",
                    (Color::White, Role::Queen)  => "♕",
                    (Color::White, Role::Rook)   => "♖",
                    (Color::White, Role::Bishop) => "♗",
                    (Color::White, Role::Knight) => "♘",
                    (Color::White, Role::Pawn)   => "♙",
                    (Color::Black, Role::King)   => "♚",
                    (Color::Black, Role::Queen)  => "♛",
                    (Color::Black, Role::Rook)   => "♜",
                    (Color::Black, Role::Bishop) => "♝",
                    (Color::Black, Role::Knight) => "♞",
                    (Color::Black, Role::Pawn)   => "♟",
                },
                None => ".",
            };

            print!("{} ", symbol);
        }
        println!("|{}", rank + 1);
    }

    println!("  ---------------");
    println!("  a b c d e f g h");
}