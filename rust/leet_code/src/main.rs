struct Solution;
impl Solution {
   pub fn is_match(s: String, p: String,) -> bool {
      let (mut i, s, mut p,): (_, Vec<u8,>, Vec<u8,>,) =
         (0, s.as_bytes().into(), p.as_bytes().into(),);

      while i < s.len() {
         if i >= p.len() {
            return false;
         }

         if s[i] == p[i] {
            i += 1;
            //from here, s[i] != p[i]
         } else if p[i] == b'.' {
            if i + 1 >= p.len() {
               // BUG: 8 early return occurs here
               p[i] = s[i];
            } else {
               if p[i + 1] == b'*' {
                  let pat = Solution::sub_pat(&p[i + 2..],);

                  if pat.len() == 0 {
                     return true;
                  }

                  if let Some(len,) = Solution::match_sub(&s[i + 2..], pat,) {
                     let fill = &s[i + 2..i + 2 + len];
                     let (l, r,) = (&p[..i], &p[i + 2..],);
                     let pat_len = pat.len();
                     p = [l, fill, r,].concat();
                     i += 2 + len + pat_len;
                  } else {
                     return false;
                  }
               } else {
                  p[i] = s[i];
                  i += 1;
               }
            }
         } else if p[i] == b'*' {
            //remind p[i-1] !=b'.'
            if i + 1 >= p.len() {
               //NB p[i+1] may out of range
               return !s[i..].iter().any(|&u| u != p[i - 1],);
            }
            if p[i - 1] == p[i + 1] {
               let s_rep = Solution::repeat_length(&s[i + 1..], p[i + 1],);
               if s_rep == 0 {
                  p.remove(i,);
                  i += 1;
               } else {
                  let p_rep = Solution::repeat_length(&p[i + 1..], p[i + 1],);
                  if p_rep > s_rep {
                     return false;
                  }
                  let fill = &s[i + 1..=i + 1 + s_rep - p_rep];
                  p = [&p[..i], fill, &p[i + 1..],].concat();
                  i += s_rep;
               }
            } else {
               let pat = Solution::sub_pat(&p[i + 1..],);
               if pat == &[] {
                  let s_rep = Solution::repeat_length(&s[i..], p[i - 1],);
                  p = [&p[..i], &s[i..i + s_rep], &p[i + 1..],].concat();
                  i += s_rep;
                  continue;
               }

               if let Some(l,) = Solution::match_sub(&s[i..], pat,) {
                  p = [&p[..i], &s[i..=i].repeat(l,), &p[i + 1..],].concat();
                  i += l;
               } else {
                  println!("i is {i}");
                  return false;
               }
            }
         } else if i + 1 < p.len() && p[i + 1] == b'*' {
            p.remove(i,);
            p.remove(i,);
         } else {
            return false;
         }
      }

      //println!("s={s:?}, p={p:?}");
      s == p
   }

   ///Return length of string from head of `pat` which doesnt contain '.' & '*'
   fn sub_pat(pat: &[u8],) -> &[u8] {
      for l in 0..pat.len() {
         if pat[l] == b'.' || pat[l] == b'*' {
            return &pat[0..l];
         }
      }
      pat
   }

   ///Return first index of `sub` which match `pat`. If didn't match, return None
   fn match_sub(sub: &[u8], pat: &[u8],) -> Option<usize,> {
      if sub.len() < pat.len() {
         return None;
      }
      for l in 0..=sub.len() - pat.len() {
         if &sub[l..pat.len() + l] == pat {
            return Some(l,);
         }
      }
      return None;
   }

   ///Calculate number of repeating `r` in `s`
   fn repeat_length(s: &[u8], r: u8,) -> usize {
      for i in 0..s.len() {
         if s[i] != r {
            return i;
         }
      }
      return s.len();
   }
}

fn main() {
   println!("7-------");
   assert_eq!(Solution::is_match("mississippi".to_string(), "mis*is*ip*.".to_string()), true);
   println!("8-------");
   assert_eq!(Solution::is_match("mississippi".to_string(), "mis*is*p*.".to_string()), false);
   println!("9-------");
   assert_eq!(Solution::is_match("aaa".to_string(), "ab*a*c*a".to_string()), true);
}
