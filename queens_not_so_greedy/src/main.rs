/*
    Este programa genera 1 solución al azar y hace movimientos greedy para resolver el problema de las reinas
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

    let mut vector_of_queens = Vec::<(isize,isize)>::with_capacity(8);

    let step =  Range::new(0, 8);
    let moves = vec![(-1,1),(0,1),(1,1),(-1,0),(1,0),(-1,-1),(0,-1),(1,-1)];
    let mut rng = rand::thread_rng();

    get_random_vector(& mut vector_of_queens, & step, & mut rng);

    let mut collisions = get_collisions( & vector_of_queens );

    let queen = step.ind_sample( & mut rng);
    println!("{} con {:?}", collisions, vector_of_queens);

    do_greedy_move( queen as usize, & mut vector_of_queens, &moves , & mut collisions);

    println!("TERMINADO");
    pause();
}
fn do_greedy_move ( queen: usize, mut vec: & mut Vec<(isize,isize)> ,moves: &Vec<(isize,isize)>, mut collisions: &mut isize ) {

    let mut newvec = vec.to_vec();
    let mut tuple = vec[queen];
    let mut mincollisions;
    let mut a;
    let mut b;

    for &(x,y) in moves {
        newvec[queen] = vec[queen];
        a = newvec[queen].0;
        b = newvec[queen].1;
        if a + x <= 8 && a + x >= 1 && b + y <= 8 && b + y >= 1 {
            newvec[queen] = (a+x,b+y);
            mincollisions = get_collisions( & newvec );
            if mincollisions < *collisions {
                *collisions = mincollisions;
                tuple = newvec[queen];
            }
        }
    }

    if tuple == vec[queen] {
        println!("Llegué a un mínimo local :c ");
        return;
    }

    vec[queen] = tuple;
    println!("{}, con {:?}", *collisions, vec );
    do_greedy_move( queen, & mut vec, &moves, & mut collisions );

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
