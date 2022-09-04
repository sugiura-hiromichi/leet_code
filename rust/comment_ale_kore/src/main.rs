//! This is DocComment for file itself
//! ---
//!  
//! *italic*
//!
//! **bold**
//!
//! - list
//! - list
//!
//! [link](https://github.com/ah-y)
//!
//! # Header
//!
//! body
//!
//! ## SubHeader
//!
//! body
//!
//! ~~strikethrough~~
//!
//! > quotation
//!
//! inline code `println!("Good Night Garden...")`
//!
//! ```rust
//! //code block
//! println!("good_night_garden..");
//! ```
//!
//! |table|notation|none|
//! |:---|:---:|--:|
//! |right_align|centered|left_align|

mod commented_module {
   //! This is DocComment for module

   //! This is DocComment for struct
   struct CommentedStruct {}
}

/// This is normal DocComment
///
/// # Panic
///
/// panic!
fn main() {
   //This is normal Comment
   println!("Hello, world!");
}