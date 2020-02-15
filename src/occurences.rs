use crate::Perm;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Occurence {
    position: Vec<usize>
}

impl Occurence {
    pub fn new(occ: Vec<usize>) -> Occurence {
        Occurence{ position: occ }
    }

    fn from_option_vector(occ: &Vec<Option<usize>>) -> Occurence {
        Occurence{ position: occ.iter().map(|x| x.unwrap()).collect() }
    }
}

pub struct OccurencesIterator<'a> {
    pub perm: &'a Perm,
    pub patt: &'a Perm,
    pattern_details: Vec<(Option<usize>, Option<usize>, usize, usize)>,
    occurrence_indices: Vec<Option<usize>>,
    stack: Vec<(usize, usize)>,
}

impl<'a> OccurencesIterator<'a> {
    pub fn new(perm: &'a Perm, patt: &'a Perm) -> OccurencesIterator<'a> {
        let occurrence_indices = vec![None; patt.len()];
        let pattern_details = patt.pattern_details();
        OccurencesIterator {
            perm: perm,
            patt: patt,
            pattern_details: pattern_details,
            occurrence_indices: occurrence_indices,
            stack: vec![(0,0)],
        }
    }
}

impl<'a> Iterator for OccurencesIterator<'a> {
    type Item = Occurence;

    fn next(&mut self) -> Option<Occurence> {
        let (i, k) = match self.stack.pop() {
            None => { return None; },
            Some(t) => t
        };
        if k == self.patt.len() {
            return Some(Occurence::from_option_vector(&self.occurrence_indices))
        }
        if i >= self.perm.len() { return self.next(); }
        self.stack.push((i+1, k));
        let (lfi, lci, lbp, ubp) = self.pattern_details[k];
        let lower_bound = lbp + match lfi {
            None => 0,
            Some(i) => self.perm[self.occurrence_indices[i].unwrap()]
        };
        let upper_bound = match lci {
            None => self.perm.len(),
            Some(i) => self.perm[self.occurrence_indices[i].unwrap()]
        } - ubp;
        let element = self.perm[i];
        if lower_bound <= element && element <= upper_bound {
            self.occurrence_indices[k] = Some(i);
            self.stack.push((i+1, k+1));
        }
        return self.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let empty_perm = Perm::new(vec![]);
        let perm = Perm::new(vec![4,1,2,3,0]);

        let expected: Vec<Occurence> = vec![
            Occurence::new(vec![]),
        ];
        assert_eq!(
            empty_perm.occurrences_in(&perm).collect::<Vec<Occurence>>(),
            expected
        );

        let expected: Vec<Occurence> = Vec::new();
        assert_eq!(
            perm.occurrences_in(&empty_perm).collect::<Vec<Occurence>>(),
            expected
        );
    }

    #[test]
    fn test_small() {
        let patt = Perm::new(vec![1,0]);
        let perm = Perm::new(vec![1,2,3,0]);
        let expected: Vec<Occurence> = vec![
            Occurence::new(vec![0,3]),
            Occurence::new(vec![1,3]),
            Occurence::new(vec![2,3]),
        ];
        assert_eq!(
            patt.occurrences_in(&perm).collect::<Vec<Occurence>>(),
            expected
        );
    }

    #[test]
    fn test_big() {
        let perm = Perm::new(vec![5,3,0,4,2,1]);
        let patt = Perm::new(vec![2,0,1]);
        let expected: Vec<Occurence> = vec![
            Occurence::new(vec![0,1,3]),
            Occurence::new(vec![0,2,3]),
            Occurence::new(vec![0,2,4]),
            Occurence::new(vec![0,2,5]),
            Occurence::new(vec![1,2,4]),
            Occurence::new(vec![1,2,5]),
        ];
        assert_eq!(
            patt.occurrences_in(&perm).collect::<Vec<Occurence>>(),
            expected
        );
    }
}
