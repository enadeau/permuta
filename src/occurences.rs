use crate::Perm;

#[derive(Debug)]
pub struct Occurence {
    position: Vec<usize>
}

impl Occurence {
    fn from_option_vector(occ: &Vec<Option<usize>>) -> Occurence {
        Occurence{ position: occ.iter().map(|x| x.unwrap()).collect() }
    }
}

pub struct OccurencesIterator<'a> {
    pub perm: &'a Perm,
    pub patt: &'a Perm,
    i: usize,
    k: usize,
    pattern_details: Vec<(Option<usize>, Option<usize>, usize, usize)>,
    occurrence_indices: Vec<Option<usize>>,
}

impl<'a> OccurencesIterator<'a> {
    pub fn new(perm: &'a Perm, patt: &'a Perm) -> OccurencesIterator<'a> {
        let occurrence_indices = vec![None; patt.len()];
        let pattern_details = patt.pattern_details();
        OccurencesIterator {
            perm: perm,
            patt: patt,
            i: 0,
            k: 0,
            pattern_details: pattern_details,
            occurrence_indices: occurrence_indices
        }
    }
}

impl<'a> Iterator for OccurencesIterator<'a> {
    type Item = Occurence;

    fn next(&mut self) -> Option<Occurence> {
        println!("i={}, k={}", self.i, self.k);
        let elements_remaining = self.perm.len() - self.i;
        let elements_needed = self.patt.len() - self.k;
        let (lfi, lci, lbp, ubp) = self.pattern_details[self.k];
        let lower_bound = match lfi {
            None => lbp,
            Some(i) => self.perm.values[self.occurrence_indices[i].unwrap()] + lbp
        };
        let upper_bound = match lci {
            None => self.perm.len() - ubp,
            Some(i) => self.perm.values[self.occurrence_indices[i].unwrap()] + ubp
        };
        loop {
            if elements_remaining < elements_needed { return None }
            let element = self.perm.values[self.i];
            if lower_bound <= element && element <= upper_bound {
                self.occurrence_indices[self.k] = Some(self.i);
                if elements_needed == 1 {
                    self.i += 1;
                    return Some(Occurence::from_option_vector(&self.occurrence_indices))
                } else {
                    self.i += 1;
                    self.k += 1;
                    return self.next()
                }
            }
            panic!("Not implemented")
        }
    }
}
