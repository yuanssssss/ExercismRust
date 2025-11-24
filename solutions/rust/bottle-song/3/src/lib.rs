use std::fmt::Write;

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut out = String::new();
    if take_down == 0 {
        return out;
    }

    let start = start_bottles;
    let end = start_bottles - take_down + 1;

    for n in (end..=start).rev() {
        let head = format!("{}", NumString::from(n));
        let bottle_word = if n == 1 { "bottle" } else { "bottles" };

        writeln!(&mut out, "{head} green {bottle_word} hanging on the wall,").unwrap();
        writeln!(&mut out, "{head} green {bottle_word} hanging on the wall,").unwrap();
        writeln!(&mut out, "And if one green bottle should accidentally fall,").unwrap();

        let remaining = n.saturating_sub(1);
        let rem_word = if remaining == 0 {
            "no".to_string()
        } else {
            format!("{}", NumString::from(remaining)).to_lowercase()
        };
        let rem_bottle_word = if remaining == 1 { "bottle" } else { "bottles" };

        writeln!(&mut out, "There'll be {rem_word} green {rem_bottle_word} hanging on the wall.").unwrap();

        if n != end {
            writeln!(&mut out).unwrap();
        }
    }

    out
}

pub enum NumString{
    No,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten
}

impl From<u32> for NumString {
    fn from(n: u32) -> Self {
        match n {
            0 => NumString::No,
            1 => NumString::One,
            2 => NumString::Two,
            3 => NumString::Three,
            4 => NumString::Four,
            5 => NumString::Five,
            6 => NumString::Six,
            7 => NumString::Seven,
            8 => NumString::Eight,
            9 => NumString::Nine,
            10 => NumString::Ten,
            _ => panic!("Number out of range"),
        }
    }
}
impl std::fmt::Display for NumString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            NumString::No => "No",
            NumString::One => "One",
            NumString::Two => "Two",
            NumString::Three => "Three",
            NumString::Four => "Four",
            NumString::Five => "Five",
            NumString::Six => "Six",
            NumString::Seven => "Seven",
            NumString::Eight => "Eight",
            NumString::Nine => "Nine",
            NumString::Ten => "Ten",
        };
        write!(f, "{s}")
    }
}