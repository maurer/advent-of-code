use std::collections::HashSet;
use quote::{quote, format_ident};
use proc_macro2::{Ident, Span};
use proc_macro2::TokenStream;
/// I am bad at writing parsers quickly, but AoC's formats are simple enough that I think they can
/// just be straight up guessed. This program is an attempt to convert from an input (either the
/// sample or the real input, though the real one will probably work better) into a parser and
/// skeleton so I can jump right in to the interesting part.
///
/// I assume that the format of the input is
///
/// (Header\n\n)?(Record)*
///
/// where
/// A Header is either a tuple or vector of fields, separated by whitespace or commas
/// A Record is either
///     A single line, as a tup
use std::str::FromStr;

fn capitalize(name: &mut String) {
    if let Some(r) = name.get_mut(0..1) {
        r.make_ascii_uppercase();
    }
}

#[derive(PartialEq, Eq, Ord, PartialOrd, Hash, Clone, Debug)]
enum TokenType {
    String,
    SInt,
    UInt,
}

impl TokenType {
    fn matches(&self, token: &str) -> bool {
        use TokenType::*;
        match *self {
            String => true,
            UInt => usize::from_str(token).is_ok(),
            SInt => isize::from_str(token).is_ok(),
        }
    }

    fn gen(token: &str) -> HashSet<TokenType> {
        use TokenType::*;
        let mut out = HashSet::new();
        for tt in [String, UInt, SInt].into_iter() {
            if tt.matches(token) {
                out.insert(tt);
            }
        }
        out
    }

    fn to_fragment(&self) -> TokenStream {
        use TokenType::*;
        match *self {
            String => quote! { String::from(tok) },
            UInt => quote! { usize::from_str(tok).unwrap() },
            SInt => quote! { isize::from_str(tok).unwrap() },
        }
    }

