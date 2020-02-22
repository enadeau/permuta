use permuta::perm::Perm;
use permuta::perm_class::Av;

pub fn main() {
    let patt = Perm::new(vec![0,1,2]);
    let mut class = Av::new(vec![patt]);
    for i in 0..15 {
        println!("{}: {}", i, class.num_of_length(i));
    }
}
