use std::fs::read_to_string;
use rpassword::prompt_password;

fn open_pins_file<S: AsRef<str>>(file_name: S) -> Vec<String> {
  let file_contents: String = match read_to_string(file_name.as_ref()) {
    Ok(values) => values,
    Err(e) => panic!("Cannot read {} file: {}", file_name.as_ref(), e)
  };

  let tmp_vec: Vec<&str> = file_contents.split("\r\n").collect();
  let mut content_vec: Vec<String> = Vec::new();

  for i in 0..tmp_vec.len() {
    content_vec.push(tmp_vec[i].to_string());
  }

  content_vec
}

fn linear_search<S: AsRef<str>>(vec: &Vec<String>, target: &S) -> u64 {
  let mut index: usize = 0;
  let copied_vector: Vec<String> = vec.clone();
  for i in 0..copied_vector.len() {
    if copied_vector[i] == target.as_ref() {
      index = i;
      break;
    }
  }
  index as u64 + 1
}

fn get_percentage(useful: u64, total: u64) -> u64 {
  let mut result = ((useful as f32 / total as f32) * 100 as f32) as u64;
  if result == 0 {
    result = 1
  }
  result
}

fn main() -> Result<(), String>{
  let content: Vec<String> = open_pins_file("pins.txt"); 
  println!("Check your 4 numeric pin frequency!!");
  let pin: String = prompt_password("Password: ").expect("Cannot get password");
  // let pin:String = "1234".to_owned();

  if pin.len() != 4 { return Err(String::from("Pin length needs to be 4 digits")); }
  match pin.parse::<u64>() {
    Ok(_) => {},
    Err(e) => panic!("Please enter a 4 digit number: {}", e)
  }
  
  let pos = linear_search(&content, &pin);
  println!("Part of the {:?}% of the easist passwords", get_percentage(pos.clone(), content.clone().len() as u64 ));
  println!("Is guessed {:?}th out of a total of {:?} pins", pos.clone(),  content.len());

  Ok(())
}