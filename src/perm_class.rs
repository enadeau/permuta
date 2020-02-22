use crate::perm::Perm;

pub struct Av {
    basis: Vec<Perm>,
    cache: Vec<Vec<Perm>>,
}

impl Av {

    pub fn new(basis: Vec<Perm>) -> Av {
        Av {
            basis: basis,
            cache: vec![vec![Perm::new(vec![])]],
        }
    }

    fn ensure_level(&mut self, level_number: usize) {
        if self.cache.len() < level_number {
            self.ensure_level(level_number - 1);
        }
        if self.basis.len() == level_number {
            return
        }
        let mut new_level: Vec<Perm> = Vec::new();
        let last_level = self.cache.last().unwrap();
        let total_indices = self.cache.len();
        for perm in last_level {
            for index in 0..total_indices {
                let new_perm = perm.insert_max(index);
                if new_perm.avoids(&self.basis) { new_level.push(new_perm); }
            }
        }
        self.cache.push(new_level)
    }

    pub fn num_of_length(&mut self, length: usize) -> usize {
        self.ensure_level(length);
        self.cache[length].len()
    }
}

