use std::string;

pub trait Capitalize {
    fn capitalize(self) -> String;
}
impl Capitalize for String {
    fn capitalize(self) -> String {
        let mut c = self.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    }
}

fn get_bottle_count(bottle_nb: u32) -> String {
    return match bottle_nb {
        0 => "no more bottles of beer".to_string(),
        1 => "1 bottle of beer".to_string(),
        _ => format!("{} bottles of beer", bottle_nb),
    };
}

fn get_first_part(verse_nb: u32) -> String {
    format!("{} on the wall", get_bottle_count(verse_nb).capitalize())
}

fn get_second_part(verse_nb: u32) -> String {
    get_bottle_count(verse_nb)
}

fn get_third_part(verse_nb: u32) -> String {
    match verse_nb {
        0 => return "Go to the store and buy some more".to_owned(),
        1 => return "Take it down and pass it around".to_owned(),
        _ => return "Take one down and pass it around".to_owned(),
    }   
}

fn get_fourth_part(verse_nb: u32) -> String {
    match verse_nb {
        0 => format!("{} on the wall", get_bottle_count(99)),
        _ => format!("{} on the wall", get_bottle_count(verse_nb - 1))
    }
}

pub fn verse(n: u32) -> String {
    let first_part = get_first_part(n);
    let second_part = get_second_part(n);
    let third_part = get_third_part(n);
    let fourth_part = get_fourth_part(n);

    format!(
        "{}, {}.\n{}, {}.\n",
        first_part, second_part, third_part, fourth_part
    )
}

pub fn sing(start: u32, end: u32) -> String {
    let mut song : Vec<String>= vec!();

    for i in (end..=start).rev() {
        song.push(verse(i));
    }

    song.join("\n")
}
