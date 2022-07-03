use {
   anyhow::Result,
   clap::Parser,
   grrs::find_match,
   std::{
      fs,
      io::{self, BufReader},
      path::PathBuf,
   },
};

/// Search for a pattern in a file and display the lies that contain it
#[derive(Parser,)]
struct Cli {
   /// the pattern to look for
   pattern: String,

   /// the path to the file to read
   #[clap(parse(from_os_str))]
   path: PathBuf,
}

fn main() -> Result<(),> {
   let args = Cli::parse();
   let f = fs::File::open(&args.path,).expect("no files found",);

   //for faster io
   //Each time using println! macro, rust rock and unrock over and over. Thus,
   // handle lock and print directly helps performance optm
   let reader = BufReader::new(f,);
   let stdout = io::stdout();
   let writer = stdout.lock();

   find_match(reader, &args.pattern, writer,)
}

#[test]
fn match_test() {
   let mut rslt = Vec::new();
   let _ = find_match(BufReader::new("a\ni\nu\nuuuuueeee\no".as_bytes(),), "u", &mut rslt,);
   assert_eq!(rslt, b"u\nuuuuueeee\n")
}
