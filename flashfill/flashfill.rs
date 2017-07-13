type State = Vec<String>;
type StateSet = Vec<State>;

struct E {}

struct T {
  states : StateSet,
  exp : E,
}

struct ConstStr {
  s : String,
}

struct SubStr {
  v : String,
  p1 : usize,
  p2 : usize, 
}

struct CPos {
  k : usize,
}

enum Token { StartTok, EndTok, AlphaTok, SlashTok }

enum f { ConstStr, SubStr }

fn regex_of_str(s:String) -> Vec<Token>{
  let mut res: Vec<Token> = Vec::new();
  res.push(Token::StartTok);
  for c in s.chars() {
    match c {
      '\\' => res.push(Token::SlashTok),
      _ => res.push(Token::AlphaTok),
    }
  }
  res.push(Token::EndTok);
  res
}

fn generate_regex(r:Vec<Token>, s:String) -> Vec<Token> {r}

fn generate_position(s:String, k:usize){
  let mut result: Vec<CPos> = Vec::new();
  result.push(CPos{k:k});
  result.push(CPos{k:k-s.len()});
}

fn generate_substring(input_state:State, s:String) {
}

fn generate_str(input_state:State, output_string:String) -> E{
  let eta: Vec<usize> = (0..output_string.len()).collect();
  let eta_s: Vec<usize> = vec![0];
  let eta_t: Vec<usize> = vec![output_string.len()];
  let mut xi: Vec<(usize, usize)> = Vec::new();
  for j in 1..output_string.len() {
    for i in 0..j {
      xi.push((i,j));      
    }
  }
  let mut w: Vec<f> = Vec::new();
  for i in 1..xi.len() {
    let edge = xi[i];
    let ss = &output_string[edge.0..edge.0 + edge.1 - 1];
    let cs = ConstStr {s:ss.to_string()};
  }
  panic!()
}

fn generate_string_program(input_states:Vec<State>, output_strings:Vec<String>) {
  let mut t : Vec<T> = Vec::new();
  for i in 0..input_states.len() {
    let generated_strs = generate_str(input_states[i].clone(), output_strings[i].clone());
    let tt : T = T {states: vec![input_states[i].clone()], exp: generated_strs};
    t.push(tt);
  }
}
fn main() {}
