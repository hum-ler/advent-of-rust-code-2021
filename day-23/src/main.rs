use anyhow::{Result, anyhow};
use pathfinding::prelude::dijkstra;

use cli::{Part, get_part};

fn main() {
    match get_part("inputs/day-23.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{error:?}"),
    }
}

fn part_1(input: &str) -> Result<u32> {
    let start = parse_input_into_node(input);
    let end = Node {
        rooms: [
            [b'A', b'A', 0, 0],
            [b'B', b'B', 0, 0],
            [b'C', b'C', 0, 0],
            [b'D', b'D', 0, 0],
        ],
        ..Default::default()
    };

    let Some((_, cost)) = dijkstra(&start, successors, |node| *node == end) else {
        return Err(anyhow!("Cannot find cheapest path"));
    };

    Ok(cost)
}

fn part_2(input: &str) -> Result<u32> {
    let start = extend_rooms(parse_input_into_node(input));
    let end = Node {
        room_len: 4,
        rooms: [
            [b'A', b'A', b'A', b'A'],
            [b'B', b'B', b'B', b'B'],
            [b'C', b'C', b'C', b'C'],
            [b'D', b'D', b'D', b'D'],
        ],
        ..Default::default()
    };

    let Some((_, cost)) = dijkstra(&start, successors, |node| *node == end) else {
        return Err(anyhow!("Cannot find cheapest path"));
    };

    Ok(cost)
}

// #############
// #01234567890#
// ###3#3#3#3###
//   #2#2#2#2#
//   #1#1#1#1#
//   #0#0#0#0#
//   #########
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Node {
    room_len: usize,
    corridor: [u8; 11],
    rooms: [[u8; 4]; 4],
}

impl Default for Node {
    fn default() -> Self {
        Self {
            room_len: 2,
            corridor: Default::default(),
            rooms: Default::default(),
        }
    }
}

impl Node {
    fn can_pop(&self, room_index: usize) -> bool {
        // Can pop if we are not in pushing condition i.e. the stack is not all either 0 or the
        // correct amphipod.

        // 0, 0, 0, 0
        // a, 0, 0, 0
        // a, a, 0, 0
        // a, a, a, 0
        // a, a, a, a
        !(0..self.room_len).all(|stack_index| {
            self.rooms[room_index][stack_index] == 0
                || self.rooms[room_index][stack_index] == b'A' + room_index as u8
        })
    }

    fn can_push(&self, room_index: usize) -> bool {
        // Can push if there is space at the the top of the stack, and the rest are all either 0 or
        // the correct amphipod.

        // 0, 0, 0, 0
        // a, 0, 0, 0
        // a, a, 0, 0
        // a, a, a, 0
        self.rooms[room_index][self.room_len - 1] == 0
            && (0..self.room_len - 1).all(|stack_index| {
                self.rooms[room_index][stack_index] == 0
                    || self.rooms[room_index][stack_index] == b'A' + room_index as u8
            })
    }

    /// Creates the successor [Node] and the associated cost if popping the room stack and moving
    /// the amphipod along the corridor is permitted.
    fn pop(&self, room_index: usize, corridor_index: usize) -> Option<(Node, u32)> {
        let room_exit = Self::room_exit(room_index)?;

        if self.can_pop(room_index) && self.can_move_along_corridor(room_exit, corridor_index) {
            let amphipod_index = self.rooms[room_index].iter().rposition(|byte| *byte != 0)?;
            let room_steps = self.room_len - amphipod_index;
            let corridor_steps = room_exit.abs_diff(corridor_index);
            let step_cost = amphipod_step_cost(self.rooms[room_index][amphipod_index])?;

            let mut node = *self;
            node.corridor[corridor_index] = self.rooms[room_index][amphipod_index];
            node.rooms[room_index][amphipod_index] = 0;

            return Some((node, step_cost * (room_steps + corridor_steps) as u32));
        }

        None
    }

