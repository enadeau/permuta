mod misc;
pub use crate::misc::left_floor_and_ceiling;

#[derive(PartialEq)]
#[derive(Debug)]
struct Perm {
    values: Vec<usize>,
}

impl Perm {
    fn new(values: Vec<usize>) -> Perm {
        Perm { values }
    }

    fn pattern_details(&self)
        -> Vec<(Option<usize>, Option<usize>, usize, usize)> {
        let mut result = Vec::new();
        let mut index = 0;
        for fac_indices in &left_floor_and_ceiling(&self.values, None, None) {
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
}
