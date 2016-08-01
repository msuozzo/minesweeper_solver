
#[inline]
pub fn abs_difference(a: usize, b: usize) -> usize {
    if a < b { b - a } else { a - b }
}

pub fn is_adjacent(position: usize, other_position: usize, max_x: usize, max_y: usize) -> bool {
    let max_pos = max_x * max_y - 1;
    if position > max_pos || other_position > max_pos
        || position == other_position
        || abs_difference(position / max_x, other_position / max_x) > 1
        || abs_difference(position % max_x, other_position % max_x) > 1 {
        false
    } else {
        true
    }
}

