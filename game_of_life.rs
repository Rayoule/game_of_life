use crate::{GAME_TOTAL, GAME_H, GAME_W};

type Cells = [Cell; GAME_TOTAL];

pub struct World {
    cells: Cells,
}

impl World {
    pub fn init() -> Self {
        World { cells: core::array::from_fn(|i| Cell::new(i)) }
    }

    pub fn is_cell_alive(&self, idx: usize) -> bool {
        self.cells[idx].state == CellState::Alive
    }

    pub fn set_cell(&mut self, cell_index: usize, set_alive: bool) {
        let new_cell_state = if set_alive { CellState::Alive } else { CellState::Dead };
        self.cells[cell_index].state = new_cell_state;
    }

    pub fn update(&mut self) {
        println!("Lets updaaaate !");
        let mut new_state: Cells = self.cells.clone();

        println!("Ok copied !");

        // Apply Conway's game of life rules:
        new_state
            .iter_mut()
            .for_each(|c| {
                let alive_neibs = self.cells[c.id].get_num_neighbours_alive(&self.cells);
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
    id: usize,
    state: CellState,
}

impl Cell {
    pub fn new(id: usize) -> Self {
        Cell {
            state: CellState::Dead,
            id,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.state == CellState::Alive
    }

    pub fn get_num_neighbours_alive(&self, world_state: &Cells) -> usize {
        let i = self.id;
        let mut alive_count: usize = 0;

        let x = i % GAME_H;
        let y = i / GAME_W;

        // is cell on left side ?
        let l_side = x == 0;
        // is cell on right side ?
        let r_side = x == GAME_W - 1;
        // is cell on top side ?
        let t_side = y == 0;
        // is cell on botom side ?
        let b_side = y == GAME_H - 1;

        // Check all neighbours and increment alive_count
        (!t_side).then(|| world_state[i - GAME_H].is_alive().then(|| alive_count += 1));                      // N
        (!t_side && !r_side).then(|| world_state[i - GAME_H + 1].is_alive().then(|| alive_count += 1));       // NE
        (!r_side).then(|| world_state[i + 1].is_alive().then(|| alive_count += 1));                           // E
        (!b_side && !r_side).then(|| world_state[i + GAME_H + 1].is_alive().then(|| alive_count += 1));       // SE
        (!b_side).then(|| world_state[i + GAME_H].is_alive().then(|| alive_count += 1));                      // S
        (!l_side && !b_side).then(|| world_state[i + GAME_H - 1].is_alive().then(|| alive_count += 1));        // SW
        (!l_side).then(|| world_state[i - 1].is_alive().then(|| alive_count += 1));                           // W
        (!t_side && !l_side).then(|| world_state[ i - GAME_H - 1].is_alive().then(|| alive_count += 1));      // NW

        alive_count
    }
}


#[derive(PartialEq, Clone)]
pub enum CellState {
    Dead,
    Alive,
}