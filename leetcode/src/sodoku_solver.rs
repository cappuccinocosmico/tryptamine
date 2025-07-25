use std::collections::HashSet;

// Write a program to solve a Sudoku puzzle by filling the empty cells.
//
// A sudoku solution must satisfy all of the following rules:
//
//     Each of the digits 1-9 must occur exactly once in each row.
//     Each of the digits 1-9 must occur exactly once in each column.
//     Each of the digits 1-9 must occur exactly once in each of the 9 3x3 sub-boxes of the grid.
//
// The '.' character indicates empty cells.
//

// Input: board = [["5","3",".",".","7",".",".",".","."],["6",".",".","1","9","5",".",".","."],[".","9","8",".",".",".",".","6","."],["8",".",".",".","6",".",".",".","3"],["4",".",".","8",".","3",".",".","1"],["7",".",".",".","2",".",".",".","6"],[".","6",".",".",".",".","2","8","."],[".",".",".","4","1","9",".",".","5"],[".",".",".",".","8",".",".","7","9"]]
// Output: [["5","3","4","6","7","8","9","1","2"],["6","7","2","1","9","5","3","4","8"],["1","9","8","3","4","2","5","6","7"],["8","5","9","7","6","1","4","2","3"],["4","2","6","8","5","3","7","9","1"],["7","1","3","9","2","4","8","5","6"],["9","6","1","5","3","7","2","8","4"],["2","8","7","4","1","9","6","3","5"],["3","4","5","2","8","6","1","7","9"]]
// Explanation: The input board is shown above and the only valid solution is shown below:
// Could you write a couple adapter functions to conver the characters to and from colors and color
// options. (Use the new from opt color for the color options)
fn convert_char_to_option(c: char) -> Option<SudokuColor> {
    match c {
        '1' => Some(SudokuColor::One),
        '2' => Some(SudokuColor::Two),
        '3' => Some(SudokuColor::Three),
        '4' => Some(SudokuColor::Four),
        '5' => Some(SudokuColor::Five),
        '6' => Some(SudokuColor::Six),
        '7' => Some(SudokuColor::Seven),
        '8' => Some(SudokuColor::Eight),
        '9' => Some(SudokuColor::Nine),
        '.' => None,
        _ => panic!("Invalid Sudoku character: {c}"),
    }
}

fn convert_option_to_char(opt: Option<SudokuColor>) -> char {
    match opt {
        Some(SudokuColor::One) => '1',
        Some(SudokuColor::Two) => '2',
        Some(SudokuColor::Three) => '3',
        Some(SudokuColor::Four) => '4',
        Some(SudokuColor::Five) => '5',
        Some(SudokuColor::Six) => '6',
        Some(SudokuColor::Seven) => '7',
        Some(SudokuColor::Eight) => '8',
        Some(SudokuColor::Nine) => '9',
        None => '.',
    }
}

// Convert entire Sudoku board (9x9 chars) to a flat vector of GraphColorOptions
fn board_to_color_options(board: &[Vec<char>]) -> Vec<GraphColorOption<FiniteSodokuColorSet>> {
    board
        .iter()
        .flatten()
        .map(|&c| GraphColorOption::new_from_opt_color(convert_char_to_option(c)))
        .collect()
}

// Convert solved color results (Vec<SudokuColor>) back to 9x9 char board
fn color_vec_to_board(colors: Vec<SudokuColor>) -> Vec<Vec<char>> {
    colors
        .chunks(9)
        .map(|chunk| {
            chunk
                .iter()
                .map(|&color| convert_option_to_char(Some(color)))
                .collect()
        })
        .collect()
}

