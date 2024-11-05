
extern crate nalgebra;

use std::cmp;
use std::collections::HashSet;
use std::ops::Add;

use std::fs::File;
use std::time::Instant;
use nalgebra::{DimAdd, Vector2};
use average::{MeanWithError, Kurtosis, Estimate};


#[derive(Clone)]
pub struct State{
    pub grid : Vec<Vec<f64>>,
    pub indicH : Vec<Vec<usize>>,
    pub indicV : Vec<Vec<usize>>,
    pub seq : Vec<Move>,
    pub colToCheck : Vec<usize>,
    pub lineToCheck : Vec<usize>,
    pub filled : usize,
    pub guessing : bool,
    pub discardedMoves : Vec<usize>,
    pub nGuessing : usize

}

impl State{
    pub const CONSIDER_NON_TERM: bool = false;
    pub const RETURN_FIRST_LEGAL_MOVE: bool = true; //retourne le premier coup trouvé
    pub const USE_PARTIAL_CHECK: bool = true; //check en priorité les dernières ligne modifiées (stack), utile avec RETURN_FIRST_LEGAL_MOVE
    pub const VERBOSE: bool = true;

    //0: jsp
    //1 blanc
    //-1 noir
    pub fn new_from_png(name : String) -> Self {


        // The decoder is a build for reader and can be used to set various decoding options
        // via `Transformations`. The default output transformation is `Transformations::IDENTITY`.
        //let decoder = png::Decoder::new(File::open("nonograms/100Stone.png").unwrap());
        let decoder = png::Decoder::new(File::open(name).unwrap());
        let mut reader = decoder.read_info().unwrap();
        // Allocate the output buffer.
        let mut buf = vec![0; reader.output_buffer_size()];
        // Read the next frame. An APNG might contain multiple frames.
        let info = reader.next_frame(&mut buf).unwrap();

        //println!("{:?}", info);
        println!("{:?}", buf);

        let w = info.width as usize;
        let h = info.height as usize;

        let mut grid = vec![];
        for i in 0..h {
            grid.push(vec![0.0; w]);
        }

        let mut sprite = grid.clone();

        for i in 0..h{
            for j in 0..w{
                sprite[i][j] = ((buf[i * w * 4 + j * 4] as usize+buf[i * w * 4 + j * 4 +1]as usize+buf[i * w * 4 + j * 4 +2]as usize+buf[i * w * 4 + j * 4 +3]as usize)/1020) as f64;
            }
        }

        let mut indicH = vec![];
        let mut lineToCheck = vec![];
        for i in 0..h{
            lineToCheck.push(i);
            indicH.push(vec![]);
            let mut ind = 0;
            for j in 0..w{
                if sprite[i][j] == 1.0{
                    ind+=1;
                }else{
                    if ind != 0{
                        indicH[i].push(ind);
                        ind = 0;
                    }
                }
            }
            if ind != 0{
                indicH[i].push(ind);
                ind  = 0;
            }
            if indicH[i].len() == 0 {
                indicH[i].push(0);
            }
        }


        let mut indicV = vec![];
        let mut colToCheck = vec![];
        for i in 0..w{
            colToCheck.push(i);
            indicV.push(vec![]);
            let mut ind = 0;
            for j in 0..h{
                if sprite[j][i] == 1.0{
                    ind+=1;
                }else{
                    if ind != 0 {
                        indicV[i].push(ind);
                        ind = 0;
                    }
                }
            }
            if ind != 0{
                indicV[i].push(ind);
                ind  = 0;
            }

            if indicV[i].len() == 0 {
                indicV[i].push(0);
            }
        }


        if false {
            println!("{:?}", sprite);
            println!("{:?}", indicH);
            println!("{:?}", indicV);
        }



        Self{ grid: grid, indicH : indicH, indicV : indicV, seq: vec![], colToCheck: colToCheck, lineToCheck: lineToCheck, filled : 0, guessing : true, discardedMoves: vec![], nGuessing : 0 }
    }

