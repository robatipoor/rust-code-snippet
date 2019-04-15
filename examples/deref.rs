use std::ops::Deref;

struct Token<'a> {
    inner: &'a str,
}

impl<'a> Deref for Token<'a> {
    type Target = &'a str;
    /// Dereferences the value.
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

fn main() {
    let token = Token {
        inner:"some string ...."
    };
    println!("{}", *token);
}
