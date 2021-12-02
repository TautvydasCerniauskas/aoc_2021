use std::collections::HashMap;

pub fn day2_sol1(input: &Vec<(String, usize)>) -> Option<usize> {
    let mut coordinates = HashMap::from([("horizontal", 0), ("depth", 0)]);

    input.iter().for_each(|(k, v)| {
        if k == "forward" {
            *coordinates.get_mut("horizontal").unwrap() += v;
        }
        if k == "down" {
            *coordinates.get_mut("depth").unwrap() += v;
        }
        if k == "up" {
            *coordinates.get_mut("depth").unwrap() -= v;
        }
    });
    calculate_result(coordinates)
}

pub fn day2_sol2(input: &Vec<(String, usize)>) -> Option<usize> {
    let mut coordinates = HashMap::from([("horizontal", 0), ("depth", 0), ("aim", 0)]);

    input.iter().for_each(|(k, v)| {
        if k == "forward" {
            *coordinates.get_mut("horizontal").unwrap() += v;
            *coordinates.get_mut("depth").unwrap() += *coordinates.get_mut("aim").unwrap() * v;
        }
        if k == "down" {
            *coordinates.get_mut("aim").unwrap() += v;
        }
        if k == "up" {
            *coordinates.get_mut("aim").unwrap() -= v;
        }
    });
    calculate_result(coordinates)
}

fn calculate_result(map: HashMap<&str, usize>) -> Option<usize> {
    let hor = map["horizontal"];
    let depth = map["depth"];
    Some(hor * depth)
}
