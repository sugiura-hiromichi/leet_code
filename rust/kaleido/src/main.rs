use std::char;

//lexer-----------------------------------------------------------------------------------------------------
#[derive(PartialEq, Clone, Debug,)]
enum Token {
   TokEof,

   //commands
   TokDef,
   TokExtern,

   //primary
   TokIdentifier(String,), //IdentifierStr always called when Token is identifier.
   TokNumber(f64,),        // same as above

   Other(char,),
}

/// Spec of "static" is different between C++ and Rust.
/// Thus, here, I use struct instead.
struct Buf {
   lst_chr: Option<char,>,
   curtok:  Option<Token,>,
   content: String,
}

impl Buf {
   ///this fn is responsible for stdin
   fn new() -> Buf {
      let mut content = String::new();
      let mut content = match std::io::stdin().read_line(&mut content,) {
         Ok(_n,) => content,
         Err(err,) => panic!("io error: {err}"),
      };

      let lst_chr = Some(content.remove(0,),);

      Buf { lst_chr, curtok: None, content, }
   }

   ///return the next token from buffer
   fn gettok(&mut self,) -> Token {
      //skip any whitespace
      while self.lst_chr != None && self.lst_chr.unwrap().is_whitespace() {
         self.lst_chr = self.getchar();
      }

      //identifier: [a-zA-Z][a-zA-Z0-9]*
      if self.lst_chr != None && self.lst_chr.unwrap().is_alphabetic() {
         let mut token_content = String::from(self.lst_chr.unwrap(),);

         loop {
            self.lst_chr = self.getchar();
            if self.lst_chr != None && self.lst_chr.unwrap().is_alphanumeric() {
               token_content.push(self.lst_chr.unwrap(),);
            } else {
               //self.content.insert(0, self.last_char.unwrap());
               break;
            }
         }

         if token_content == "def" {
            return Token::TokDef;
         }
         if token_content == "extern" {
            return Token::TokExtern;
         }
         return Token::TokIdentifier(token_content,);
      }

      //number: [0-9.]+
      if self.lst_chr != None && (self.lst_chr.unwrap().is_numeric() || self.lst_chr.unwrap() == '.') {
         let mut num = String::new();

         let num = loop {
            num.push(self.lst_chr.unwrap(),);
            self.lst_chr = self.getchar();
            if self.lst_chr != None && !(self.lst_chr.unwrap().is_numeric() || self.lst_chr.unwrap() == '.') {
               //self.content.insert(0, self.last_char.unwrap());
               break num;
            }
         }
         .parse::<f64>()
         .unwrap();

         return Token::TokNumber(num,);
      }

      //Comment until end of line
      if self.lst_chr != None && self.lst_chr.unwrap() == '#' {
         loop {
            self.lst_chr = self.getchar();
            if self.lst_chr == None || self.lst_chr == Some('\r',) || self.lst_chr == Some('\n',) {
               break;
            }
         }

         if self.lst_chr != None {
            return self.gettok();
         }
      }

      if self.lst_chr == None {
         return Token::TokEof;
      }

      let tok = Token::Other(self.lst_chr.unwrap(),);
      self.lst_chr = self.getchar();
      tok
   }

   /// curtok & peak_token provide simple token buffer
   fn peek_token(&mut self,) -> Token {
      let tok = self.gettok();
      self.curtok = Some(tok.clone(),);
      tok
   }

   /// helper fn for gettok()
   fn getchar(&mut self,) -> Option<char,> {
      if self.content.len() == 0 {
         return None;
      }
      let head = self.content.remove(0,);
      self.lst_chr = Some(head,);
      Some(head,)
   }
}

//AST-------------------------------------------------------------------------------------------------
///helper for binary operator
#[derive(PartialEq, PartialOrd, Eq, Ord,)]
enum Op {
   NotOp,
   Nothing, //this is needed when token is head of exp
   //Cmp,     //Cmp is < > =< >= but here, only consider <
   AddSub(char,),
   AddSubR(char,), //this is needed in parse_bin_op_rhs() of where it's recursive session
   MulDiv(char,),
   MulDivR(char,),
}

/// base for all expression nodes
/// all these are exp. Thus they return value
enum ExprAST {
   Num(f64,),                                 //exp_type for numeric literals like "3.0"
   Var(String,),                              //exp_type for referencing a variable like "a"
   Binary(Op, Box<ExprAST,>, Box<ExprAST,>,), //exp_type for binary operator. Binary(Operator, Lhs, Rhs)
   Call(String, Vec<ExprAST,>,),              //exp_type for fn call. Call(Callee, Args)
}