    pub fn new_from_hashset(hash : HashSet<(Vector2<i32>)>) -> Self {


        let mut w:usize  = 0;
        let mut h:usize = 0;

        for e in hash.iter(){
            if e.x > w as i32 {
                w = e.x as usize;
            }
            if e.y > h as i32 {
                h = e.y as usize;
            }
        }
        w +=1;
        h+=1;

        let mut grid = vec![];
        for i in 0..h {
            grid.push(vec![0.0; w as usize]);
        }

        let mut sprite = grid.clone();

        for e in hash.iter(){
            sprite[e.y as usize][e.x as usize] = 1.0;
        }


        let mut indicH = vec![];
        let mut lineToCheck = vec![];
        for i in 0..h{
            lineToCheck.push(i);
            indicH.push(vec![]);
            let mut ind = 0;
            for j in 0..w{
                if sprite[i][j] == 1.0{
                    ind+=1;
                }else{
                    if ind != 0{
                        indicH[i].push(ind);
                        ind = 0;
                    }
                }
            }
            if ind != 0{
                indicH[i].push(ind);
                ind  = 0;
            }
            if indicH[i].len() == 0 {
                indicH[i].push(0);
            }
        }


        let mut indicV = vec![];
        let mut colToCheck = vec![];
        for i in 0..w{
            colToCheck.push(i);
            indicV.push(vec![]);
            let mut ind = 0;
            for j in 0..h{
                if sprite[j][i] == 1.0{
                    ind+=1;
                }else{
                    if ind != 0 {
                        indicV[i].push(ind);
                        ind = 0;
                    }
                }
            }
            if ind != 0{
                indicV[i].push(ind);
                ind  = 0;
            }

            if indicV[i].len() == 0 {
                indicV[i].push(0);
            }
        }



        if false {
            println!("{:?}", sprite);
            println!("{:?}", indicH);
            println!("{:?}", indicV);
        }


        Self{ grid: grid, indicH : indicH, indicV : indicV, seq: vec![], colToCheck: colToCheck, lineToCheck: lineToCheck, filled : 0, guessing : true, discardedMoves: vec![], nGuessing : 0 }
    }

    pub fn new(indicH : Vec<Vec<usize>>, indicV : Vec<Vec<usize>>) -> Self {
        Self{ grid: vec![], indicH : indicH, indicV : indicV, seq: vec![], colToCheck: vec![], lineToCheck: vec![], filled : 0, guessing : true, discardedMoves: vec![], nGuessing : 0 }
    }

    pub fn fullGridValid(&self) -> bool{
        for i in 0..self.grid.len(){
            if self.indicFromVec(self.getLine(i)) != self.indicH[i] {
                return false;
            }
        }

        for i in 0..self.grid[0].len(){
            if self.indicFromVec(self.getCol(i)) != self.indicV[i] {
                return false;
            }
        }
        return true;
    }

    pub fn indicFromVec(& self, v : Vec<f64>) -> Vec<usize>{
        let mut indic = vec![];
        let mut ind = 0;
        for j in 0..v.len(){
            if v[j] == 1.0{
                ind+=1;
            }else{
                if ind != 0{
                    indic.push(ind);
                    ind = 0;
                }
            }
        }
        if ind != 0{
            indic.push(ind);
        }
        if indic.len() == 0 {
            indic.push(0);
        }
        return indic
    }

    pub fn play(&mut self, m : Move){

        for i in 0..m.val.len(){
            if self.grid[m.y[i]][m.x[i]] == 0.0{
                self.filled += 1;
            }

            self.grid[m.y[i]][m.x[i]] = m.val[i];
            if Self::USE_PARTIAL_CHECK{
                //self.lineToCheck.push(m.y[i]);
                //self.colToCheck.push(m.x[i]);

                self.lineToCheck.insert(0, m.y[i]);
                self.colToCheck.insert(0, m.x[i]);
            }


        }
        self.seq.push(m);
    }

    pub fn getLine(&self, i : usize) -> Vec<f64>{
        return self.grid[i].clone();
    }

    pub fn display(&self){
        for i in 0..self.grid.len(){

            for j in 0..self.grid[0].len(){
                if j == 0 {
                    print!("|")
                }
                if self.grid[i][j] == 1.0 {
                    print!("\u{25A0}")
                }else{
                    if self.grid[i][j] == -1.0 {
                        print!("x")
                    }else{
                        print!(" ")
                    }

                }
                print!("|")
            }
            print!("\n")
        }
        print!("\n")
    }

