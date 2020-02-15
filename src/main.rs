extern crate permuta;

use permuta::Perm;

pub fn main() {
    let perm = Perm::new(vec![1,2,3]);
    let patt = Perm::new(vec![1,2]);
    println!("{:?}", perm);
    for occ in patt.occurrences_in(&perm) {
        println!("{:?}", occ);
    }
}
