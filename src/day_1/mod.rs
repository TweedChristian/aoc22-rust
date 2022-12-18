

mod load_elves {
  use std::fs::File;
  use std::io::{BufReader, BufRead};

  pub fn create_elf_calorie_list() -> Vec<i32> {
    let mut calorie_list: Vec<i32> = vec![];
    
    let path: &str = "src/day_1/input_1.txt";
    let file: File = match File::open(path) {
      Ok(loaded_file) => loaded_file,
      Err(error) => {
        panic!("Problem opening file: {:?}", error);
      }
    };

    let buffered: BufReader<File> = BufReader::new(file);
    let mut current_calorie_count = 0;

    for line in buffered.lines() {
      let line_val = line.unwrap().parse::<i32>();
      match line_val {
          Ok(calories) => {
            current_calorie_count += calories;
          },
          Err(_) => {
            calorie_list.push(current_calorie_count);
            current_calorie_count = 0;
          },
      };
    }
    calorie_list.sort();
    return calorie_list;
  }
}

pub fn problem_1() -> i32 {
    return match crate::day_1::load_elves::create_elf_calorie_list().last() {
      Some(last_item) => *last_item,
      None => panic!("Day 1: Last Item Not Found"),
    };
}

pub fn problem_2() -> i32 {
  let calorie_list = crate::day_1::load_elves::create_elf_calorie_list();
  let top_three = calorie_list.get(calorie_list.len() - 3..calorie_list.len());

  return match top_three {
    Some(elf_calories) => elf_calories.iter().sum(),
    None => panic!("Last three items not present")
  }
}

