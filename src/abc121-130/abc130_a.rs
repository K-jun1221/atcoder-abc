fn main() {
  let mut buffer = String::new();
  std::io::stdin().read_line(&mut buffer).ok();
  let (x, a) = {
    let mut input = buffer.split_whitespace();
    (
      input.next().unwrap().parse::<i32>().unwrap(),
      input.next().unwrap().parse::<i32>().unwrap(),
    )
  };

  if x < a {
    println!("0");
  } else {
    println!("10");
  }
}