fn main() {
    println!("apple:{}", pig_latin("apple"));
    println!("eat:{}", pig_latin("eat"));
    println!("first:{}", pig_latin("first"));
    println!("(empty):{}", pig_latin(""));
    println!("a:{}", pig_latin("a"));
    println!("ai:{}", pig_latin("ai"));
    println!("v:{}", pig_latin("v"));
    println!("we:{}", pig_latin("we"));
}

fn pig_latin(str: &str) -> String {
    let c = str.chars().nth(0);
    match c {
        Some('a') | Some('i') |Some('u') |
        Some('e') | Some('o') => pig_latin_hay(str),
        Some(_) => pig_latin_ay(str),
        None => String::from(""),
    }
}

fn pig_latin_ay(str: &str) -> String {
    if str.chars().count() < 2 {
        return str.to_string();
    }

    str.chars().skip(1).collect::<String>()
        + "-"
        + str.chars().nth(0).unwrap().to_string().as_str()
        + "ay"
}

fn pig_latin_hay(str: &str) -> String {
    str.to_string() + "-hay"
}