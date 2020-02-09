struct FloorAndCeiling {
    floor: usize,
    ceiling: usize,
}

pub fn left_floor_and_ceiling(iterable: impl Iterator<PartialOrd>)
    -> Iterator<FloorAndCeiling> {
    true
}

#[cfg(test)]
mod tests {
    fn it_works() {
        left_floor_and_ceiling(vec![1,2,3]);
}
