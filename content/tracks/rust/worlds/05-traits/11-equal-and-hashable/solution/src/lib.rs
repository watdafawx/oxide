#[allow(unused_imports)]
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cell {
    pub col: i32,
    pub row: i32,
}

/// How many different cells the brush touched.
pub fn distinct_cells(cells: &[Cell]) -> usize {
    cells.iter().copied().collect::<HashSet<Cell>>().len()
}

/// How many times each cell was painted.
pub fn paint_counts(strokes: &[Cell]) -> HashMap<Cell, u32> {
    let mut counts = HashMap::new();
    for c in strokes {
        *counts.entry(*c).or_insert(0) += 1;
    }
    counts
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
