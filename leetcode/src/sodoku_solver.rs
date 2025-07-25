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
    fn set(&mut self, val: Self::FiniteEnumType);
    fn remove(&mut self, val: Self::FiniteEnumType);
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
    fn set(&mut self, val: SudokuColor) {
        self.raw_inner[val.get_index()] = true
    }
    fn remove(&mut self, val: SudokuColor) {
        self.raw_inner[val.get_index()] = false
    }
    fn contains(&self, val: SudokuColor) -> bool {
        self.raw_inner[val.get_index()]
    }
    fn len(&self) -> usize {
        let mut collector = 0;
        for val in self.raw_inner {
            collector = collector + val as usize
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

impl<T: FiniteEnumFixedSet> GraphColorOption<T> {
    fn len(&self) -> usize {
        match self {
            Self::Fixed(_) => 0,
            Self::Variable(val) => val.len(),
        }
    }
    fn new_from_enumset(set: T) -> Option<GraphColorOption<T>> {
        match set.len() {
            0 => None,
            1 => Some(Self::Fixed(set.into_vec()[0])),
            _ => Some(Self::Variable(set)),
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
    todo!()
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
                    color_options.remove(color);
                    changed_edges = true;
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