// Implement solver using these adapters
struct Solution {}
impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        // Generate Sudoku constraint graph
        let graph_nodes = generate_sudoku_graph_nodes();

        // Convert input board to color options
        let color_options = board_to_color_options(board);

        // Solve using graph coloring algorithm
        let result =
            solve_graph_coloring(color_options, &graph_nodes).expect("Sudoku has no solution");

        // Convert solution back to board format
        let solved_board = color_vec_to_board(result);

        // Update original board in-place
        *board = solved_board;
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SudokuColor {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

pub trait FiniteEnum: Copy + Sized + Eq {
    const QUANTITY: usize;
    fn list_all() -> Vec<Self>;
    fn index_lookup(index: usize) -> Self;
    fn get_index(&self) -> usize;
}

trait FiniteEnumFixedSet: Copy + Sized {
    type FiniteEnumType: FiniteEnum;
    fn set(&mut self, val: Self::FiniteEnumType) -> Option<Self::FiniteEnumType>;
    fn remove(&mut self, val: Self::FiniteEnumType) -> Option<Self::FiniteEnumType>;
    fn contains(&self, val: Self::FiniteEnumType) -> bool;
    fn len(&self) -> usize;
    fn into_vec(self) -> Vec<Self::FiniteEnumType>;
    fn new_full() -> Self;
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct FiniteSodokuColorSet {
    raw_inner: [bool; 9],
}
impl FiniteEnumFixedSet for FiniteSodokuColorSet {
    type FiniteEnumType = SudokuColor;
    fn set(&mut self, val: SudokuColor) -> Option<SudokuColor> {
        let prev = self.contains(val).then_some(val);
        self.raw_inner[val.get_index()] = true;
        prev
    }
    fn remove(&mut self, val: SudokuColor) -> Option<SudokuColor> {
        let prev = self.contains(val).then_some(val);
        self.raw_inner[val.get_index()] = false;
        prev
    }
    fn contains(&self, val: SudokuColor) -> bool {
        self.raw_inner[val.get_index()]
    }
    fn len(&self) -> usize {
        let mut collector = 0;
        for val in self.raw_inner {
            collector += val as usize
        }
        collector
    }
    fn into_vec(self) -> Vec<SudokuColor> {
        let mut collector = Vec::new();
        for (index, val) in self.raw_inner.iter().enumerate() {
            if *val {
                collector.push(SudokuColor::index_lookup(index));
            }
        }
        collector
    }
    fn new_full() -> Self {
        FiniteSodokuColorSet {
            raw_inner: [true; 9],
        }
    }
}

impl FiniteEnum for SudokuColor {
    fn index_lookup(index: usize) -> Self {
        match index {
            0 => SudokuColor::One,
            1 => SudokuColor::Two,
            2 => SudokuColor::Three,
            3 => SudokuColor::Four,
            4 => SudokuColor::Five,
            5 => SudokuColor::Six,
            6 => SudokuColor::Seven,
            7 => SudokuColor::Eight,
            8 => SudokuColor::Nine,
            _ => panic!("Invalid index for SudokuColors"),
        }
    }
    fn get_index(&self) -> usize {
        match self {
            SudokuColor::One => 0,
            SudokuColor::Two => 1,
            SudokuColor::Three => 2,
            SudokuColor::Four => 3,
            SudokuColor::Five => 4,
            SudokuColor::Six => 5,
            SudokuColor::Seven => 6,
            SudokuColor::Eight => 7,
            SudokuColor::Nine => 8,
        }
    }
    const QUANTITY: usize = 9;
    fn list_all() -> Vec<Self>
    where
        Self: Sized,
    {
        vec![
            SudokuColor::One,
            SudokuColor::Two,
            SudokuColor::Three,
            SudokuColor::Four,
            SudokuColor::Five,
            SudokuColor::Six,
            SudokuColor::Seven,
            SudokuColor::Eight,
            SudokuColor::Nine,
        ]
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum GraphColorOption<S: FiniteEnumFixedSet> {
    Fixed(S::FiniteEnumType),
    Variable(S),
}
impl<S: FiniteEnumFixedSet> GraphColorOption<S> {}

impl<S: FiniteEnumFixedSet> GraphColorOption<S> {
    fn len(&self) -> usize {
        match self {
            Self::Fixed(_) => 1,
            Self::Variable(val) => val.len(),
        }
    }
    fn new_from_opt_color(c: Option<S::FiniteEnumType>) -> GraphColorOption<S> {
        match c {
            Some(color) => GraphColorOption::Fixed(color),
            None => GraphColorOption::Variable(S::new_full()),
        }
    }
    fn new_from_enumset(set: S) -> Option<GraphColorOption<S>> {
        match set.len() {
            0 => None,
            1 => Some(Self::Fixed(set.into_vec()[0])),
            _ => Some(Self::Variable(set)),
        }
    }

    fn remove(&mut self, val: S::FiniteEnumType) -> Option<S::FiniteEnumType> {
        match self {
            Self::Fixed(_) => None,
            Self::Variable(set) => {
                set.remove(val);
                let newval = Self::new_from_enumset(*set)?;
                *self = newval;
                Some(val)
            }
        }
    }
}

struct CondensedGraphNodes {
    size: usize,
    graph: Vec<Vec<usize>>,
}

fn generate_sudoku_graph_nodes() -> CondensedGraphNodes {
    // This should return a condensed graph node of 81 size, and it should represent the grid as  and then convert solving a sudoku to a graph coloring problem, with nine possible graph colors.
    // For example, the fact that the position (0,0) must have a different digit on the first row and the first column, means that it would be connected to the nodes:
    // (0,1), (0,2), (0,3), (0,4), (0,5), (0,6), (0,7), (0,8)
    // and
    // (1,0), (2,0), (3,0), (4,0), (5,0), (6,0), (7,0), (8,0).
    // and all the values in a square, namely:
    // (0,1), (0,2), (1,0), (1,1), (1,2), (2,0), (2,1), (2,2)
    //
    // And the values for this are condensed, so (x,y) -> 9*x+y.
    // (Also no graph nodes are connected to themselves, and also the list of all graph nodes
    // should be deduplicated and sorted.)
    let mut graph = Vec::with_capacity(81);

    for i in 0..81 {
        let row = i / 9;
        let col = i % 9;
        let box_r_start = (row / 3) * 3;
        let box_c_start = (col / 3) * 3;

        let mut neighbors_set = HashSet::with_capacity(20);

        for c in 0..9 {
            if c != col {
                let neighbor_idx = row * 9 + c;
                neighbors_set.insert(neighbor_idx);
            }
        }

        for r in 0..9 {
            if r != row {
                let neighbor_idx = r * 9 + col;
                neighbors_set.insert(neighbor_idx);
            }
        }

        for dr in 0..3 {
            for dc in 0..3 {
                let r_idx = box_r_start + dr;
                let c_idx = box_c_start + dc;
                if r_idx != row || c_idx != col {
                    let neighbor_idx = r_idx * 9 + c_idx;
                    neighbors_set.insert(neighbor_idx);
                }
            }
        }

        let mut neighbors_vec: Vec<usize> = neighbors_set.into_iter().collect();
        neighbors_vec.sort_unstable();
        graph.push(neighbors_vec);
    }

    CondensedGraphNodes { size: 81, graph }
}

fn eliminate_ambiguity_color_graph<T: FiniteEnumFixedSet>(
    options: &mut [GraphColorOption<T>],
    graphnodes: &CondensedGraphNodes,
) -> Result<(), &'static str> {
    let edges_list = &*graphnodes.graph;
    let mut no_more_work = true;
    while no_more_work {
        for (index, edges) in edges_list.iter().enumerate() {
            let mut color_options = match options[index] {
                GraphColorOption::Fixed(_) => {
                    continue;
                }
                GraphColorOption::Variable(val) => val,
            };
            let mut changed_edges = false;
            for edge_index in edges {
                if let GraphColorOption::Fixed(color) = options[*edge_index] {
                    // If color is in color_options then remove it from the list, and then set
                    // no_more_work equal to false.
                    let opt = color_options.remove(color);
                    if opt.is_some() {
                        changed_edges = true;
                    }
                }
            }
            if changed_edges {
                options[index] = match GraphColorOption::new_from_enumset(color_options) {
                    Some(val) => val,
                    None => return Err("One solve instance had zero solutions."),
                };
                no_more_work = true;
            }
        }
    }
    Ok(())
}

fn is_color_guess_complete<S: FiniteEnumFixedSet>(
    partial_colors: &[GraphColorOption<S>],
) -> Option<Vec<S::FiniteEnumType>> {
    if partial_colors
        .iter()
        .all(|color| matches!(color, GraphColorOption::Fixed(_)))
    {
        let mut result = Vec::with_capacity(partial_colors.len());
        for color in partial_colors {
            if let GraphColorOption::Fixed(val) = color {
                result.push(*val);
            }
        }
        Some(result)
    } else {
        None
    }
}

fn make_color_guess<S: FiniteEnumFixedSet>(
    partial_colors: &[GraphColorOption<S>],
) -> (usize, S::FiniteEnumType) {
    let mut best_index = 0;
    let mut best_len = partial_colors[0].len();
    for (index, color) in partial_colors.iter().enumerate() {
        let len = color.len();
        if len < best_len && len >= 2 {
            best_index = index;
            best_len = len;
        }
    }
    let color = match partial_colors[best_index] {
        GraphColorOption::Fixed(_) => unreachable!(),
        GraphColorOption::Variable(set) => set.into_vec()[0],
    };
    (best_index, color)
}

struct GameState<S: FiniteEnumFixedSet> {
    board: Vec<GraphColorOption<S>>,
    decision_index: usize,
    decision_color: S::FiniteEnumType,
}

fn solve_graph_coloring<S: FiniteEnumFixedSet>(
    partial_colors: Vec<GraphColorOption<S>>,
    graphnodes: &CondensedGraphNodes,
) -> Result<Vec<S::FiniteEnumType>, &'static str> {
    let mut gamestack: Vec<GameState<S>> = Vec::new(); // Stack for backtracking
    let mut gamehead = partial_colors; // Current coloring state

    loop {
        match eliminate_ambiguity_color_graph(&mut gamehead, graphnodes) {
            Ok(_) => {
                if let Some(result) = is_color_guess_complete(&gamehead) {
                    return Ok(result);
                }
                // Find an ambiguous node to make a guess
                let (guess_index, guess_color) = make_color_guess(&gamehead);

                gamestack.push(GameState {
                    board: gamehead.clone(), // Snapshot of entire state
                    decision_index: guess_index,
                    decision_color: guess_color,
                });

                // Make a guess: Fix this node to chosen_color
                gamehead[guess_index] = GraphColorOption::Fixed(guess_color);
            }

            Err(_) => {
                // Backtrack: Restore previous state and remove failed choice
                let prev_state = gamestack.pop().ok_or("No states left to backtrack")?;
                gamehead = prev_state.board;

                // Eliminate last guessed color from possibilities
                if gamehead[prev_state.decision_index]
                    .remove(prev_state.decision_color)
                    .is_none()
                {
                    return Err(
                        "Got into unreachable code path, it probably indicates one of the fields got horrifically overconstrained and the graph is impossible to solve.",
                    );
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{sync::mpsc, thread, time::Instant};

    #[test]
    fn solve_sudoku_example() {
        let mut board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        let expected_solution = vec![
            vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
            vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
            vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
            vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
            vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
            vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
            vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
            vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
            vec!['3', '4', '5', '2', '8', '6', '1', '7', '9'],
        ];

        let (tx, rx) = mpsc::channel();

        let handle = thread::spawn(move || {
            let now = Instant::now();
            Solution::solve_sudoku(&mut board);
            let elapsed = now.elapsed();
            tx.send((board, elapsed)).unwrap();
        });

        let result = rx.recv_timeout(std::time::Duration::from_secs(15));

        match result {
            Ok((solved_board, elapsed)) => {
                println!("Sudoku solved in: {elapsed:.2?}");
                assert_eq!(solved_board, expected_solution);
            }
            Err(_) => {
                handle
                    .join()
                    .unwrap_or_else(|_| panic!("Sudoku solver timed out after 15 seconds"));
                panic!("Sudoku solver timed out after 15 seconds");
            }
        }
    }
}
