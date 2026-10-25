#[allow(unused_imports)]
use std::collections::{HashMap, HashSet};

// TODO: when Rust asks for more traits, add them to this derive line
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell {
    pub col: i32,
    pub row: i32,
}

/// How many different cells the brush touched.
pub fn distinct_cells(cells: &[Cell]) -> usize {
    // TODO: collect the cells into a HashSet<Cell> and return how many it holds
    cells.len()
}

/// How many times each cell was painted.
pub fn paint_counts(strokes: &[Cell]) -> HashMap<Cell, u32> {
    // TODO: for each cell, add one to its count with counts.entry(..).or_insert(0)
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cell(col: i32, row: i32) -> Cell {
        Cell { col, row }
    }

    #[test]
    fn distinct_cells_counts_each_cell_once() {
        let brush = [cell(0, 0), cell(1, 0), cell(0, 0), cell(1, 0), cell(-2, 3)];
        assert_eq!(distinct_cells(&brush), 3);
        assert_eq!(distinct_cells(&[]), 0);
    }

    #[test]
    fn paint_counts_tallies_each_cell() {
        let counts: Vec<(Cell, u32)> = paint_counts(&[cell(0, 0), cell(1, 0), cell(0, 0)]).into_iter().collect();
        assert_eq!(counts.len(), 2, "two different cells were painted");
        assert!(counts.contains(&(cell(0, 0), 2)), "(0, 0) was painted twice");
        assert!(counts.contains(&(cell(1, 0), 1)), "(1, 0) was painted once");
    }
}