    fn to_type(&self) -> TokenStream {
        use TokenType::*;
        match self {
            String => quote! {String},
            UInt => quote! {usize},
            SInt => quote! {isize},
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Line {
    Seq(TokenType),
    Tuple(Vec<TokenType>),
    Array(TokenType, usize),
}

fn best_tt(tts: HashSet<TokenType>) -> TokenType {
    tts.into_iter().max().unwrap()
}

impl Line {
    fn gen(lines: &[String]) -> Line {
        let all_tokens: Vec<Vec<_>> =
            lines.iter().filter(|s| s.as_str() != "").map(|line| aoc::tokenize(line).collect()).collect();
        // Generate sequence answers
        let mut ft = TokenType::gen(all_tokens[0][0]);
        let mut tup = Vec::new();
        for tokens in &all_tokens {
            for (idx, token) in tokens.iter().enumerate() {
                let tts = TokenType::gen(token);
                ft = &ft & &tts;
                if tup.len() <= idx {
                    tup.push(tts.clone());
                }
                tup[idx] = &tup[idx] & &tts;
            }
        }
        let ftt = best_tt(ft);
        let tuptt: Vec<TokenType> = tup.into_iter().map(best_tt).collect();
        // If the lines are variable length, it needs to be a sequence
        let first_len = all_tokens[0].len();
        if all_tokens.iter().any(|tokens| tokens.len() != first_len) {
            Line::Seq(ftt)
        } else {
            // We want the tuple only if at least one of the types was more specific
            if tuptt.iter().any(|tt| tt != &ftt) {
                return Line::Tuple(tuptt);
            } else {
                return Line::Array(ftt, all_tokens[0].len());
            }
        }
    }

    fn is_simpler_than(&self, other: &Line) -> bool {
        use Line::*;
        match (self, other) {
            (&Seq(ref t), &Tuple(ref ts)) |
            (&Array(ref t, _), &Tuple(ref ts)) => ts.iter().any(|t2| t > t2),
            (&Tuple(ref ts), &Seq(ref t)) |
            (&Tuple(ref ts), &Array(ref t, _)) => ts.iter().any(|t2| t <= t2),
            (&Seq(ref t), &Seq(ref t2)) |
            (&Array(ref t, _), &Array(ref t2, _)) => t > t2,
            (&Tuple(ref ts1), &Tuple(ref ts2)) => ts1.iter().zip(ts2.iter()).any(|(t1, t2)| t1 > t2),
            (&Array(_ , _), &Seq(_)) => true,
            (&Seq(_ ), &Array(_, _)) => false
        }
    }

    fn to_parse(&self, name: &str) -> TokenStream {
        let mut name = String::from(name);
        let fn_name = format_ident!("parse_{}", name);
        capitalize(&mut name);
        let type_name = format_ident!("{}", name);
        use Line::*;
        // TODO make tok not a magic ident
        let from_tok = match *self {
            Array(ref t, 1) => {
                let single = t.to_fragment();
                quote! {
                let tok = tokens.next().unwrap();
                assert!(tokens.next().is_none());
                #single
            }}
            Seq(ref t) | Array(ref t, _) => {
                    let single = t.to_fragment();
                    quote! {
                        let mut out = Vec::new();
                        while let Some(tok) = tokens.next() {
                            out.push(#single);
                        }
                        out
                    }
            }
            Tuple(ref ts) => {
                let singles = ts.iter().map(|t| t.to_fragment());
                quote! {
                (#(#singles),*)
                }
            }
        };

        quote! {
                fn #fn_name(line: &str) -> #type_name {
                    let mut tokens = aoc::tokenize(line);
                    #from_tok
                }
        }

    }
    
    fn to_type(&self, name: &str) -> TokenStream {
        let mut name = String::from(name);
        capitalize(&mut name);
        let type_name = format_ident!("{}", name);
        use Line::*;
        match self {
            // TODO do this for tuples of size 1 too, though they shouldn't happen
            &Array(ref t, 1) => {
                let at = t.to_type();
                quote! {type #type_name = #at;}
            }
            &Array(ref t, _) | &Seq(ref t) => {
                let at = t.to_type();
                quote! {type #type_name = Vec<#at>;}
            }
            &Tuple(ref ts) => {
                let ats = ts.iter().map(|t| t.to_type());
                quote! {type #type_name = (#(#ats),*);}
            }
        }
    }

    fn cg(&self, name: &str) -> TokenStream {
        let ty = self.to_type(name);
        let p = self.to_parse(name);
        quote! {
            #ty
            #p
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Schema {
    header: Option<Line>,
    record_line: Line,
    multi_line_record: bool,
}

fn nc(s: &[String]) -> usize {
    s.iter().filter(|s| s.as_str() == "").count()
}

impl Schema {
    fn gen(lines: &[String]) -> Schema {
        let noheader = Schema {
            header: None,
            record_line: Line::gen(lines),
            multi_line_record: nc(lines) > 0,
        };
        if lines.len() == 1 {
            return noheader
        }
        let header = Schema {
            header: Some(Line::gen(&lines[0..1])),
            record_line: Line::gen(&lines[1..]),
            multi_line_record: nc(lines) > 1,
        };
 
        if header.record_line.is_simpler_than(&noheader.record_line) { header } else { noheader }
    }
}

fn main() {
    let input: Vec<String> = aoc::stdin_input().collect();
    // Use a header if it makes the *record* line type simpler. The existence of the header line
    // type is complexity, so that being simpler isn't important.
    // Use multiline records if there is at least one blank line in a non-header schema, or more
    // than one blank line in a header based schema
    let schema = Schema::gen(&input);

    let hid = Ident::new("Header", Span::call_site());
    let mhid = if schema.header.is_some() {
        Some(hid.clone()).into_iter()
    } else {
        None.into_iter()
    };
    let uses_header = schema.header.is_some();
    let hc = schema.header.map(|header_line| header_line.cg("header")).into_iter();

    let rid = Ident::new("Record", Span::call_site());
    let rc = schema.record_line.cg("record");

    let types = quote! {
        #(#hc)*
        #rc
        struct Input {
            #(header: #mhid,)*
            record: Vec<#rid>
        }
    };
    let hpo = if uses_header {
        Some(quote! {
            let header = parse_header(input.next().unwrap());
        })
    } else {
        None
    };
    let hp = hpo.into_iter();
    let rp = if schema.multi_line_record {
        quote! {
            let mut out = Vec::new();
            let mut rec = Vec::new();
            while let Some(line) = input.next() {
                if line.as_str() == "" {
                    if rec.len() != 0 {
                        out.push(rec)
                    }
                    rec = Vec::new();
                }
                rec.push(parse_record(&line));
            }
            if rec.len() != 0 {
                out.push(rec)
            }
            out
        }
    } else {
        quote! {
            let mut out = Vec::new();
            while let Some(line) = input.next() {
                if line.as_str() == "" {
                    continue
                }
                out.push(parse_record(&line));
            }
            out
        }
    };
    let mh = (if uses_header {
        Some(quote! {header,})
    } else {
        None
    }).into_iter();
    let pi = quote! {
        fn parse(mut input: impl Iterator<Item = String>) -> Input {
            #(#hp)*
            let record = { #rp };
            Input {
                #(#mh)*
                record,
            }
        }
    };
    let template = quote! {
        use std::str::FromStr;
        use std::collections::{HashMap, HashSet};
        use itertools::Itertools;
        const INPUT: &str = include_str!("../../inputs/2021/24");
        fn main() {
            println!(
                "A: {}\tB: {}",  
                solve_a(parse(aoc::str_input(INPUT))),            
                solve_b(parse(aoc::str_input(INPUT)))             
            );
        }
        fn solve_a(mut input: Input) -> isize {
            0
        }
        fn solve_b(mut input: Input) -> isize {
            0
        }
        
#[cfg(test)]                                              
mod test {                            
    use super::*;                                                           
    use aoc::str_input;                                             
                            
    const TEST_INPUT: &'static str = "\
";                              
 
    #[test]
    fn sample_a() {
        assert_eq!(solve_a(parse(str_input(TEST_INPUT))), 0)
    }
 
    #[test]
    fn sample_b() {
        assert_eq!(solve_b(parse(str_input(TEST_INPUT))), 0)
    }
}
    };

    let out = quote!{ #types #pi #template};
    println!("{}", out);
}
