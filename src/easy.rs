use std::io::{Write, Read};

pub fn input<S: AsRef<str>>(prompt: Option<S>) -> String {
  let stdin = std::io::stdin();
  let mut stdout = std::io::stdout();
  if prompt.is_some() {
    stdout.write(
        prompt
          .unwrap()
          .as_ref()
          .as_bytes())
      .expect("Cannot read STDOUT");
    stdout.flush().expect("Cannot flush STDOUT");
  }
  
  let mut input = String::new();
  stdin.read_line(&mut input)
    .expect("Cannot read STDIN"); 
  

  stdout.flush().expect("Cannot flush STDOUT");
  input[0..input.len()-2].to_string()
}