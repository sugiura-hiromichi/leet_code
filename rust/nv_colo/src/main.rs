use {rand::prelude::*, std::process::Command};
fn main() {
   let colors = vec!["PaperColor", "flatui", "nova"];
   let colo = colors[thread_rng().gen_range(0..colors.len(),)];
   Command::new("nvim",).arg(format!("+\"colorscheme {}\"", colo),);
}