    pub fn getCol(&self, i : usize) -> Vec<f64>{
        let mut ret = vec![];
        for j in 0..self.grid.len(){
            ret.push(self.grid[j][i]);
        }
        return ret;
    }

    pub fn wiggle_indic_recur(&self, slice : Vec<f64>, indics: Vec<usize>, origin_indics: Vec<usize>) -> Vec<Vec<f64>>{
        let mut possible = vec![];

        //place indics blancs sauf là où il y a des noirs sûrs
        for i in 0..(slice.len()-indics[0]+1){
            let mut call_recur = true;
            let mut new_slice = slice.clone();

            //pas de blanc avant commencer

            //pas de blanc après terminé
            if i + indics[0] < new_slice.len() && new_slice[i + indics[0]] == 1.0{
                call_recur = false;
            }

            if call_recur {
                for j in 0..indics[0]{
                    if new_slice[i+j] == -1.0 {
                        call_recur = false;
                        break
                    }else{
                        new_slice[i+j] = 1.0;
                    }
                }
            }




            if call_recur {
                let mut new_indics = indics.clone();
                new_indics.remove(0);
                if new_indics.len() != 0 {
                    let childs = self.wiggle_indic_recur(new_slice, new_indics, origin_indics.clone());
                    for c in childs {
                        possible.push(c);
                    }
                }else{
                    let checkIndic = self.indicFromVec(new_slice.clone());
                    if checkIndic.len() == origin_indics.len() {
                        let mut topush = true;
                        for e in 0..checkIndic.len(){
                            if checkIndic[e] != origin_indics[e]{
                                topush = false;
                            }
                        }
                        if topush{
                            possible.push(new_slice);
                        }

                    }


                }
            }
        }

        return possible;
    }

