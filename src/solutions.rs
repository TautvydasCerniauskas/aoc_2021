// Day 1
pub fn day1_sol1(input: &Vec<i32>) -> Option<usize> {
    return Some(
        input
            .as_slice()
            .windows(2)
            .map(|pair| (pair[0], pair[1]))
            .filter(|(prev, next)| prev < next)
            .count(),
    );
}

pub fn day1_sol2(input: &Vec<i32>) -> Option<usize> {
    let mut last_chunk_value = 0;
    Some(input
        .as_slice()
        .windows(6)
        .map(|pairs| pairs[0] + pairs[1] + pairs[2])
        .filter(|sum| {
            let temp_sum = last_chunk_value;
            last_chunk_value = *sum;
            return sum > &temp_sum;
        })
    .count() - 1)
}
