use std::any::{Any, TypeId};

///Documentation
fn main() {
   //Rust can define function in function, return closure intuitive way by using
   // impl trait.

   ///This returns closure
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

   //closure's type is different even arguments and return types are same this
   // call's "anonymous type"
   let cl1 = || 1;
   let cl2 = || 2;
   println!("{}", cl1.type_id() == cl2.type_id());

   //fn's type is same if types of all arguments and return value are same
   fn ret1() -> i32 { 1 }
   fn ret2() -> i32 { 2 }
   println!("{}", ret1.type_id() == ret2.type_id());

   //=============================================================

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

   //===============================================================

   //experiment of behavior of std::io::. Whether read_line rewrite buf or not?
   /*
   let mut buf = String::new();
   let _ = std::io::stdin().read_line(&mut buf,);
   let _ = std::io::stdin().read_line(&mut buf,);
   println!("{buf}");
   */

   //==============================================================

   //Option<T> supports iterator
   let a = Some("a",);
   for i in a.iter() {
      println!("{i}");
   }

   //===============================================================

   //Rust's pattern matching arm & @ binding
   fn odd(i: i32,) -> bool {
      if i % 2 == 0 {
         true
      } else {
         false
      }
   }

   match 3 {
      c @ (4..=8 | 33) => println!("{c} is 4..=8 or 33"),
      c if odd(c,) => println!("{c} is odd"),
      666 if false => println!("†666†"),
      c @ (11 | 22) => println!("{c} is 11 or 22"),
      _ => println!("I know that I know nothing"),
   }

   //if keyword in match arm & @ binding is similar, but following example is
   // not compliable
   /*
   match 3 {
      c if (4..=8 | 33) => println!("{c} is 4..=8 or 33"),
      c @ odd(c,) => println!("{c} is odd"),
      666 @ false => println!("†666†"),
      c if (11 | 22) => println!("{c} is 11 or 22"),
      _ => println!("I know that I know nothing"),
   }
   */

   //===============================================================

   //Option::map uses raw-value if is Some().
   //But how about self is None?
   let mut some_none = None;
   some_none.map(|_one| panic!("This painc shouldn't be executed"),);
   some_none = Some(1,);
   some_none.map(|one| println!("{one}"),);

   //===============================================================

   //Checking idea that returning private method's pointer enables to access private method
   //Result is bad at rust-nightly 1.64.0
   let has_prv = HasPrivate { pub_member: 0, private_member: 0, };
   has_prv.pub_f()();
}

struct HasPrivate {
   pub pub_member: usize,
   private_member: usize,
}
impl HasPrivate {
   pub fn pub_f(self,) -> impl Fn() -> () {
      //self.prv_fn | this cause error. compiler recognize as field, not method
      || println!("from pub_f")
   }

   fn prv_fn(self,) { println!("in plivate function") }
}
