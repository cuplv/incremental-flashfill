/* 
  Reference: http://research.microsoft.com/en-us/um/people/sumitg/pubs/popl11-synthesis.pdf
*/

// The program takes user-defined input-output examples to synthesize a set a programs consistent with the examples.
// The input of an example can span across multiple columns in a spreadsheet.
// The output of an example is expressed in one spreadsheet column.
// The contents of any spreadsheet cell is treated as a string.

// For simplifying the implementation, I've used alternative data structures in many places.
// For example, the paper represents the input of an example as a tuple of spreadsheet columns.
// I'm using a Vec of spreadsheet columns instead. Column contents are represented as strings as mentioned before. 
type SpreadsheetColumn = String;
type ExampleInput      = Vec<SpreadsheetColumn>;

// Assuming a set of tokens to get Example 3 from the paper to work
// TODO - Figure out how to generate a comprehensive set of Tokens for a given set of input-output examples
enum SpecialToken { SlashTok }

#[derive(Clone)]
enum Token        { SpecialToken }
type RegularExpr  = Vec<Token>;

type int = i32;
enum Position    { CPos {k:int},
                   Pos  {r1:RegularExpr, r2:RegularExpr, c:int} }

// TODO - Need to define struct Loop
// TODO - Redefine Concatenate to use DAGS to represent atomic expressions
enum AtomicExpr  { SubStr   {v:String, p1:Position, p2:Position},
                   ConstStr {s:String} }
type Concatenate = Vec<AtomicExpr>;
type DAG         = Concatenate;

struct Match     { v:String, r:RegularExpr, k:int }
struct NotMatch  { v:String, r:RegularExpr, k:int }
enum Predicate   { Match    {v:String, r:RegularExpr, k:int},
                   NotMatch {v:String, r:RegularExpr, k:int} }

type Conjuct = Vec<Predicate>;
type Bool    = Vec<Conjuct>;

struct Switch { bool_vec:Vec<Bool>, e_vec:Vec<DAG> } 
type StringExpr = Switch;

type ExampleInputSet = Vec<ExampleInput>;
struct Traces { b_vec:Vec<ExampleInputSet>, e_vec:Vec<DAG> }

fn util_vec_diff<T> (a: Vec<T>, b:Vec<T>) -> Vec<T> {
    panic!();
}

fn util_bool_expr_sort (b_vec:Vec<ExampleInputSet>, e_vec:Vec<DAG>) -> Traces {
    panic!();
}

fn util_compatibility (e_vec1:Vec<DAG>, e_vec2:Vec<DAG>) -> bool {
    panic!();
}

fn iParts (tok:Token, s:String) -> Token {
    panic!();
}
    
fn generate_string_program (example_inputs:Vec<ExampleInput>, example_outputs:Vec<SpreadsheetColumn>) -> StringExpr {
    let mut b_vec_init:  Vec<ExampleInputSet> = Vec::new();
    let mut e_vec_init:  Vec<DAG>             = Vec::new();
    let mut traces:      Traces               = Traces { b_vec:b_vec_init, e_vec:e_vec_init };
    
    for i in 0..example_inputs.len() {
        traces.b_vec.push(vec![example_inputs[i].clone()]);
        let dag = generate_str(example_inputs[i].clone(), example_outputs[i].clone());
        traces.e_vec.push(dag);
    }
    traces = generate_partition(example_inputs, example_outputs);

    let mut bool_classifiers: Vec<Bool> = Vec::new();
    for i in 0..traces.b_vec.len() {
        bool_classifiers.push(generate_bool_classifier(traces.b_vec[i].clone(), traces.b_vec[i].clone()));
    }
    traces = util_bool_expr_sort(traces.b_vec, traces.e_vec);

    let switch: Switch = Switch { bool_vec:bool_classifiers, e_vec:traces.e_vec };
    switch
}

fn generate_partition (example_inputs:Vec<ExampleInput>, example_outputs:Vec<SpreadsheetColumn>) -> Traces {
    panic!();
}

fn generate_bool_classifier (example_inputs1:Vec<ExampleInput>, example_inputs2:Vec<ExampleInput>) -> Bool {
    panic!();
}

fn generate_str (example_input:ExampleInput, example_output:SpreadsheetColumn) -> DAG {
    panic!();
}

// TODO Add parameter W
fn generate_loop (example_input:ExampleInput, example_output:SpreadsheetColumn) {
    panic!();
}

fn generate_substring (example_input:ExampleInput, s:String) {
    panic!();
}

fn generate_position (s:String, k:int) {
    let mut result: Vec<Position> = Vec::new();
    let cpos_init_1: Position = Position::CPos { k:k };
    let cpos_init_2: Position = Position::CPos { k:(k - (s.len() as int)) };
    result.push(cpos_init_1);
    result.push(cpos_init_2);
    panic!();
}

fn generate_regex (r:RegularExpr, s:String) -> RegularExpr {
    let mut regex: Vec<Token> = Vec::new();
    for i in 0..r.len() {
        regex.push(iParts(r[i].clone(), s.clone()));
    }
    regex
}
    
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
