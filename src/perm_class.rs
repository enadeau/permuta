use std::cmp::Ordering;
use crate::perm::Perm;

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Av {
    basis: Vec<Perm>,
    perms: Vec<Vec<Perm>>,
    slots: Vec<Vec<Vec<usize>>>,
}

impl Av {

    pub fn new(basis: Vec<Perm>) -> Av {
        Av {
            basis: basis,
            perms: vec![vec![Perm::new(vec![])]],
            slots: vec![vec![vec![0]]],
        }
    }

    /// Returns a permutation class from a basis written as a string
    ///
    /// # Examples
    /// ```
    /// use permuta::perm::Perm;
    /// use permuta::perm_class::Av;
    ///
    /// let class = Av::from_string("012_0213");
    ///
    /// assert_eq!(
    ///     class,
    ///     Av::new(vec![Perm::from_string("012"), Perm::from_string("0213")])
    ///     );
    /// ```
    pub fn from_string(basis_str: &str) -> Av {
        let basis: Vec<Perm> = basis_str.split("_").map(|p| Perm::from_string(p)).collect();
        Av::new(basis)
    }

    fn ensure_level(&mut self, level_number: usize) {
        if self.perms.len() < level_number {
            self.ensure_level(level_number - 1);
        }
        if self.basis.len() == level_number {
            return
        }
        let mut new_level: Vec<Perm> = Vec::new();
        let mut new_slots: Vec<Vec<usize>> = Vec::new();
        let last_level_perms = self.perms.last().unwrap();
        let last_level_slots = self.slots.last().unwrap();
        for (perm, slot) in last_level_perms.iter().zip(last_level_slots) {
            let mut good_slots: Vec<usize> = Vec::new();
            for index in slot {
                let new_perm = perm.insert_max(*index);
                if new_perm.avoids(&self.basis) {
                    new_level.push(new_perm);
                    good_slots.push(*index);
                }
            }
            new_slots.append(&mut slot_expansion(&good_slots));
        }
        self.perms.push(new_level);
        self.slots.push(new_slots);
    }

    pub fn num_of_length(&mut self, length: usize) -> usize {
        self.ensure_level(length);
        self.perms[length].len()
    }
}

fn slot_expansion(good_slots: &Vec<usize>) -> Vec<Vec<usize>> {
    let mut all_new_slot = vec![Vec::<usize>::with_capacity(
        good_slots.len()); good_slots.len()];
    for (i, slot_index) in good_slots.iter().enumerate() {
       for (j, slot_vec) in all_new_slot.iter_mut().enumerate() {
           if i <= j { slot_vec.push(*slot_index); }
           if i >= j { slot_vec.push(slot_index+1); }
       }
    }
    all_new_slot
}
