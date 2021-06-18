use if_empty::*;

fn main() {
    let s = String::default();
    let s_ = "";
    println!("'{}' should be '{}'", s.clone(), s.if_empty("with content".to_string()));
    println!("'{}' -> '{}'", s_, s_.if_empty("not anymore"));
}

