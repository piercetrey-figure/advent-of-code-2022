use std::{env, fs};

pub fn get_input(day: i32, example: bool, qualifier: Option<&str>) -> String {
    let filename = format!(
        "{}/src/input/{}{}{}",
        env::current_dir().unwrap().to_str().unwrap(),
        day,
        if example { "example" } else { "" },
        qualifier
            .map(|q| format!("v{}", q))
            .unwrap_or("".to_string())
    );
    fs::read_to_string(filename).unwrap()
}
