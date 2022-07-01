use std::any::{Any, TypeId};

fn main() {
   //Rust can define function in function, return closure intuitive way by using
   // impl trait.
   fn return_closure() -> impl Fn() -> i32 {
      fn fn_in_main() -> impl Fn(String,) -> String { |x| x }
      || fn_in_main()("7".to_string(),).parse::<i32>().unwrap()
   }

   println!("{}", return_closure()());

   //How rust identify closure itself and closure's return value in fn's argument
   fn closure_ornot<GenT: 'static,>(_which: GenT,) {
      //typeid of generic parameter's type
      let gen_id = TypeId::of::<GenT,>();
      if TypeId::of::<i32,>() == gen_id {
         println!("GenT is i32");
      } else if TypeId::of::<dyn Fn() -> i32,>() == gen_id {
         println!("GenT is Fn()->i32");
      } else {
         println!("unexpected!!");
      }
   }

   closure_ornot(return_closure()(),);

   //closure's type is different even arguments and return types are same
   //this call's "anonymous type"
   let cl1 = || 1;
   let cl2 = || 2;
   println!("{}", cl1.type_id() == cl2.type_id());

   //fn's type is same if types of all arguments and return value are same
   fn ret1() -> i32 { 1 }
   fn ret2() -> i32 { 2 }
   println!("{}", ret1.type_id() == ret2.type_id());

   //========================================================================

   //rust's trait has alot advanced features:D
   trait MyName {
      fn is(&self,);
   }

   impl<T,> MyName for Vec<T,> {
      fn is(&self,) {
         println!("!Vec<T>!");
      }
   }

   impl MyName for i32 {
      fn is(&self,) {
         println!("!int!");
      }
   }

   impl<T,> MyName for (i32, Vec<T,>,) {
      fn is(&self,) {
         println!("!(i32, Vec<T>)!");
      }
   }

   let v = vec![0, 1, 1, 2, 23];
   let ai = 0;
   v.is();
   ai.is();
   (ai, v,).is();

   //methods (in this case, is()) are only able to use within defined crate.
   // This guarantees safety

   //==========================================================================

   //experiment of behavior of std::io::. Whether read_line rewrite buf or not?
   /*
   let mut buf = String::new();
   let _ = std::io::stdin().read_line(&mut buf,);
   let _ = std::io::stdin().read_line(&mut buf,);
   println!("{buf}");
   */

   //=========================================================================
   let a = Some("a",);
   for i in a.iter() {
      println!("{i}");
   }
}
