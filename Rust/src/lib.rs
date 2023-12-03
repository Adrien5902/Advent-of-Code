use std::fs::read;

pub fn loadinput(day_id: &str) -> String {
    String::from_utf8(read(format!("../../Inputs/{}.txt", day_id)).expect("file not found"))
        .expect("")
}
