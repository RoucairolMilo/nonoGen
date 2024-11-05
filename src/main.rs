
mod nonogramSolver;
mod generator;

mod BFS;
mod BEAM;

mod NMCS;
mod lazyNMCSv3;
mod NRPA;
mod UCT;
mod GRAVE;

mod tools;
mod Sampling;

use std::fs;
use std::time::{Duration, Instant, SystemTime};
use crate::NRPA::launch_nrpa;


fn main() {

    let paths = fs::read_dir("nonograms/").unwrap();

//to try nonograms one by one
    if false {
        let mut init_st = nonogramSolver::State::new_from_png("nonograms/001bird.png".to_string());
        let res = init_st.solve(true);

        println!("res {:?}", res);

        println!("estimated difficulty {}", res[1]);
        println!("estimated fun {}", res[2]);
        println!("----------------------------------------------------------------------------------------");
    }

//to try all nonograms
    if false {
        for path in paths {

            let mut start_time = Instant::now();

            let pa = format!("{}", path.unwrap().path().display());
            println!("Name: {}", pa);
            let mut init_st = nonogramSolver::State::new_from_png(pa);
            let res = init_st.solve(true);

            println!("res {:?}", res);

            println!("estimated difficulty {}", res[1]);
            println!("estimated fun {}", res[2]);
            println!("----------------------------------------------------------------------------------------");

        }
    }

//to generate nonograms that are difficult for human, difficult for the solver, and fun (maybe)
    if true {
        for size in vec![(5, 5), (5, 10), (10, 10), (15, 15)]{
            for opti in vec![2]{ //in vec![0, 1, 2]{
                for i in 0..10 {
                    let mut init_st = generator::State::new();
                    init_st.sizeX = size.0;
                    init_st.sizeY = size.1;
                    init_st.optimizationMode = opti;

                    NMCS::launch_nmcs(init_st.clone(), 2, 0.0, true, 60.0,
                                      String::from(format!("NMCS{}x{}opti{}_{} ", size.0, size.1, opti, i)));

                    BFS::launch_bfs(init_st.clone(), 0.0, -1, 60.0,
                                    String::from(format!("BFS{}x{}opti{}_{} ", size.0, size.1, opti, i)));
                    UCT::launch_UCT(init_st.clone(), 1.0, 1000000, 0.0, 60.0,
                                    String::from(format!("UCT{}x{}opti{}_{} ", size.0, size.1, opti, i)));
                    lazyNMCSv3::launch_lazy_nmcs_v3(init_st.clone(), 3, 0.8, 3, 0, 0.0, 60.0, 0, true,
                                                    String::from(format!("LNMCS{}x{}opti{}_{} ", size.0, size.1, opti, i)));
                    NRPA::launch_nrpa(2, init_st.clone(), 60.0,
                                      String::from(format!("NRPA{}x{}opti{}_{} ", size.0, size.1, opti, i)));
                    GRAVE::launch_grave(init_st.clone(), 5, 0.0, 0.0, 60.0,
                                        String::from(format!("GRAVE{}x{}opti{}_{} ", size.0, size.1, opti, i)), false);
                    BEAM::launch_beam(init_st.clone(), 10, 0.0, -1, 60.0,
                                      String::from(format!("BEAM{}x{}opti{}_{} ", size.0, size.1, opti, i)));


                }

            }


        }
    }
}
