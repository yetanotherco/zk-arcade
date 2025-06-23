//! this module contains the Beast trait with a couple default implmentation of helper functions

use std::{cmp::Ordering, collections::HashMap};

use crate::{board::Board, Coord, Tile, BOARD_HEIGHT, BOARD_WIDTH};

/// the action a beast can take
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BeastAction {
    /// the beast has killed the player
    PlayerKilled,
    /// the beast has moved to a new position
    Moved,
    /// the beast stayed in its current position
    Stayed,
}

#[derive(Debug, Clone)]
pub enum BeastAdvanceError {
    InvalidMovement,
}
/// this trait defines the common behavior of all beasts in the game
pub trait Beast {
    /// creates a new instance of the beast and stores its position
    fn new(position: Coord) -> Self;

    /// advances the beast's position to the given coord and returns the action taken
    fn advance_to(
        &mut self,
        board: &mut Board,
        player_position: Coord,
        new_pos: Coord,
    ) -> Result<BeastAction, BeastAdvanceError>;

    /// advances the beast's position and returns the action taken
    fn advance(&mut self, board: &mut Board, player_position: Coord) -> BeastAction;

    /// returns the score for when this beast is crushed
    fn get_score() -> u16;

    /// return if a tile is walkable
    fn is_walkable_tile(tile: &Tile) -> bool {
        matches!(tile, Tile::Empty | Tile::Player)
    }

