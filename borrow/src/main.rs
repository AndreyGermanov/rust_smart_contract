use std::str::FromStr;

fn concatenate_strings(s1: &str, s2: &str) -> String {
    let mut result = s1.to_string();
    result.push_str(s2);
    result
}

fn main() {
    let (s1,s2) = (String::from_str("Hello ").unwrap(),String::from_str("world!").unwrap());
    let concatenated_string = concatenate_strings(&s1, &s2);
    println!("{}",concatenated_string);
}
