fn main() {
    let string = "this is text".to_owned();
    let _s: &str = &string; // deref coercion
    let vector = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let _slice: &[i32] = &vector; // deref coercion
    // other sample CString/CStr , OsString/OsStr , PathBuf/Path
}