    /// returns all walkable neighbors (8-directional) for a given position
    fn get_walkable_coords(
        board: &Board,
        position: &Coord,
        player_position: &Coord,
        check_tiles: bool,
    ) -> Vec<Coord> {
        let mut result = Vec::with_capacity(8);

        // top row
        let left_top: Coord = Coord {
            column: position.column.saturating_sub(1),
            row: position.row.saturating_sub(1),
        };
        let middle_top: Coord = Coord {
            column: position.column,
            row: position.row.saturating_sub(1),
        };
        let right_top: Coord = Coord {
            column: std::cmp::min(position.column + 1, BOARD_WIDTH - 1),
            row: position.row.saturating_sub(1),
        };

        // middle row
        let left_middle: Coord = Coord {
            column: position.column.saturating_sub(1),
            row: position.row,
        };
        let right_middle: Coord = Coord {
            column: std::cmp::min(position.column + 1, BOARD_WIDTH - 1),
            row: position.row,
        };

        // bottom row
        let left_bottom: Coord = Coord {
            column: position.column.saturating_sub(1),
            row: std::cmp::min(position.row + 1, BOARD_HEIGHT - 1),
        };
        let middle_bottom: Coord = Coord {
            column: position.column,
            row: std::cmp::min(position.row + 1, BOARD_HEIGHT - 1),
        };
        let right_bottom: Coord = Coord {
            column: std::cmp::min(position.column + 1, BOARD_WIDTH - 1),
            row: std::cmp::min(position.row + 1, BOARD_HEIGHT - 1),
        };

        match (
            player_position.column.cmp(&position.column),
            player_position.row.cmp(&position.row),
        ) {
            (Ordering::Equal, Ordering::Greater) => {
                // player is straight below
                // 6 8  7
                // 4 ├┤ 5
                // 2 ◀▶ 3
                if Self::is_walkable_tile(&board[&middle_bottom]) || !check_tiles {
                    result.push(middle_bottom);
                }

                if !result.contains(&left_bottom) && Self::is_walkable_tile(&board[&left_bottom])
                    || !check_tiles
                {
                    result.push(left_bottom);
                }
                if !result.contains(&right_bottom) && Self::is_walkable_tile(&board[&right_bottom])
                    || !check_tiles
                {
                    result.push(right_bottom);
                }
                if !result.contains(&left_middle) && Self::is_walkable_tile(&board[&left_middle])
                    || !check_tiles
                {
                    result.push(left_middle);
                }
                if !result.contains(&right_middle) && Self::is_walkable_tile(&board[&right_middle])
                    || !check_tiles
                {
                    result.push(right_middle);
                }
                if !result.contains(&left_top) && Self::is_walkable_tile(&board[&left_top])
                    || !check_tiles
                {
                    result.push(left_top);
                }
                if !result.contains(&right_top) && Self::is_walkable_tile(&board[&right_top])
                    || !check_tiles
                {
                    result.push(right_top);
                }
                if !result.contains(&middle_top) && Self::is_walkable_tile(&board[&middle_top])
                    || !check_tiles
                {
                    result.push(middle_top);
                }
            }
            (Ordering::Equal, Ordering::Less) => {
                // player is straight above
                // 2 ◀▶ 3
                // 4 ├┤ 5
                // 6 8  7
                if Self::is_walkable_tile(&board[&middle_top]) || !check_tiles {
                    result.push(middle_top);
                }

                if !result.contains(&left_top) && Self::is_walkable_tile(&board[&left_top])
                    || !check_tiles
                {
                    result.push(left_top);
                }
                if !result.contains(&right_top) && Self::is_walkable_tile(&board[&right_top])
                    || !check_tiles
                {
                    result.push(right_top);
                }
                if !result.contains(&left_middle) && Self::is_walkable_tile(&board[&left_middle])
                    || !check_tiles
                {
                    result.push(left_middle);
                }
                if !result.contains(&right_middle) && Self::is_walkable_tile(&board[&right_middle])
                    || !check_tiles
                {
                    result.push(right_middle);
                }
                if !result.contains(&left_bottom) && Self::is_walkable_tile(&board[&left_bottom])
                    || !check_tiles
                {
                    result.push(left_bottom);
                }
                if !result.contains(&right_bottom) && Self::is_walkable_tile(&board[&right_bottom])
                    || !check_tiles
                {
                    result.push(right_bottom);
                }
                if !result.contains(&middle_bottom)
                    && Self::is_walkable_tile(&board[&middle_bottom])
                    || !check_tiles
                {
                    result.push(middle_bottom);
                }
            }
            (Ordering::Less, Ordering::Equal) => {
                // player is straight left
                // 2 4  6
                // ◀▶├┤ 8
                // 3 5  7
                if Self::is_walkable_tile(&board[&left_middle]) || !check_tiles {
                    result.push(left_middle);
                }

                if !result.contains(&left_top) && Self::is_walkable_tile(&board[&left_top])
                    || !check_tiles
                {
                    result.push(left_top);
                }
                if !result.contains(&left_bottom) && Self::is_walkable_tile(&board[&left_bottom])
                    || !check_tiles
                {
                    result.push(left_bottom);
                }
                if !result.contains(&middle_top) && Self::is_walkable_tile(&board[&middle_top])
                    || !check_tiles
                {
                    result.push(middle_top);
                }
                if !result.contains(&middle_bottom)
                    && Self::is_walkable_tile(&board[&middle_bottom])
                    || !check_tiles
                {
                    result.push(middle_bottom);
                }
                if !result.contains(&right_top) && Self::is_walkable_tile(&board[&right_top])
                    || !check_tiles
                {
                    result.push(right_top);
                }
                if !result.contains(&right_bottom) && Self::is_walkable_tile(&board[&right_bottom])
                    || !check_tiles
                {
                    result.push(right_bottom);
                }
                if !result.contains(&right_middle) && Self::is_walkable_tile(&board[&right_middle])
                    || !check_tiles
                {
                    result.push(right_middle);
                }
            }
            (Ordering::Greater, Ordering::Equal) => {
                // player is straight right
                // 6 4  2
                // 8 ├┤◀▶
                // 7 5  3
                if Self::is_walkable_tile(&board[&right_middle]) || !check_tiles {
                    result.push(right_middle);
                }

                if !result.contains(&right_top) && Self::is_walkable_tile(&board[&right_top])
                    || !check_tiles
                {
                    result.push(right_top);
                }
                if !result.contains(&right_bottom) && Self::is_walkable_tile(&board[&right_bottom])
                    || !check_tiles
                {
                    result.push(right_bottom);
                }
                if !result.contains(&middle_top) && Self::is_walkable_tile(&board[&middle_top])
                    || !check_tiles
                {
                    result.push(middle_top);
                }
                if !result.contains(&middle_bottom)
                    && Self::is_walkable_tile(&board[&middle_bottom])
                    || !check_tiles
                {
                    result.push(middle_bottom);
                }
                if !result.contains(&left_top) && Self::is_walkable_tile(&board[&left_top])
                    || !check_tiles
                {
                    result.push(left_top);
                }
                if !result.contains(&left_bottom) && Self::is_walkable_tile(&board[&left_bottom])
                    || !check_tiles
                {
                    result.push(left_bottom);
                }
                if !result.contains(&left_middle) && Self::is_walkable_tile(&board[&left_middle])
                    || !check_tiles
                {
                    result.push(left_middle);
                }
            }
            (Ordering::Greater, Ordering::Greater) => {
                // player is below right
                // 8 7  5
                // 6 ├┤ 3
                // 4 2 ◀▶
                if Self::is_walkable_tile(&board[&right_bottom]) || !check_tiles {
                    result.push(right_bottom);
                }

                if !result.contains(&middle_bottom)
                    && Self::is_walkable_tile(&board[&middle_bottom])
                    || !check_tiles
                {
                    result.push(middle_bottom);
                }
                if !result.contains(&right_middle) && Self::is_walkable_tile(&board[&right_middle])
                    || !check_tiles
                {
                    result.push(right_middle);
                }
                if !result.contains(&left_bottom) && Self::is_walkable_tile(&board[&left_bottom])
                    || !check_tiles
                {
                    result.push(left_bottom);
                }
                if !result.contains(&right_top) && Self::is_walkable_tile(&board[&right_top])
                    || !check_tiles
                {
                    result.push(right_top);
                }
                if !result.contains(&left_middle) && Self::is_walkable_tile(&board[&left_middle])
                    || !check_tiles
                {
                    result.push(left_middle);
                }
                if !result.contains(&middle_top) && Self::is_walkable_tile(&board[&middle_top])
                    || !check_tiles
                {
                    result.push(middle_top);
                }
                if !result.contains(&left_top) && Self::is_walkable_tile(&board[&left_top])
                    || !check_tiles
                {
                    result.push(left_top);
                }
            }
            (Ordering::Greater, Ordering::Less) => {
                // player is above right
                // 4 2 ◀▶
                // 6 ├┤ 3
                // 8 7  5
                if Self::is_walkable_tile(&board[&right_top]) || !check_tiles {
                    result.push(right_top);
                }

                if !result.contains(&middle_top) && Self::is_walkable_tile(&board[&middle_top])
                    || !check_tiles
                {
                    result.push(middle_top);
                }
                if !result.contains(&right_middle) && Self::is_walkable_tile(&board[&right_middle])
                    || !check_tiles
                {
                    result.push(right_middle);
                }
                if !result.contains(&left_top) && Self::is_walkable_tile(&board[&left_top])
                    || !check_tiles
                {
                    result.push(left_top);
                }
                if !result.contains(&right_bottom) && Self::is_walkable_tile(&board[&right_bottom])
                    || !check_tiles
                {
                    result.push(right_bottom);
                }
                if !result.contains(&left_middle) && Self::is_walkable_tile(&board[&left_middle])
                    || !check_tiles
                {
                    result.push(left_middle);
                }
                if !result.contains(&middle_bottom)
                    && Self::is_walkable_tile(&board[&middle_bottom])
                    || !check_tiles
                {
                    result.push(middle_bottom);
                }
                if !result.contains(&left_bottom) && Self::is_walkable_tile(&board[&left_bottom])
                    || !check_tiles
                {
                    result.push(left_bottom);
                }
            }
            (Ordering::Less, Ordering::Greater) => {
                // player is below left
                // 4 6  8
                // 2 ├┤ 7
                // ◀▶ 3 5
                if Self::is_walkable_tile(&board[&left_bottom]) || !check_tiles {
                    result.push(left_bottom);
                }

                if !result.contains(&left_middle) && Self::is_walkable_tile(&board[&left_middle])
                    || !check_tiles
                {
                    result.push(left_middle);
                }
                if !result.contains(&middle_bottom)
                    && Self::is_walkable_tile(&board[&middle_bottom])
                    || !check_tiles
                {
                    result.push(middle_bottom);
                }
                if !result.contains(&left_top) && Self::is_walkable_tile(&board[&left_top])
                    || !check_tiles
                {
                    result.push(left_top);
                }
                if !result.contains(&right_bottom) && Self::is_walkable_tile(&board[&right_bottom])
                    || !check_tiles
                {
                    result.push(right_bottom);
                }
                if !result.contains(&right_middle) && Self::is_walkable_tile(&board[&right_middle])
                    || !check_tiles
                {
                    result.push(right_middle);
                }
                if !result.contains(&middle_top) && Self::is_walkable_tile(&board[&middle_top])
                    || !check_tiles
                {
                    result.push(middle_top);
                }
                if !result.contains(&right_top) && Self::is_walkable_tile(&board[&right_top])
                    || !check_tiles
                {
                    result.push(right_top);
                }
            }
            (Ordering::Less, Ordering::Less) => {
                // player is above left
                // ◀▶ 3 5
                // 2 ├┤ 7
                // 4 6  8
                if Self::is_walkable_tile(&board[&left_top]) || !check_tiles {
                    result.push(left_top);
                }

                if !result.contains(&left_middle) && Self::is_walkable_tile(&board[&left_middle])
                    || !check_tiles
                {
                    result.push(left_middle);
                }
                if !result.contains(&middle_top) && Self::is_walkable_tile(&board[&middle_top])
                    || !check_tiles
                {
                    result.push(middle_top);
                }
                if !result.contains(&left_bottom) && Self::is_walkable_tile(&board[&left_bottom])
                    || !check_tiles
                {
                    result.push(left_bottom);
                }
                if !result.contains(&right_top) && Self::is_walkable_tile(&board[&right_top])
                    || !check_tiles
                {
                    result.push(right_top);
                }
                if !result.contains(&middle_bottom)
                    && Self::is_walkable_tile(&board[&middle_bottom])
                    || !check_tiles
                {
                    result.push(middle_bottom);
                }
                if !result.contains(&right_middle) && Self::is_walkable_tile(&board[&right_middle])
                    || !check_tiles
                {
                    result.push(right_middle);
                }
                if !result.contains(&right_bottom) && Self::is_walkable_tile(&board[&right_bottom])
                    || !check_tiles
                {
                    result.push(right_bottom);
                }
            }
            (Ordering::Equal, Ordering::Equal) => {
                // player is at the same position.
                unreachable!("The beast can't be at the same place at the player")
            }
        };

        result
    }

    /// return the Chebyshev distance on a 2D board
    fn heuristic(a: &Coord, b: &Coord) -> i32 {
        let distance_column = (a.column as i32 - b.column as i32).abs();
        let distance_row = (a.row as i32 - b.row as i32).abs();

        distance_column.max(distance_row)
    }

    /// reconstructs the path from start to goal using the came_from map
    fn reconstruct_path(came_from: &HashMap<Coord, Coord>, mut current: Coord) -> Vec<Coord> {
        let mut reconstructed_path = vec![current];
        while let Some(&prev) = came_from.get(&current) {
            current = prev;
            reconstructed_path.push(current);
        }
        reconstructed_path.reverse();

        reconstructed_path
    }
}
