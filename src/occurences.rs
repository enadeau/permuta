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
        if self.stack.len() == 0 {return None;}
        let (i, k) = self.stack.pop().expect("Empty stack");
        // println!("Call next: i={}, k={}", i, k);
        // println!("Occ indices: {:?}", self.occurrence_indices);
        // println!("stack: {:?}", self.stack);
        if i >= self.perm.len() { return self.next(); }
        let elements_remaining = self.perm.len() - i;
        let elements_needed = self.patt.len() - k;
        let (lfi, lci, lbp, ubp) = self.pattern_details[k];
        let lower_bound = match lfi {
            None => lbp,
            Some(i) => self.perm.values[self.occurrence_indices[i].unwrap()] + lbp
        };
        let upper_bound = match lci {
            None => self.perm.len() - ubp,
            Some(i) => self.perm.values[self.occurrence_indices[i].unwrap()] + ubp
        };
        // if elements_remaining < elements_needed { return None }
        let element = self.perm.values[i];
        if lower_bound <= element && element <= upper_bound {
            self.stack.push((i+1, k));
            self.occurrence_indices[k] = Some(i);
            if elements_needed == 1 {
                return Some(Occurence::from_option_vector(&self.occurrence_indices))
            } else {
                self.stack.push((i+1, k+1));
                return self.next()
            }
        }
        self.stack.push((i+1, k));
        return self.next()
        // panic!("Not implemented")
    }
}
