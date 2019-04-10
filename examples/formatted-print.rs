

pub fn main() {
  print!("{}{} ", "Hello","World!");
  println!("{1}{0} ", "World!","Hello");
  println!("{h}{w}",h="Hello",w="World");
  eprintln!("{}","Print Error");
  let string = format!("{} {} {}","Create","New","String");
  print!("{}",string);
}
