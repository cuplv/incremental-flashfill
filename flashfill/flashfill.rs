/* 
  Reference: http://research.microsoft.com/en-us/um/people/sumitg/pubs/popl11-synthesis.pdf
*/

// The program takes user-defined input-output examples to synthesize a set a programs consistent with the examples.
// The input of an example can span across multiple columns in a spreadsheet.
// The output of an example is expressed in one spreadsheet column.
// The contents of any spreadsheet cell is treated as a string.

// For simplifying the implementation, I've used alternative data structures in many places.
// For example, The paper represents the input of an example as a tuple of spreadsheet columns.
// I'm using a Vec of spreadsheet columns instead. Column contents are represented as strings as mentioned before. 
type SpreadsheetColumn = String;
type ExampleInput      = Vec<SpreadsheetColumn>;

// Assuming a set of tokens to get Example 3 from the paper to work
// TODO - Figure out how to generate a comprehensive set of Tokens for a given set of input-output examples
enum SpecialToken { SlashTok }
enum Token        { SpecialToken }
type RegularExpr  = Vec<Token>;

type int = i32;
struct CPos      { k:int }
struct Pos       { r1:RegularExpr, r2:RegularExpr, c:int }
enum Position    { CPos, Pos }

// TODO - Need to define struct Loop
struct SubStr    { v:String, p1:Position, p2:Position }
struct ConstStr  { s:String }

// TODO - Redefine Concatenate to use DAGS to represent atomic expressions
enum AtomicExpr  { SubStr, ConstStr }
type Concatenate = Vec<AtomicExpr>;

struct Match     { v:String, r:RegularExpr, k:int }
struct NotMatch  { v:String, r:RegularExpr, k:int }
enum Predicate   { Match, NotMatch}

type Conjuct = Vec<Predicate>;
type Bool    = Vec<Conjuct>;

struct Switch { b_vec:Vec<Bool>, e_vec:Vec<AtomicExpr> } 
type StringExpr = Switch;

fn generate_string_program (example_inputs:Vec<ExampleInput>, example_ouputs:Vec<SpreadsheetColumn>) {}

fn generate_partition (example_inputs:Vec<ExampleInput>, example_outputs:Vec<SpreadsheetColumn>) {}

fn generate_bool_classifier (example_inputs1:Vec<ExampleInput>, example_inputs2:Vec<ExampleInput>) {}

fn generate_str (example_input:ExampleInput, example_output:SpreadsheetColumn) {}

// TODO Add parameter W
fn generate_loop (example_input:ExampleInput, example_output:SpreadsheetColumn) {}

fn generate_substring (example_input:ExampleInput, s:String) {}

fn generate_position (s:String, k:int) {}

fn generate_regex (r:RegularExpr, s:String) {}
    
/*
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

*/
fn main() {}
