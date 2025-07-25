use std::{collections::HashSet, usize};

struct Solution {}
impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {}
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

// fn solve_graph_coloring<S: FiniteEnumFixedSet>(
//     partial_colors: Vec<GraphColorOption<S>>,
//     graphnodes: &CondensedGraphNodes,
// ) -> Result<Vec<S::FiniteEnumType>, &'static str> {
//     let mut gamestack: Vec<GameState<S>> = Vec::with_capacity(100);
//     let mut gamehead = partial_colors;
//     loop {
//         match eliminate_ambiguity_color_graph(&mut gamehead, graphnodes) {
//             Ok(_) => (),
//             Err(_) => {
//                 let prev_game = match gamestack.pop() {
//                     Some(val) => val,
//                     None => return Err("Ran out of game states in main loop"),
//                 };
//                 // This game state failed to produce any valid game options which lets us know our
//                 // initial guess was wrong:
//                 gamehead = prev_game.board;
//                 let opt = gamehead[prev_game.decision_index].remove(prev_game.decision_color);
//                 // If opt is none, its an indication that the gamestate we generated by trying to
//                 // remove an option is corrupted and we need to pop another gamestate. and set that
//                 // as the gamehead before continuing
//                 continue;
//             }
//         }
//     }
//     // Err("Encountered error and somehow ran out of gamestates")
// }

fn solve_graph_coloring<S: FiniteEnumFixedSet>(
    partial_colors: Vec<GraphColorOption<S>>,
    graphnodes: &CondensedGraphNodes,
) -> Result<Vec<S::FiniteEnumType>, &'static str> {
    type State<S> = GameState<S>;

    let mut gamestack: Vec<State<S>> = Vec::new(); // Stack for backtracking
    let mut gamehead = partial_colors; // Current coloring state

    loop {
        // Propagate constraints: Reduce ambiguities through constraint propagation
        match eliminate_ambiguity_color_graph(&mut gamehead, graphnodes) {
            Ok(_) => {
                // Check if all nodes are solved (fixed to a single color)

                if let Some(result) = is_color_guess_complete(&gamehead) {
                    return Ok(result);
                }
                // Find an ambiguous node to make a guess
                let (guess_index, guess_color) = make_color_guess(&gamehead);

                // Save current state FOR backtracking
                gamestack.push(State {
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
