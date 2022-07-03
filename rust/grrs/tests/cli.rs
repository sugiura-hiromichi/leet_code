use {
   assert_cmd::prelude::*,
   assert_fs::prelude::*,
   predicates::prelude::*,
   signal_hook::{consts::SIGINT, iterator::Signals},
   std::{process::Command, thread, time::Duration},
};

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
   let mut cmd = Command::cargo_bin("grrs")?;
   cmd.arg("foobar").arg("test/file/doesnt/exist");
   cmd.assert().failure().stderr(predicate::str::contains("cold not read file"));
   Ok(())
}

#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
   let f = assert_fs::NamedTempFile::new("sample.txt")?;
   f.write_str("A test\nActual content\nMore content\nAnother test")?;
   let mut cmd = Command::cargo_bin("grrs")?;
   cmd.arg("test").arg(f.path());
   cmd.assert().success().stdout(predicate::str::contains("test\nAnother test"));
   Ok(())
}

#[test]
fn passed_empty_pattern() -> Result<(), Box<dyn std::error::Error>> {
   let f = assert_fs::NamedTempFile::new("not_empty.txt")?;
   f.write_str("A test\nActual content\nMore content\nAnother test")?;
   let mut cmd = Command::cargo_bin("grrs")?;
   cmd.arg("test").arg(f.path());
   cmd.assert().failure().stderr(predicate::str::contains("pattern is empty"));
   Ok(())
}

#[test]
fn ctrlc() {
   ctrlc::set_handler(move || {
      println!("\nreceived <C-c>");
   }).expect("\nerr setting <C-c> handling");
}

#[test]
fn signal_hook() -> Result<(), Box<dyn std::error::Error>> {
   let mut signals = Signals::new(&[SIGINT])?;

   thread::spawn(move || {
      for sig in signals.forever() {
         println!("received signal {:?}", &sig);
      }
   });

   thread::sleep(Duration::from_secs(2));
   Ok(())
}
