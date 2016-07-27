use util::is_adjacent;


pub struct AbsoluteAdjacencyIterator {
    position: usize,
    max_x: usize,
    max_y: usize,
    current: usize,
}

static ADJACENT_INDICES: [(i32, i32); 8] = [
    (-1, -1), (-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1), (0, -1)];
impl AbsoluteAdjacencyIterator {
    pub fn new(position: usize, max_x: usize, max_y: usize) -> AbsoluteAdjacencyIterator {
        AbsoluteAdjacencyIterator{
            position: position, max_x: max_x, max_y: max_y, current: 0}
    }
}

impl Iterator for AbsoluteAdjacencyIterator {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        loop {
            if self.current >= ADJACENT_INDICES.len() {
                return None;
            }
            let (x_multiplier, constant_value) = ADJACENT_INDICES[self.current];
            self.current += 1;
            let pos =
                self.position as i32 + x_multiplier * self.max_x as i32 + constant_value;
            // Perform bounds-checking.
            if pos >= 0 && is_adjacent(pos as usize, self.position, self.max_x, self.max_y) {
                return Some(pos as usize);
            }
        }
    }
}
