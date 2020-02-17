use permuta::perm::Perm;

pub fn main() {
    let perm_inc = Perm::new(vec![0,1,2]);
    let perm_dec = Perm::new(vec![3,2,1,0]);
    let patt = Perm::new(vec![0,1]);
    println!("Occ of {:?} in {:?}", patt, perm_inc);
    for occ in patt.occurrences_in(&perm_inc) {
        println!("============================>{:?}", occ);
    }
    println!("Occ of {:?} in {:?}", patt, perm_dec);
    for occ in patt.occurrences_in(&perm_dec) {
        println!("============================>{:?}", occ);
    }
}
