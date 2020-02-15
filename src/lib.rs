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
                                None => self.len() - self.values[index],
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
}
