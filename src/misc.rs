use std::collections;

#[derive(PartialEq)]
#[derive(Debug)]
pub struct FloorAndCeiling {
    pub floor: Option<usize>,
    pub ceiling: Option<usize>,
}

impl FloorAndCeiling {
    pub fn new(floor: Option<usize>, ceiling: Option<usize>)
        -> FloorAndCeiling {
        FloorAndCeiling { floor, ceiling }
    }
}

pub fn left_floor_and_ceiling(iterable: &Vec<usize>,
                              default_floor: Option<usize>,
                              default_ceiling: Option<usize>)
    -> Vec<FloorAndCeiling> {
    let mut result: Vec<FloorAndCeiling> = Vec::new();
    let mut dq: collections::VecDeque<(usize, usize)> = collections::VecDeque::new();
    let mut smallest = iterable[0];
    let mut biggest = iterable[0];
    let mut index = 0;
    for element in iterable {
        let element = *element;
        if index == 0 {
            dq.push_back((element, index));
            smallest = element;
            biggest = element;
            result.push(FloorAndCeiling::new(default_floor, default_ceiling));
        } else {
            if element <= smallest {
                let mut front = dq.front().unwrap();
                while front.0 != smallest {
                    dq.rotate_left(1);
                    front = dq.front().unwrap();
                }
                result.push(FloorAndCeiling::new(default_floor, Some(front.1)));
                dq.push_front((element, index));
                smallest = element;
            } else if element >= biggest {
                let mut back = dq.back().unwrap();
                while back.0 != biggest {
                    dq.rotate_left(1);
                    back = dq.back().unwrap();
                }
                result.push(FloorAndCeiling::new(Some(back.1), default_ceiling));
                dq.push_back((element, index));
                biggest = element;
            } else {
                let mut front = dq.front().unwrap();
                let mut back = dq.back().unwrap();
                while back.0 > element || element > front.0 {
                    dq.rotate_right(1);
                    front = dq.front().unwrap();
                    back = dq.back().unwrap();
                }
                result.push(FloorAndCeiling::new(Some(back.1), Some(front.1)));
                dq.push_front((element, index));
            }
        }
        index += 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_floor_ceiling() {
        let iterable = vec![4,5,1,2,3,6];
        let expected = [
            FloorAndCeiling::new(None, None),
            FloorAndCeiling::new(Some(0), None),
            FloorAndCeiling::new(None, Some(0)),
            FloorAndCeiling::new(Some(2), Some(0)),
            FloorAndCeiling::new(Some(3), Some(0)),
            FloorAndCeiling::new(Some(1), None),
        ];
        assert_eq!(
            left_floor_and_ceiling(&iterable, None, None),
            expected
        );

        let iterable = vec![4,1,2,5,3];
        let expected = vec![
            FloorAndCeiling::new(Some(100), Some(5)),
            FloorAndCeiling::new(Some(100), Some(0)),
            FloorAndCeiling::new(Some(1), Some(0)),
            FloorAndCeiling::new(Some(0), Some(5)),
            FloorAndCeiling::new(Some(2), Some(0)),
        ];
        assert_eq!(
            left_floor_and_ceiling(&iterable, Some(100), Some(5)),
            expected
        );

        let iterable = vec![1,2,3];
        let expected = vec![
            FloorAndCeiling::new(None, None),
            FloorAndCeiling::new(Some(0), None),
            FloorAndCeiling::new(Some(1), None),
        ];
        assert_eq!(
            left_floor_and_ceiling(&iterable, None, None),
            expected
        );

        let iterable = vec![3,2,1];
        let expected = vec![
            FloorAndCeiling::new(None, None),
            FloorAndCeiling::new(None, Some(0)),
            FloorAndCeiling::new(None, Some(1)),
        ];
        assert_eq!(
            left_floor_and_ceiling(&iterable, None, None),
            expected
        );
    }
}