/// base for all statement nodes
enum StateAST {
   Prototype(String, Vec<String,>,), // represent the "Prototype". Capture its name and arguments. Prototype(Name, Args)
   Function(Box<StateAST,>, ExprAST,), //represent a function definition. Function(Proto, Body)
}

//parser----------------------------------------------------------------------------------------
/// helper fn for error handling
fn log_err(err: String,) -> Result<ExprAST, (),> {
   eprintln!("{err}");
   Err((),)
}
/// helper fn for error handling
fn log_errp(err: String,) -> Result<StateAST, (),> {
   eprintln!("{err}");
   Err((),)
}

/// this fn doesn't eat token
fn op_prec(buf: &mut Buf,) -> Op {
   match buf.curtok.clone().unwrap() {
      Token::Other('+',) => Op::AddSub('+',),
      Token::Other('-',) => Op::AddSub('-',),
      Token::Other('*',) => Op::MulDiv('*',),
      Token::Other('/',) => Op::MulDiv('/',),
      _ => Op::NotOp, // AddSubR and MulDivR shouldn't be appeared out of parse_bin_op_rhs(). That should be an error
   }
}

/// exp::=primary binoprhs
fn parse_exp(buf: &mut Buf,) -> Result<ExprAST, (),> {
   let lhs = parse_primary(buf,)?;
   parse_bin_op_rhs(buf, lhs, Op::Nothing,)
}

/// numexp::=number
fn parse_num_exp(buf: &mut Buf,) -> Result<ExprAST, (),> {
   if let Token::TokNumber(i,) = buf.curtok.clone().unwrap() {
      buf.peek_token();
      Ok(ExprAST::Num(i,),)
   } else {
      buf.peek_token();
      log_err("in parse_num_exp: from:[parse_num_exp] -not number".to_string(),)
   }
}

/// parenexp::='('expression')'
fn parse_paren_exp(buf: &mut Buf,) -> Result<ExprAST, (),> {
   buf.peek_token(); //eat (.
   let in_paren = parse_exp(buf,)?;

   // if paren doesn't end, it's illegal.
   if let Token::Other(')',) = buf.curtok.clone().unwrap() {
      buf.peek_token(); //eat ).
      Ok(in_paren,)
   } else {
      log_err("in parse_paren_exp: expected ')'".to_string(),)
   }
}

/// identifierexp::=identifier'('exp*')'
fn parse_identifier_exp(buf: &mut Buf,) -> Result<ExprAST, (),> {
   let id_name = match buf.curtok.clone().unwrap() {
      Token::TokIdentifier(id_name,) => id_name,
      _ => return log_err("in parse_identifier_exp: expected identifier".to_string(),),
   }; // store identifier's name

   buf.peek_token(); //eat identifier
   if buf.curtok.clone().unwrap() != Token::Other('(',) {
      Ok(ExprAST::Var(id_name,),) //simple var ref
   } else {
      buf.peek_token(); //eat '('  // fn call
      let mut args: Vec<ExprAST,> = Vec::new();

      while buf.curtok.clone().unwrap() != Token::Other(')',) {
         let arg = parse_exp(buf,)?;
         args.push(arg,);
         if buf.curtok.clone().unwrap() != Token::Other(',',) {
            return log_err("in parse_identifier_exp: Expected ')' or ',' in argument list".to_string(),);
         };
         buf.peek_token();
      }
      buf.peek_token(); //eat ')'
      Ok(ExprAST::Call(id_name, args,),)
   }
}

/// primary::=identifierexp | numexp | parenexp
/// primary means primary arithmetic item of equation
fn parse_primary(buf: &mut Buf,) -> Result<ExprAST, (),> {
   match buf.curtok.clone().unwrap() {
      Token::TokIdentifier(_id,) => parse_identifier_exp(buf,),
      Token::TokNumber(_i,) => parse_num_exp(buf,),
      Token::Other('(',) => parse_paren_exp(buf,),
      _ => log_err("in parse_primary: expected identifier or number or '('".to_string(),),
   }
}

