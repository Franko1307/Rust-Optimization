/*
    Queens problem with simulated annealing
*/

extern crate rand;
use rand::Rng;
use rand::distributions::{IndependentSample, Range};

use std::io;
use std::io::prelude::*;

fn main() {

    let mut i = 0;

    while simulated_annealing() {
        i += 1;
    }

    println!("i: {}", i);
    pause();

}

fn simulated_annealing() -> bool{

    let mut vector_of_queens = Vec::<(isize,isize)>::with_capacity(8);
    let mut vector_of_queens_a;

    let step  =  Range::new(0, 8);
    let _step =  Range::new(-1, 2);
    let mut rng = rand::thread_rng();

    get_random_solution(& mut vector_of_queens, & step, & mut rng);
    vector_of_queens_a = vector_of_queens.to_vec();



    let mut current_temperature : f64 = 5.0;
    let freezing_temperature    : f64 = 0.0;
    let mut current_stabilizer  : f64 = 50.0;
    let mut system_energy       : f64 = get_cost ( & vector_of_queens ) as f64;
    let mut new_system_energy;
    let mut random_queen;
    let mut j: f64 = 0.0;
    let mut delta;
    let expected_system_energy : f64 = 0.0;

    while current_temperature > freezing_temperature {
        while j < current_stabilizer {
            random_queen = step.ind_sample(&mut rng) as usize;
            new_system_energy = get_neighbour_cost( & mut vector_of_queens_a, &(random_queen), &_step, & mut rng ) as f64;

            delta = system_energy - new_system_energy;

            if delta > 0.0 || rng.next_f64() < (delta / current_temperature).exp() {
                system_energy = new_system_energy;
                vector_of_queens[random_queen] = vector_of_queens_a[random_queen];

                if system_energy == expected_system_energy {
                    println!("TERMINADO {:?} :  {:?}", system_energy, vector_of_queens);
                    return true;
                }

            } else {
                vector_of_queens_a[random_queen] = vector_of_queens[random_queen];
            }

            j += 1.0;
        }
        current_temperature -= 0.05;
        current_stabilizer *= 1.08;
    }
    false
}

fn get_neighbour_cost <> ( mut queens: & mut Vec<(isize,isize)>, index: &usize, step: &Range<isize>, rng: &mut Rng ) -> isize {
    queens[*index] = get_n_tuple( queens[*index], step, rng );
    get_cost ( & queens )
}
fn get_n_tuple <> ( tuple: (isize,isize), step: &Range<isize>, mut rng: &mut Rng ) -> (isize,isize)  {
    let mut n_tuple = tuple;
    let mut a;
    let mut b;
    while n_tuple.0 == tuple.0 && n_tuple.1 == tuple.1 {
        a = step.ind_sample(&mut rng);
        b = step.ind_sample(&mut rng);
        if a + n_tuple.0 >= 0 && a + n_tuple.0 < 8 && b + n_tuple.1 >= 0 && b + n_tuple.1 < 8 {
            n_tuple = ( a + n_tuple.0, b + n_tuple.1 );
        }
    }
    n_tuple
}
fn get_cost (vec : & Vec<(isize,isize)> ) -> isize {

    let mut collisions = 0;
    for a in 0..8 {
        for b in a+1 .. 8 {
            if vec[a].0 == vec[b].0 {
                collisions += 1;
            }
            if vec[a].1 == vec[b].1 {
                collisions += 1;
            }
            if ( vec[a].1 - vec[b].1 ).abs() == ( vec[a].0 - vec[b].0 ).abs() {
                collisions += 1;
            }
        }
    }
    collisions
}
fn get_random_solution(vec: & mut Vec<(isize,isize)>, step: & Range<isize>, mut rng: & mut Rng ) {
    vec.clear();
    for _ in 0..8 { vec.push((step.ind_sample(&mut rng),step.ind_sample(&mut rng))); }
}
fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    write!(stdout, "Press enter to continue...").unwrap();
    stdout.flush().unwrap();

    let _ = stdin.read(&mut [0u8]).unwrap();
}
