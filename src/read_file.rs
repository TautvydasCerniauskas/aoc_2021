use std::fs;
use std::str::FromStr;

pub fn read_all<T: FromStr>(file_name: &str) -> Vec<T> {
    fs::read_to_string(file_name)
        .expect("File not found!")
        .lines()
        .map(|x| x.parse::<T>())
        .filter_map(Result::ok)
        .collect()
}

pub fn read_to_tuple<T: FromStr, U: FromStr>(file_name: &str) -> Vec<(T, U)> {
    fs::read_to_string(file_name)
        .expect("File not found!")
        .lines()
        .map(|i| {
            let (action, value) = i.split_once(" ").unwrap();
            let action = action.parse::<T>().ok().unwrap();
            let value = value.parse::<U>().ok().unwrap();
            (action, value)
        })
        .collect::<Vec<(T, U)>>()
}
