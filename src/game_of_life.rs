use crate::{GAME_TOTAL, GAME_H, GAME_W};

type Cells = Vec<Cell>;

pub struct World {
    cells: Cells,
}

impl World {
    pub fn init() -> Self {
        World {
            cells:
                (0..GAME_TOTAL)
                    .map(|i| Cell::new(i))
                    .collect::<Vec<Cell>>()
        }
    }

    pub fn is_cell_alive(&self, idx: usize) -> bool {
        self.cells[idx].state == CellState::Alive
    }

    pub fn toggle_cell(&mut self, cell_index: usize) {
        self.cells[cell_index].toggle();
    }

    pub fn update(&mut self) {
        // Clone the state
        let mut new_state: Cells = self.cells.clone();

        // Apply Conway's game of life rules:
        new_state
            .iter_mut()
            .for_each(|c| {
                let alive_neibs = self.cells[c.idx].get_num_neighbours_alive(&self.cells);
                c.state =
                    
                    if !c.is_alive() && alive_neibs == 3 {
                        CellState::Alive // if dead and 3 alive neibs -> alive
                    } else if c.is_alive() && alive_neibs >= 2 && alive_neibs <= 3 {
                        CellState::Alive // if alive and 2 or 3 alive neibs -> alive
                    } else {
                        CellState::Dead // Otherwise -> dead
                    };
            });
        
        self.cells = new_state;
    }
}


#[derive(Clone)]
pub struct Cell {
    /// Index of the cell within its world
    idx: usize,
    /// State of the cell, Alive or Dead
    state: CellState,
    /// All indices of valid neighbours cells: [N, NE, E, SE, S, SW, W, NW]
    neighbours_idx: [Option<usize>; 8],
}

impl Cell {
    pub fn new(idx: usize) -> Self {
        Cell {
            idx,
            state: CellState::Dead,
            neighbours_idx: {

                // 2D cell coords
                let x = idx % GAME_H;
                let y = idx / GAME_W;
                
                let l_side = x == 0; // is cell on left side ?
                let r_side = x == GAME_W - 1; // is cell on right side ?
                let t_side = y == 0; // is cell on top side ?
                let b_side = y == GAME_H - 1; // is cell on botom side ?

                // fill neighbours array
                [
                    (!t_side).then(|| idx - GAME_H),                    // N
                    (!t_side && !r_side).then(|| idx - GAME_H + 1),     // NE
                    (!r_side).then(|| idx + 1),                         // E
                    (!b_side && !r_side).then(|| idx + GAME_H + 1),     // SE
                    (!b_side).then(|| idx + GAME_H),                    // S
                    (!l_side && !b_side).then(|| idx + GAME_H - 1),     // SW
                    (!l_side).then(|| idx - 1),                         // W
                    (!t_side && !l_side).then(|| idx - GAME_H - 1),     // NW
                ]
            },
        }
    }

    pub fn toggle(&mut self) {
        self.state = match self.state {
            CellState::Dead => CellState::Alive,
            CellState::Alive => CellState::Dead,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.state == CellState::Alive
    }

    pub fn get_num_neighbours_alive(&self, world_state: &Cells) -> usize {
        let mut alive_count: usize = 0;

        for idx in self.neighbours_idx {
            if let Some(idx) = idx {
                if world_state[idx].is_alive() {
                    alive_count += 1;
                }

            }
        }

        alive_count
    }
}


#[derive(PartialEq, Clone)]
pub enum CellState {
    Dead,
    Alive,
}