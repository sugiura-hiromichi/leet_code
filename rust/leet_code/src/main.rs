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
               p[i] = s[i];
            } else {
               if p[i + 1] == b'*' {
                  let pat = Solution::sub_pat(&p[i + 2..],);

                  if pat.len() == 0 {
                     let mut l = 2;
                     let mut min = 0;
                     loop {
                        if p[i + l] == b'*' {
                           min -= 1;
                        } else if p[i + l] == b'.' {
                           min += 1;
                        } else {
                           break;
                        }
                        l += 1;
                     }

                     if i + min >= s.len() {
                        return false;
                     }
                  }

                  if let Some(len,) = Solution::match_sub(&s[i..], pat,) {
                     let fill = &s[i..i + len];
                     let (l, r,) = (&p[..i], &p[i + 2..],);
                     let pat_len = pat.len();
                     p = [l, fill, r,].concat();
                     i += len + pat_len;
                  } else {
                     p.remove(i,);
                     p.remove(i,);
                  }
               } else {
                  p[i] = s[i];
                  i += 1;
               }
            }
         } else if p[i] == b'*' {
            //remind p[i-1] !=b'.'
            if i + 1 >= p.len() {
               // NOTE: p[i+1] may out of range
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
               if pat == &[] as &[u8] {
                  let s_rep = Solution::repeat_length(&s[i..], p[i - 1],);
                  p = [&p[..i], &s[i..i + s_rep], &p[i + 1..],].concat();
                  i += s_rep;
                  continue;
               }

               if let Some(l,) = Solution::match_sub(&s[i..], pat,) {
                  p = [&p[..i], &s[i..=i].repeat(l,), &p[i + 1..],].concat();
                  i += l;
               } else {
                  if pat.len() == 1 && i + 2 < p.len() && p[i + 2] == b'*' {
                     p.remove(i + 1,);
                     p.remove(i + 1,);
                  } else {
                     return false;
                  }
               }
            }
         } else if i + 1 < p.len() && p[i + 1] == b'*' {
            p.remove(i,);
            p.remove(i,);
         } else {
            return false;
         }
      }

      // CASE: s:"a", p:"ab*". This is true. but returns false if do noting
      p = Solution::trim_tail(&p, s.len(),);
      println!("s= {s:?}, p= {p:?}");
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

   fn trim_tail(p: &[u8], s_len: usize,) -> Vec<u8,> {
      if p.len() == s_len {
         return p.to_vec();
      }
      let tail = &p[s_len..];

      // CASE: Below for loop does not consider tail.len==1. Treat here.
      if tail.len() == 1 {
         if tail == &[b'*',] {
            return p[..s_len].to_vec();
         } else {
            return p.to_vec();
         }
      }

      for i in 0..tail.len() - 1 {
         if tail[i] != b'*' && tail[i + 1] != b'*' {
            return p.to_vec();
         }
      }

      if tail[tail.len() - 1] != b'*' {
         if tail[0] == b'*' {
            [&p[..s_len - 1], &p[p.len() - 1..p.len()],].concat()
         } else {
            p.to_vec()
         }
      } else {
         p[..s_len].to_vec()
      }
   }
}

fn main() {
   println!("1-------");
   assert_eq!(Solution::is_match("bbbba".to_string(), ".*a*a".to_string()), true);
   println!("2-------");
   assert_eq!(Solution::is_match("a".to_string(), ".*..a*".to_string()), false);

   println!("7-------");
   assert_eq!(Solution::is_match("mississippi".to_string(), "mis*is*ip*.".to_string()), true);
   println!("8-------");
   assert_eq!(Solution::is_match("mississippi".to_string(), "mis*is*p*.".to_string()), false);
   println!("9-------");
   assert_eq!(Solution::is_match("aaa".to_string(), "ab*a*c*a".to_string()), true);
   println!("10-------");
   assert_eq!(Solution::is_match("a".to_string(), "ab*".to_string()), true);
   println!("11-------");
   assert_eq!(Solution::is_match("a".to_string(), "a*".to_string()), true);
   println!("12-------");
   assert_eq!(Solution::is_match("aaa".to_string(), "aaaa".to_string()), false);
   println!("13-------");
   assert_eq!(Solution::is_match("a".to_string(), "ab*a".to_string()), false);
}
