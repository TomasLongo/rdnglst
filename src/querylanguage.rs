use std::result;
use std::collections::HashMap;

// Nachdem man die Query geparst hat, kann man mit dem gelieferten Modifier die 
// einzelnen Zeilen einer Tabelle abgrasse;
pub fn eval(modifier: &Modifier, row: &mut TableRow) -> bool {
    match &modifier.right {
        Some(x) => {
            match modifier.t {
                ModType::And => return perform_comparison(&modifier.left, row) && perform_comparison(&x, row),
                ModType::Or => {
                    println!("Modifier or not covered yet");
                    return false;
                }
            }
        },
        None => return perform_comparison(&modifier.left, row)
    }
}

fn perform_comparison(comp: &Comparison, row: &mut TableRow) -> bool {
    match &comp.t {
        Equal => {
            println!("Performin equal comparison");
            let col_value = row.get(&comp.col.name);

            println!("comparing {} with {}", &comp.ident.name, &col_value);

            return comp.ident.name == col_value;
        }
    }
}

#[test]
fn equal_comparison_works() {
    let comp = Comparison {
        t: CompType:: Equal,
        col: Column {name: String::from("foo")},
        ident: Ident { name: String::from("bar") }
    };

    let mut row = TableRow::new();
    row.insert(&String::from("foo"), &String::from("bar"));

    assert!(perform_comparison(&comp, &mut row), "Comparison should be true");


    row.insert(&String::from("foo"), &String::from("hola"));
    assert!(!perform_comparison(&comp, &mut row), "Comparison should be false");
}

// Abstraction over a table row holding columns and associated values
pub struct TableRow {
    values: HashMap<String, String>
}

impl TableRow {
    pub fn new() -> TableRow {
        return TableRow { values: HashMap::new() }
    }

    pub fn get(&mut self, column: &String) -> String {
        match self.values.get(column) {
            Some(x) => return x.clone(),
            None => panic!("Could not find column {} in row", column)
        }
    }

    pub fn insert(&mut self, column: &String, value: &String) {
        self.values.insert(column.clone(), value.clone());
    }
}

pub struct Table {
    rows: Vec<TableRow>
}

fn is_col(cols: &Vec<String>, token: &String) -> bool {
    return cols.iter().any(|x| x == token);
}

fn is_mod(mods: &Vec<String>, token: &String) -> bool {
    return mods.iter().any(|x| x == token);
}

#[derive(PartialEq, Debug)]
enum State {
    Start,
    Col,
    Value,
    CompOp,
    MultiWordValue,
    Operator
}

#[test]
fn test_query_parsing_single_value() {
    let cols = vec![String::from("eins")];

    let modi = parse_query("eins is zwei", &cols);
    assert_eq!(modi.t, ModType::And);
    assert_eq!(modi.left.col.name, String::from("eins"));
    assert_eq!(modi.left.ident.name, String::from("zwei"));
    assert!(modi.right.is_none());

}

#[test]
fn test_query_parsing_multivalue() {
    let cols = vec![String::from("eins")];

    let modi = parse_query("eins is \"zwei drei\"", &cols);
    assert_eq!(modi.t, ModType::And);
    assert_eq!(modi.left.col.name, String::from("eins"));
    assert_eq!(modi.left.ident.name, String::from("zwei drei"));
    assert_eq!(modi.left.t, CompType::Equal);
    assert!(modi.right.is_none());
}

