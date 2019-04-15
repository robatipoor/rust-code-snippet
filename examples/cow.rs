use std::borrow::Cow;

fn main() {
    let mut c = Cow::from(String::from("hi"));
    println!("{}", upper_or_lower(&mut c))
}

fn upper_or_lower<'a>(s: &mut Cow<'a, str>) -> String {
    if s.len() > 5 {
        s.to_mut().to_uppercase()
    } else {
        s.to_lowercase()
    }
}
