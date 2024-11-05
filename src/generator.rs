extern crate nalgebra;
use std::collections::HashMap;
use std::collections::HashSet;

use nalgebra::{ArrayStorage, Const, Matrix, Vector2};

use crate::nonogramSolver::State as StateSolver;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Move{
    pub pos : Vector2<i32>,
    pub size : Vector2<i32>,
    //pub len : usize
}


#[derive(Clone)]
pub struct State{
    pub sizeX : i32,
    pub sizeY : i32,
    pub optimizationMode : usize,
    pub lattice : HashSet<Vector2<i32>>,
    pub seq : Vec<Move>,
    pub reached_best_score : bool,
}

impl State{
    pub const CONSIDER_NON_TERM: bool = false;

    pub fn new() -> Self {
        Self{
            sizeX : 10,
            sizeY : 10,
            optimizationMode : 0,
            lattice: HashSet::new(),
            seq : Vec::new(),
            reached_best_score : false
        }
    }

    pub fn play(&mut self, m : Move){

        for i in 0..m.size.x {
            for j in 0..m.size.y {
                let pt = Vector2::new(i, j);
                if !self.lattice.contains(&pt) {
                    self.lattice.insert(pt);
                }else{
                    self.lattice.remove(&pt);
                }
            }
        }


        self.seq.push(m);
    }

    pub fn legal_moves(& self) -> Vec<Move>{
        let mut vec :Vec<Move> = Vec::new();

        for i in 0..self.sizeX+1 {
            for j in 0 .. self.sizeY+1 {
                for i2 in 0..self.sizeX-i+1 {
                    for j2 in 0..self.sizeY-j+1 {
                        let m = Move{pos : Vector2::new(i, j), size : Vector2::new(i2, j2) };
                        vec.push(m);
                    }
                }
            }
        }

        return vec;
    }

    pub fn terminal(& self) -> bool{
        return self.seq.len() >= 30;
    }

    pub fn score(& self) -> f64 {

        let mut st = StateSolver::new_from_hashset(self.lattice.clone());
        let sol = st.solve(false);
        //println!("fzsd {}", sol[self.optimizationMode]);
        return sol[self.optimizationMode] ; //0 time, 1 difficulty estimate, 2 fun estimate
    }

    pub fn display(&self){
        let st = StateSolver::new_from_hashset(self.lattice.clone());
        st.solve(true);
    }

    pub fn heuristic(& self, m : Move) -> f64{
        return 0.0;
    }

    pub fn smoothedScore(&self) ->f64{
        return self.score();
    }
}