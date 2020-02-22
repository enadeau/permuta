use std::ops::Index;
use crate::misc;
mod occurences;

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Perm {
    values: Vec<usize>,
}

impl Perm {
    pub fn new(values: Vec<usize>) -> Perm {
        Perm { values }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn contains(&self, patt: &Perm) -> bool {
        match patt.occurrences_in(self).next() {
            None => false,
            Some(_) => true
        }
    }

    pub fn contained_in(&self, perm: &Perm) -> bool {
        perm.contains(self)
    }

    pub fn avoids(&self, basis: &Vec<Perm>) -> bool {
        basis.iter().all(|patt| !self.contains(&patt))
    }

    pub fn occurrences_in<'a>(&'a self, perm: &'a Perm) ->  occurences::OccurencesIterator {
        occurences::OccurencesIterator::new(perm, self)
    }

    pub fn insert_max(&self, index: usize) -> Perm {
        let new_element = self.len();
        let mut new_values = self.values.clone();
        new_values.insert(index, new_element);
        Perm::new(new_values)
    }

    fn pattern_details(&self)
        -> Vec<(Option<usize>, Option<usize>, usize, usize)> {
        let mut result = Vec::new();
        let mut index = 0;
        for fac_indices in &misc::left_floor_and_ceiling(&self.values, None, None) {
            let base_element = self[index];
            let compiled = (fac_indices.floor,
                            fac_indices.ceiling,
                            match fac_indices.floor {
                                None => self[index],
                                Some(i) => base_element - self[i],
                            },
                            match fac_indices.ceiling {
                                None => self.len() - self[index],
                                Some(i) => self[i] - base_element,
                            }
                            );
            result.push(compiled);
            index += 1;
        }
        result
    }
}

impl Index<usize> for Perm {
    type Output = usize;

    fn index(&self, i: usize) -> &Self::Output {
        match self.values.get(i) {
            None => panic!("Perm index out of range"),
            Some(v) => &v
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use occurences::Occurence;

    #[test]
    fn test_cont() {
        let perm = Perm::new(vec![2,0,1]);
        let patt = Perm::new(vec![5,3,0,4,2,1]);
        let expected: Vec<Occurence> = vec![
            Occurence::new(vec![0,1,3]),
            Occurence::new(vec![0,2,3]),
            Occurence::new(vec![0,2,4]),
            Occurence::new(vec![0,2,5]),
            Occurence::new(vec![1,2,4]),
            Occurence::new(vec![1,2,5]),
        ];
        assert_eq!(
            perm.occurrences_in(&patt).collect::<Vec<Occurence>>(),
            expected
        );
    }

    #[test]
    fn pattern_details() {
        let expected = vec![(None, None, 0, 2), (Some(0), None, 1, 1)];
        assert_eq!(
            Perm::new(vec![0,1]).pattern_details(),
            expected
        );

        let expected = vec![(None, None, 1, 1), (None, Some(0), 0, 1)];
        assert_eq!(
            Perm::new(vec![1,0]).pattern_details(),
            expected
        );

        let expected = vec![
            (None, None, 9, 1),
            (None, Some(0), 2, 7),
            (None, Some(1), 1, 1),
            (Some(1), Some(0), 4, 3),
            (Some(1), Some(3), 3, 1),
            (Some(3), Some(0), 1, 2),
            (Some(5), Some(0), 1, 1),
            (Some(1), Some(4), 2, 1),
            (None, Some(2), 0, 1),
            (Some(1), Some(7), 1, 1),
        ];
        assert_eq!(
            Perm::new(vec![9, 2, 1, 6, 5, 7, 8, 4, 0, 3]).pattern_details(),
            expected
        );
    }

    #[test]
    fn index() {
        let perm = Perm::new(vec![5,3,0,4,2,1]);
        assert_eq!(perm[0], 5);
        assert_eq!(perm[1], 3);
        assert_eq!(perm[5], 1);
    }

    #[test]
    #[should_panic(expected = "Perm index out of range")]
    fn index_out_of_bound() {
        let perm = Perm::new(vec![5,3,0,4,2,1]);
        perm[6];
    }

    #[test]
    fn contains() {
        assert!(Perm::new(vec![3,7,0,8,1,6,5,2,4])
                .contains(&Perm::new(vec![3,7,0,8,1,6,5,2,4])));

        assert!(Perm::new(vec![3,7,0,8,1,6,5,2,4])
                  .contained_in(&Perm::new(vec![3,7,0,8,1,6,5,2,4])));
        assert!(Perm::new(vec![]).contained_in(&Perm::new(vec![])));
        assert!(Perm::new(vec![]).
                contained_in(&Perm::new(vec![3,7,0,8,1,6,5,2,4])));
        assert!(Perm::new(vec![0])
                .contained_in(&Perm::new(vec![0])));
        assert!(!Perm::new(vec![7,3,0,8,1,6,5,2,4])
                .contained_in(&Perm::new(vec![3,7,0,8,1,6,5,2,4])));

        assert!(!Perm::new(vec![0]).contained_in(&Perm::new(vec![])));
        assert!(!Perm::new(vec![0,1]).contained_in(&Perm::new(vec![])));
        assert!(!Perm::new(vec![0,1]).contained_in(&Perm::new(vec![0])));
        assert!(!Perm::new(vec![1,0]).contained_in(&Perm::new(vec![0,1])));
        assert!(!Perm::new(vec![0,1,2]).contained_in(&Perm::new(vec![0,1])));
        assert!(!Perm::new(vec![1,0,2]).contained_in(&Perm::new(vec![0,1,3,4,2])));
        assert!(!Perm::new(vec![0,1,2]).contained_in(&Perm::new(vec![2,1,3,0])));
        assert!(!Perm::new(vec![2,1,3,0]).contained_in(&Perm::new(vec![2,0,3,1])));
        assert!(!Perm::new(vec![0,2,1]).contained_in(&Perm::new(vec![2,0,1,3])));
        assert!(!Perm::new(vec![2,0,1,3]).contained_in(&Perm::new(vec![5,3,2,7,1,0,6,4])));
        assert!(!Perm::new(vec![0,1,2,3]).contained_in(&Perm::new(vec![4,7,5,1,6,2,3,0])));
    }

    #[test]
    fn avoids() {
        let patt1 = Perm::new(vec![0,1]);
        let patt2 = Perm::new(vec![2,0,1]);
        let patt3 = Perm::new(vec![0,3,1,2]);
        let patt4 = Perm::new(vec![0,1,2]);
        assert!(!Perm::new(vec![5,3,0,4,2,1]).avoids(&vec![patt1, patt2]));
        assert!(Perm::new(vec![5,3,0,4,2,1]).avoids(&vec![patt3, patt4]));
    }

    #[test]
    fn insert_max() {
        let perm = Perm::new(vec![0,1]);
        assert_eq!(
            perm.insert_max(0),
            Perm::new(vec![2,0,1])
        );
        assert_eq!(
            perm.insert_max(1),
            Perm::new(vec![0,2,1])
        );
        assert_eq!(
            perm.insert_max(2),
            Perm::new(vec![0,1,2])
        );
    }
}
