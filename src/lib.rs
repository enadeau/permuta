mod misc;
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

    fn pattern_details(&self)
        -> Vec<(Option<usize>, Option<usize>, usize, usize)> {
        let mut result = Vec::new();
        let mut index = 0;
        for fac_indices in &misc::left_floor_and_ceiling(&self.values, None, None) {
            let base_element = self.values[index];
            let compiled = (fac_indices.floor,
                            fac_indices.ceiling,
                            match fac_indices.floor {
                                None => self.values[index],
                                Some(i) => base_element - self.values[i],
                            },
                            match fac_indices.ceiling {
                                None => self.values.len() - self.values[index],
                                Some(i) => self.values[i] - base_element,
                            }
                            );
            result.push(compiled);
            index += 1;
        }
        result
    }

    fn len(&self) -> usize {
        self.values.len()
    }

    pub fn occurrences_in<'a>(&'a self, perm: &'a Perm) ->  occurences::OccurencesIterator {
        occurences::OccurencesIterator::new(perm, self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cont() {
        let perm = Perm::new(vec![2,0,1]);
        let patt = Perm::new(vec![5,3,0,4,2,1]);
        let expected: Vec<Vec<usize>> = Vec::new();
        assert_eq!(
            perm.occurrences_in(&patt, &None, &None),
            expected
        );
    // list(Perm((2, 0, 1)).occurrences_in(Perm((5, 3, 0, 4, 2, 1))))
    //         [(0, 1, 3), (0, 2, 3), (0, 2, 4), (0, 2, 5), (1, 2, 4), (1, 2, 5)]
    }
}