/// Parses a query string building the syntax tree. Performs
/// checks against valid columns by using the passed vec of 
/// column names.
///
/// Returns the root node of the tree which can then be used
/// to evaluate the query against real data
pub fn parse_query(q: &str, columns: &Vec<String>) -> Modifier {
    let mut comp: Comparison = Comparison::default();
    let mut root: Modifier = Modifier::default();

    let mut token = String::new();

    let mut current_state = State::Start;

    let mut currentModifier: &mut Modifier = &mut root;
    let mut current_comp: &mut Comparison = &mut currentModifier.left;

    let mods: Vec<String> = vec!["is".to_string(), "has".to_string()];

    for c in q.chars() {
        // end of word
        //
        if c != ' ' {
            if c == '"' {
                if current_state == State::Value {
                    // start of multiword value
                    eprintln!("Start of multiwordvalue");
                    current_state = State::MultiWordValue;
                } 
                else {
                    // end of multiwordvalue
                    // is this the last char in the query?
                    // If so, should we be fine b
                    //
                    // if not, we have to set the new state
                    eprintln!("End of multiwordvalue");
                    current_state = State::MultiWordValue;
                }
            } else {
                token.push(c);
            }
        } 
        else {
            //  we encountered a space which acts as a separator

            if current_state == State::MultiWordValue  {
                eprintln!("Detected multiword value {}", token);
                //  add the space if we are dealing with a multiword value
                token.push(c);
            } 
            else if current_state == State::Start {
                // We expect a column name at the very beginning
                if !is_col(&columns, &token) {
                    eprintln!("Expected a col name. Got {}", token);
                    panic!(String::from("Expected a col ident"));
                }

                eprintln!("Found column {}", token);
                current_comp.col = Column {name: token};

                // Comparison Operator should be next
                current_state = State::CompOp;

                token = String::new();

            } else if current_state == State::CompOp {
                if token == "is" {
                    current_comp.t = CompType::Equal;
                    eprintln!("Found comp type equal");
                    current_state = State::Value;
                } else {
                    panic!("Expected a comparison operator ('is' or 'has') but found {}", token);
                }
                token = String::new();
            } else if current_state == State::Value {
                eprintln!("Found value '{}'", token);
                current_comp.ident = Ident {name: token};
                current_state = State::Operator;
                token = String::new();
            } else if current_state == State::Operator {
                // an operator creates new comparison
                

                if is_mod(&mods, &token) == false {
                    panic!("expected an operator ('and' or 'or'), Found {}", &token);
                }

                let new_comp = Comparison {
                    t: comp.t, 
                    col: Column { name: current_comp.col.name.clone() }, 
                    ident: Ident{ name: String::from("") },
                };

                currentModifier.right = Some(new_comp);
                match currentModifier.right {
                    Some(ref mut x) => current_comp = x,
                    None => panic!("Could not get right hand side of modifier")
                }
                eprintln!("Found additional comparison: {:?}", current_comp);
                current_state = State::Value;
                token = String::new();
            }

            continue;
        } 

    }
    if current_state != State::Value && current_state != State::MultiWordValue {
        eprintln!("We are at the end of the query and expecting a value. But we are currently in {:?}", current_state);
        panic!(String::from("Wrong state"));
    }

    println!("Found final ident {}", token);
    current_comp.ident = Ident {name: token};

    return root;
}


#[derive(Debug, Copy, Clone, PartialEq)]
enum CompType {
    Equal,
    Contains,
    No
}

#[derive(Debug, PartialEq)]
enum ModType {
    And,
    Or
}

pub struct Modifier {
    left: Comparison,

    // Comparison gehoert nicht dem Modifier, sondern dem Option
    // Kann ich mir den Wert also vom Option leihen???
    right: Option<Comparison>,
    t: ModType
}

impl Default for Modifier {
    fn default() -> Modifier {
        return Modifier {
            left: Comparison::default(),
            right: None,
            t: ModType::And
        }
    }
}

impl Modifier {
    fn new(left: Comparison, right: Option<Comparison>, t: ModType) -> Modifier {
        return Modifier {left: left, right: right, t: t}
    }
}

/// Defines the comparison of a value with the content of a column
#[derive(Debug)]
struct Comparison {
    ident: Ident,
    col: Column,
    t: CompType
}

impl Default for Comparison {
    fn default() -> Comparison {
        return Comparison {
            t: CompType::No,
            col: Column { name: String::from("") },
            ident: Ident { name: String::from("") }
        }
    }
}

#[derive(Debug)]
struct Ident {
    name: String
}

/// Represents a named column in a table
#[derive(Debug)]
struct Column {
    name: String
}
