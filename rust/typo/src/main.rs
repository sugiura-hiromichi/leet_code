use {rand::prelude::*, std::fs};

/// argument is numeral
macro_rules! cin {
   ($b:expr) => {{
      let mut buf = String::new();
      for _ in 0..$b {
         if let Err(e,) = std::io::stdin().read_line(&mut buf,) {
            panic!("{e}");
         }
      }

      buf.split_whitespace().map(|w| w.to_string(),).collect::<Vec<String,>>()
   }};
}

fn correct(original: String,) -> bool {
   let answer = cin!(1).get(0,).unwrap().clone();
   if answer == original {
      true
   } else {
      println!("x-x-x-x-x-x-x-x-x");
      correct(original,)
   }
}

fn main() {
   let mut f: Vec<String,> = fs::read_to_string("txt/difficult.txt",)
      .unwrap()
      .split_whitespace()
      .map(|word| word.to_string(),) //avoid using &str because I'm not familiar with it
      .collect();
   let mut rng = thread_rng();
   let problems = f.len();

   for _ in 0..problems {
      let i: usize = rng.gen_range(0..f.len(),);
      let original = f.remove(i,);
      println!("\n{original}");
      correct(original,);
   }
}
