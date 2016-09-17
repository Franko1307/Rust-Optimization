/*
    Este programa genera soluciones al azar para el problema de las reinas
*/

extern crate rand;
use rand::Rng;
use rand::distributions::{IndependentSample, Range};

fn main() {
    let rep = 10000;

    let mut vector_of_queens = Vec::<(isize,isize)>::with_capacity(8);

    let step =  Range::new(1, 9);
    let mut rng = rand::thread_rng();


    for _ in 0..rep {
        get_random_vector(& mut vector_of_queens, & step, & mut rng);
        if is_solution(& vector_of_queens) {
            print!("Encontre una solucion {:?}", vector_of_queens);
            break;
        }
    }

}
fn is_solution (vec : & Vec<(isize,isize)> ) -> bool {
    for iter in vec.iter() {
        for _ in iter.next() {
            println!("asdasd");
        }

    }
    true
}
fn get_random_vector(vec: & mut Vec<(isize,isize)>, step: & Range<isize>, mut rng: & mut Rng ) {
    vec.clear();
    for _ in 0..8 {
        vec.push((step.ind_sample(&mut rng),step.ind_sample(&mut rng)));
    }
}