/// binoprhs::=( 'operator' primary )*
fn parse_bin_op_rhs(buf: &mut Buf, mut lhs: ExprAST, cur_op: Op,) -> Result<ExprAST, (),> {
   loop {
      let last_op = op_prec(buf,);

      //if cur_op is prior, we are done
      if last_op < cur_op {
         return Ok(lhs,);
      }

      buf.peek_token(); //eat binop
      let mut rhs = parse_primary(buf,)?; // Parse the primary exp after the binary operator

      // if last_op binds less tightly with rhs than the operator after rhs, let the
      // pending oerator take rhs as its lhs because current rhs have to be a
      // part of actual one
      let next_op = op_prec(buf,);
      if last_op < next_op {
         let lsp_r = match last_op {
            Op::AddSub(a,) => Op::AddSubR(a,),
            Op::MulDiv(b,) => Op::MulDivR(b,),
            _ => return log_err("in parse_bin_op_rhs: parsing should be uncertain".to_string(),),
         };
         rhs = parse_bin_op_rhs(buf, rhs, lsp_r,)?;
      }

      // merge lhs/rhs
      lhs = ExprAST::Binary(last_op, Box::new(lhs,), Box::new(rhs,),);
   }
}

/// prototype::=id '(' id* ')'
fn parse_proto(buf: &mut Buf,) -> Result<StateAST, (),> {
   match buf.curtok.clone().unwrap() {
      Token::TokIdentifier(id,) => {
         let fn_name = id;
         buf.peek_token();

         if buf.curtok.clone().unwrap() != Token::Other('(',) {
            return log_errp("in parse_proto: expected '(' in Prototype".to_string(),);
         }

         let mut arg_names: Vec<String,> = Vec::new();
         while let Token::TokIdentifier(id,) = buf.peek_token() {
            arg_names.push(id,);
         }

         if buf.curtok.clone().unwrap() != Token::Other(')',) {
            return log_errp("in parse_proto: expected ')' in Prototype".to_string(),);
         }

         buf.peek_token(); // eat ')'
         Ok(StateAST::Prototype(fn_name, arg_names,),)
      }
      _ => log_errp("in parse_proto: expected fn name in Prototype".to_string(),),
   }
}

/// definition::= 'def' prototype exp
fn parse_def(buf: &mut Buf,) -> Result<StateAST, (),> {
   buf.peek_token(); // eat def
   let proto = parse_proto(buf,)?;
   let exp = parse_exp(buf,)?;
   Ok(StateAST::Function(Box::new(proto,), exp,),)
}

/// toplevelexp::=exp
fn parse_tle(buf: &mut Buf,) -> Result<StateAST, (),> {
   let exp = parse_exp(buf,)?;
   let proto = StateAST::Prototype("__anon_expr".to_string(), {
      let null_arg: Vec<String,> = Vec::new();
      null_arg
   },); //evaluate a top-level expression into an anonymous fn

   Ok(StateAST::Function(Box::new(proto,), exp,),)
}

/// external::='extern' prototype
fn parse_extern(buf: &mut Buf,) -> Result<StateAST, (),> {
   buf.peek_token(); //eat extern
   parse_proto(buf,)
}

// top-level parsing------------------------------------------------------------------
/// treat definition of fn
fn handle_def(buf: &mut Buf,) {
   if let Ok(_stat,) = parse_def(buf,) {
      println!("parsed a fn def")
   } else {
      eprintln!("in handle_def: couldn't handle def");
      buf.peek_token(); //skip token for error recovery
   }
}

/// treat extern keyword
fn handle_extern(buf: &mut Buf,) {
   if let Ok(_stat,) = parse_extern(buf,) {
      println!("parsed an extern")
   } else {
      eprintln!("in handle_extern: couldn't handle extern");
      buf.peek_token();
   }
}

/// treat toplevelexp
/// top::=def | extern | exp | ';'
fn handle_tle(buf: &mut Buf,) {
   if let Ok(_stat,) = parse_tle(buf,) {
      println!("parsed a top-level exp")
   } else {
      eprintln!("in handle_tle: couldn't handle toplevelexp");
      buf.peek_token();
   }
}

fn main_loop(buf: &mut Buf,) {
   loop {
      match buf.curtok.clone().unwrap() {
         Token::TokEof => break,
         Token::TokDef => handle_def(buf,),
         Token::TokExtern => handle_extern(buf,),
         Token::Other(';',) => {
            buf.peek_token();
         }
         _ => handle_tle(buf,),
      }
      println!("ready>");
   }
}

fn main() {
   println!("ready>");
   let mut buf = Buf::new();
   buf.peek_token(); // prime the first token
   main_loop(&mut buf,);
}
