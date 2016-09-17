/*
    Este programa genera 1 solución al azar y hace movimientos al azar para el problema de las reinas
*/

extern crate rand;
use rand::Rng;
use rand::distributions::{IndependentSample, Range};

use std::io;
use std::io::prelude::*;

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press enter to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn main() {
    let rep = 10000;

    let mut vector_of_queens = Vec::<(isize,isize)>::with_capacity(8);

    let step =  Range::new(0, 8);
    let step2 =  Range::new(-1, 2);
    let mut rng = rand::thread_rng();

    get_random_vector(& mut vector_of_queens, & step, & mut rng);

    let mut collisions = get_collisions( & vector_of_queens );
    let mut mincollisions = collisions;

    for _ in 0..rep {
        collisions =  get_collisions(& vector_of_queens);

        if collisions < mincollisions {
            mincollisions = collisions;
            println!("{} con {:?}", mincollisions, vector_of_queens);
        }
        do_random_move(& mut vector_of_queens, &step, &step2 ,& mut rng);
    }
    println!("TERMINADO");
    pause();
}
fn do_random_move ( vec: & mut Vec<(isize,isize)> ,step : &Range<isize>, step2: &Range<isize>, mut rng : & mut Rng ) {

    let queen = step.ind_sample( & mut rng);
    let n_tuple = vec[queen as usize];

    let mut x;
    let mut y;
    loop {
        x = step2.ind_sample( & mut rng);
        y = step2.ind_sample( & mut rng);
        if n_tuple.0 + x <= 8 && n_tuple.0 + x >= 1 && n_tuple.1 + y <= 8 && n_tuple.1 + y >= 1 {
            vec[queen as usize].0 = n_tuple.0 + x;
            vec[queen as usize].1 = n_tuple.1 + y;
            break
        }
    }
}
fn get_collisions (vec : & Vec<(isize,isize)> ) -> isize {

    let mut collisions = 0;
    for a in 0..8 {
        for b in a+1 .. 8 {
            if vec[a].0 == vec[b].0 {
                collisions += 1;
            }
            if vec[a].1 == vec[b].1 {
                collisions += 1;
            }
            if ( vec[a].0 - vec[b].1 ).abs() == ( vec[a].1 - vec[b].0 ).abs() {
                collisions += 1;
            }
        }
    }
    collisions
}
fn get_random_vector(vec: & mut Vec<(isize,isize)>, step: & Range<isize>, mut rng: & mut Rng ) {
    vec.clear();
    for _ in 0..8 { vec.push((step.ind_sample(&mut rng),step.ind_sample(&mut rng))); }
}