    pub fn legal_moves(&mut self, verbose : bool) ->Vec<Move>{
        let mut vec :Vec<Move> = Vec::new();
        let mut discard = 0;

        let mut iter = (0..self.grid.len()).collect();
        if Self::USE_PARTIAL_CHECK{
            iter = self.lineToCheck.clone();
        }
        for i in iter {
            if Self::USE_PARTIAL_CHECK{
                self.lineToCheck.remove(0);
                discard += 1;
            }
            let mut m : Move = Move{x : vec![], y : vec![], val : vec![]};
            let possible = self.wiggle_indic_recur(self.getLine(i), self.indicH[i].clone(), self.indicH[i].clone() );

            if possible.len() != 0 {
                let mut sum = possible[0].clone();
                for j in 1..possible.len(){
                    for k in 0..possible[j].len(){
                        sum[k] += possible[j][k];
                    }
                }


                let mut topush = false;
                for j in 0..sum.len(){
                    if sum[j] == possible.len() as f64 && self.grid[i][j] != 1.0{
                        m.x.push(j);
                        m.y.push(i);
                        m.val.push(1.0);
                        topush = true;
                    }

                    if sum[j] == 0.0 {
                        m.x.push(j);
                        m.y.push(i);
                        m.val.push(-1.0);
                        topush = true;
                    }
                }
                if topush{
                    vec.push(m);

                    if Self::RETURN_FIRST_LEGAL_MOVE {
                        self.discardedMoves.push(discard);
                        return vec;
                    }
                }
            }


        }

        let mut iter2 = (0..self.grid[0].len()).collect();
        if Self::USE_PARTIAL_CHECK{
            iter2 = self.colToCheck.clone();
        }
        for i in iter2 {
            if Self::USE_PARTIAL_CHECK{
                discard += 1;
                self.colToCheck.remove(0);
            }
            let mut m : Move = Move{x : vec![], y : vec![], val : vec![]};
            let possible = self.wiggle_indic_recur(self.getCol(i), self.indicV[i].clone(), self.indicV[i].clone() );

            if possible.len() != 0 {
                let mut sum = possible[0].clone();
                for j in 1..possible.len(){
                    for k in 0..possible[j].len(){
                        sum[k] += possible[j][k];
                    }
                }

                let mut topush = false;
                for j in 0..sum.len(){
                    if sum[j] == possible.len() as f64 && self.grid[j][i] != 1.0  {
                        m.x.push(i);
                        m.y.push(j);
                        m.val.push(1.0);
                        topush = true;
                    }

                    if sum[j] == 0.0 && self.grid[j][i] != -1.0 {
                        m.x.push(i);
                        m.y.push(j);
                        m.val.push(-1.0);
                        topush = true;
                    }
                }
                if topush{
                    vec.push(m);

                    if Self::RETURN_FIRST_LEGAL_MOVE {
                        self.discardedMoves.push(discard);
                        return vec;
                    }
                }
            }


        }

        if(self.guessing){
            if vec.len() == 0 {
                //edge solving
                //version maline : place des blancs de grande ligne sur bords
            }

            if vec.len() == 0 {
                //desesperate guessing
                //pour chaque case libre, pose un faux blanc ou noir, fais un déroulé, et regarde si el résultat est valide (jamais deux edge solving d'affilé)
                self.nGuessing+=1;


                for i in 0..self.grid.len(){
                    for j in 0..self.grid[0].len() {
                        if self.grid[i][j] == 0.0 {
                            for v in [-1.0, 1.0]{
                                let mut cl = self.clone();
                                cl.guessing = false;
                                cl.grid[i][j] = v;
                                cl.colToCheck.push(j);
                                cl.lineToCheck.push(i);
                                cl.filled += 1;
                                let mut moves = cl.legal_moves(false);
                                while moves.len() != 0 {
                                    cl.play(moves[0].clone());
                                    moves = cl.legal_moves(false);
                                }
                                if cl.filled == self.grid[0].len() * self.grid.len() {
                                    if cl.fullGridValid(){
                                        let mut m : Move = Move{x : vec![], y : vec![], val : vec![]};
                                        m.x.push(j);
                                        m.y.push(i);
                                        m.val.push(v);

                                        vec.push(m);

                                        if Self::RETURN_FIRST_LEGAL_MOVE {
                                            return vec;
                                        }
                                    }else{
                                        let mut m : Move = Move{x : vec![], y : vec![], val : vec![]};
                                        m.x.push(j);
                                        m.y.push(i);
                                        m.val.push(-v);

                                        vec.push(m);

                                        if Self::RETURN_FIRST_LEGAL_MOVE {
                                            return vec;
                                        }
                                    }
                                }
                            }




                        }

                    }
                }
            }
        }




        if vec.len() == 0 && verbose{
            if self.guessing {
                if  self.filled != self.grid[0].len() * self.grid.len() {
                    println!("je ne sais pas quelle heuristique appliquer {}, {}", self.fullGridValid(), self.guessing);
                }else{
                    println!("resolution termine {}", self.fullGridValid());
                }
                self.display();
            }

        }


        return vec;
    }

    pub fn score(& self) -> f64{

        return 0.0 ;
    }


    pub fn smoothedScore(&self) -> f64{
        return 0.0;
    }

    pub fn heuristic(&self, m : Move) -> f64{
        return 0.0;
    }

    pub fn terminal(& self) -> bool{
        return false;
    }

