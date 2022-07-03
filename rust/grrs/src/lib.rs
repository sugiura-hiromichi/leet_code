use {
   anyhow::{Context, Result},
   std::io::{BufRead, Write},
};

/// code that [writer: impl io::Write] is awesome grammer,
/// because this shows "writer" needs to be implemented io::Write in a
/// intuitive, easy to understand, way
pub fn find_match(reader: impl BufRead, pattern: &str, mut writer: impl Write) -> Result<()> {
   for line in reader.lines() {
      let line = line.with_context(|| format!("couldn't read file"))?;
      if line.contains(pattern) {
         writeln!(writer, "{line}")?;
      }
   }
   Ok(())
}