    /// Creates the successor [Node] and the associated cost if amphipod can be moved along the
    /// corridor and pushed into the room stack.
    fn push(&self, room_index: usize, corridor_index: usize) -> Option<(Node, u32)> {
        let room_exit = Self::room_exit(room_index)?;

        // Can only push into the correct room.
        if self.corridor[corridor_index] != b'A' + room_index as u8 {
            return None;
        }

        if self.can_push(room_index) && self.can_move_along_corridor(corridor_index, room_exit) {
            let amphipod_index = self.rooms[room_index].iter().position(|byte| *byte == 0)?;
            let room_steps = self.room_len - amphipod_index;
            let corrider_steps = corridor_index.abs_diff(room_exit);
            let step_cost = amphipod_step_cost(self.corridor[corridor_index])?;

            let mut node = *self;
            node.rooms[room_index][amphipod_index] = self.corridor[corridor_index];
            node.corridor[corridor_index] = 0;

            return Some((node, step_cost * (room_steps + corrider_steps) as u32));
        }

        None
    }

    /// Checks for obstruction along the corridor.
    fn can_move_along_corridor(&self, from: usize, to: usize) -> bool {
        if from < to {
            from + 1..=to
        } else {
            to..=from - 1
        }
        .all(|index| self.corridor[index] == 0)
    }

    /// Gets the position along the corridor where a room exits.
    fn room_exit(room_index: usize) -> Option<usize> {
        match room_index {
            0 => Some(2),
            1 => Some(4),
            2 => Some(6),
            3 => Some(8),
            _ => None,
        }
    }

    /// Gets the list of positions where amphipods can be placed.
    fn stoppable_corrider_indices() -> [usize; 7] {
        [0, 1, 3, 5, 7, 9, 10]
    }
}

fn amphipod_step_cost(amphipod: u8) -> Option<u32> {
    match amphipod {
        b'A' => Some(1),
        b'B' => Some(10),
        b'C' => Some(100),
        b'D' => Some(1000),
        _ => None,
    }
}

fn parse_input_into_node(input: &str) -> Node {
    let mut node = Node::default();

    let room_lines = input
        .lines()
        .skip(2)
        .take(2)
        .map(|line| line.trim().trim_start_matches("#").trim_end_matches("#"))
        .map(|line| {
            line.bytes()
                .filter(|byte| *byte != b'#')
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    node.rooms[0][1] = room_lines[0][0];
    node.rooms[0][0] = room_lines[1][0];
    node.rooms[1][1] = room_lines[0][1];
    node.rooms[1][0] = room_lines[1][1];
    node.rooms[2][1] = room_lines[0][2];
    node.rooms[2][0] = room_lines[1][2];
    node.rooms[3][1] = room_lines[0][3];
    node.rooms[3][0] = room_lines[1][3];

    node
}

/// Gets the possible successor [Node]s and their associated cost.
fn successors(node: &Node) -> Vec<(Node, u32)> {
    let mut successors = Vec::new();

    for corridor_index in Node::stoppable_corrider_indices() {
        for room_index in 0..4 {
            if node.corridor[corridor_index] == 0 {
                // Empty space along corridor, check if we can pop the room.
                if let Some(successor) = node.pop(room_index, corridor_index) {
                    successors.push(successor);
                }
            }

            if node.corridor[corridor_index] != 0 {
                // Filled space along corridor, check if we can push into the room.
                if let Some(successor) = node.push(room_index, corridor_index) {
                    successors.push(successor);
                }
            }
        }
    }

    successors
}

/// Extends the rooms for part 2.
fn extend_rooms(mut node: Node) -> Node {
    node.room_len = 4;

    node.rooms[0][3] = node.rooms[0][1];
    node.rooms[1][3] = node.rooms[1][1];
    node.rooms[2][3] = node.rooms[2][1];
    node.rooms[3][3] = node.rooms[3][1];

    node.rooms[0][1] = b'D';
    node.rooms[0][2] = b'D';

    node.rooms[1][1] = b'B';
    node.rooms[1][2] = b'C';

    node.rooms[2][1] = b'A';
    node.rooms[2][2] = b'B';

    node.rooms[3][1] = b'C';
    node.rooms[3][2] = b'A';

    node
}

#[cfg(test)]
mod tests {
    use cli::trim_newlines;

    use super::*;

    const EXAMPLE_INPUT: &str = r"
#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(trim_newlines(EXAMPLE_INPUT))?, 12521);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(trim_newlines(EXAMPLE_INPUT))?, 44169);

        Ok(())
    }
}
