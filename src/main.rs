fn main() {
  let s: &str = "abs";
  let ss = &s;
  println!("{}", s);
  println!("{}", *ss);
}

fn first_word(s: &str) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn ownership() {
  
  let s = "123";
  let s1 = "123";
  if &s == &s1 {
    println!("true");
  }

  let s2 = String::from("value");

  drop(s2);
}

fn datatype() {
  let mut x: u32 = 4;
  println!("The value of x is: {x}");

  x = 29;
  println!("The value of x is: {x}");

  const COSNT_Y: i32 = 200;
  print!("The value of y is: {COSNT_Y}");

  // Scalar Types
  let a: i32 = -12;
  let b: u32 = 1_199;
  let c: bool = false;
  let d: f64 = 10.213;
  let e: u32 = 0x12FF;
  let f: char = 'Z';
  let g: u8 = b'Z';
  let heart_eyed_cat = '😻';

  // Compound Types
  let tup: (u32, char, f32) = (23, 'a', 3.22);

  let (x, _, z) = tup;

  let arr: [i32; 3] = [1,2,3];
  let default_value_arr = [0, 20];

  let return_value = another_function(199);

  let result = condition_function(-9);
  println!("{result}");

  loop_function();
}

fn another_function(input: i32) -> i32 {
  let y = {
      let x = input;
      x + 1
  };

  println!("The value of y is: {y}");
  y
}

fn condition_function(input: i32) -> i32 {
  if input > 0 {
    input * input
  } else if input == 0 {
    input
  } else {
    - (input * input)
  }
}

fn loop_function() {
  let mut i = 0;
  loop {
    i += 1;
    println!("hello...");
    if (i > 4) {
      break;
    }
  }
  let mut i = 0;
  while i < 4 {
    i += 1;
    println!("...hello");
  }

  for element in (1..6).rev() {
    println!("the value is: {element}");
  }
}