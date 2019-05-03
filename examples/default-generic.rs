type IOResult<T = ()> = Result<T, std::io::Error>;

fn main() {
    let _i: IOResult<i32> = Result::Ok(0);
    let _d: IOResult = Result::Ok(());
}