    pub fn solve(& self, verbose : bool) -> Vec<f64>{
        let mut start_time = Instant::now();
        let mut init_st = self.clone();
        let mut moves = init_st.legal_moves(false);


        while moves.len() != 0 {

            init_st.play(moves[0].clone());

            //init_st.display();
            moves = init_st.legal_moves(verbose);
        }

        let solving_time = start_time.elapsed().as_secs_f64();

        if init_st.nGuessing != 0 {
            init_st.nGuessing -= 1; //the last moves utimately passes through the guessing step even if there is nothing to play anymore
        }
        let mut sum = 0.0;
        let mut max = 0.0;
        let mut nbTimesDiscardBig = 0.0;
        for e in init_st.discardedMoves.clone() {
            sum += e as f64;
            if e as f64 > max {
                max = e as f64;
            }
            if e > 7 {
                nbTimesDiscardBig += 1.0;
            }
        }

        let mut moves_len = vec![];
        let mut sum_moves_len = 0.0;
        let mut max_moves_len = 0.0;
        for m in init_st.seq.clone() {
            moves_len.push(m.y.len() as f64);
            sum_moves_len += m.y.len() as f64;
            if m.y.len() as f64 > max_moves_len {
                max_moves_len = m.y.len() as f64;
            }
        }

        let mut nbw = 0.0;
        let mut nbb = 0.0;

        for line in init_st.grid.clone() {
            for i in line {
                if i == -1.0 {
                    nbb += 1.0;
                }
                if i == 1.0 {
                    nbw += 1.0;
                }
            }
        }

        if verbose {
            println!("actions : {}     discarded mean : {}       discarded max : {}      guessing : {}", init_st.seq.len(), sum/(init_st.seq.len()as f64) , max,  init_st.nGuessing);
            println!("move len mean : {}", sum_moves_len/ init_st.seq.len() as f64);
            println!("discarded mean over move len mean : {}", (sum/(init_st.seq.len()as f64))/(sum_moves_len/ init_st.seq.len() as f64));
            println!("number of great backtracking : {}", nbTimesDiscardBig);
            println!("solving time : {}", solving_time);
        }


        if init_st.filled != init_st.grid[0].len() * init_st.grid.len() {
            //println!("je ne sais pas quelle heuristique appliquer {}", self.fullGridValid());

            return vec![0.0, 0.0, 0.0];
        }

        let mut kurt : Kurtosis = moves_len.clone().into_iter().collect();
        let mut uniq_move_len = moves_len.clone();
        uniq_move_len.sort_by(|a, b| a.partial_cmp(b).unwrap());
        uniq_move_len.dedup();
        //println!("frzsdf {} {:?} {}", kurt.kurtosis(), moves_len, uniq_move_len.len());

        /*
        println!("dqf {:? }", moves_len);

        println!("frzsdf {}", kurt.kurtosis());

        let mut a: MeanWithError = (1..6).map(f64::from).collect();
        a.add(42.);
        println!("The mean is {} ± {}.", a.mean(), a.error());

        init_st.clone().display();
         */

        let mut score = vec![
            solving_time, //time
            init_st.seq.len() as f64 + (init_st.nGuessing as f64).sqrt()*50.0 + nbTimesDiscardBig *10.0, //difficulty estimation

            1000.0 - (kurt.kurtosis())*uniq_move_len.len() as f64 + (nbw*nbb*4.0)/sum_moves_len.powf(2.0)*uniq_move_len.len()as f64   +5.0- cmp::max(nbTimesDiscardBig as u32, 5) as f64 - (init_st.nGuessing as f64).powf(2.0)  , //fun v3 (idée, mais la variance envoie juste vers extremes)
            1000.0+ (nbw*nbb*4.0)/(sum_moves_len*init_st.seq.len()as f64) as f64  +5.0- cmp::max(nbTimesDiscardBig as u32, 5) as f64 - (init_st.nGuessing as f64).powf(2.0)  , //fun v2
            init_st.seq.len() as f64,
            sum/(init_st.seq.len()as f64),
            max,
            init_st.nGuessing as f64,
            sum_moves_len/ init_st.seq.len() as f64,
            (sum/(init_st.seq.len()as f64))/(sum_moves_len/ init_st.seq.len() as f64)];
        return score;
    }
}

fn mean(data: Vec<f64>) -> Option<f64> {
    let sum = data.iter().sum::<f64>();
    let count = data.len();

    match count {
        positive if positive > 0 => Some(sum / count as f64),
        _ => None,
    }
}

fn std_deviation(data: Vec<f64>) -> Option<f64> {
    match (mean(data.clone()), data.len()) {
        (Some(data_mean), count) if count > 0 => {
            let variance = data.iter().map(|value| {
                let diff = data_mean - (*value);

                diff * diff
            }).sum::<f64>() / count as f64;

            Some(variance.sqrt())
        },
        _ => None
    }
}

#[derive(Clone)]
pub struct Move{
    pub x : Vec<usize>,
    pub y : Vec<usize>,
    pub val : Vec<f64>
}
