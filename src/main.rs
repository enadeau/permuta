use std::env;
use permuta::perm::Perm;
use permuta::perm_class::Av;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let basis_str = &args.get(1).expect("Need to provide a basis");
    let mut class = Av::from_string(basis_str);
    for i in 0..15 {
        println!("{}: {}", i, class.num_of_length(i));
    }
}
