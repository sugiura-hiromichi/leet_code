use {
   rand::prelude::*,
   std::{os::unix::prelude::CommandExt, process::Command},
};

fn main() {
   let path = std::env::args().nth(1,).unwrap_or("~/.config/nvim/init.vim".to_string(),);
   let colors = vec!["PaperColor", "flatui", "nova"];
   let color = colors[thread_rng().gen_range(0..colors.len(),)];
   let colo = format!("-c color {}", color);
   Command::new("nvim",).args([&colo, &path,],).exec();
}
