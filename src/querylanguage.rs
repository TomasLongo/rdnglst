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

    // start state
    let mut expectedToken = "col";
    let mut currentModifier: &mut Modifier = &mut root;
    let mut current_comp: &mut Comparison = &mut currentModifier.left;

    let mods: Vec<String> = vec!["is".to_string(), "has".to_string()];

    for c in q.chars() {
        if c == ' ' {
            if expectedToken == "col" {
                if !is_col(&columns, &token) {
                    panic!(String::from("Expected a col ident"));
                }

                println!("Found column {}", token);
                current_comp.col = Column {name: token};
                expectedToken = "compType"

            } else if expectedToken == "compType" {
                if token == "is" {
                    current_comp.t = CompType::Equal;
                    println!("Found comp type equal");
                    expectedToken = "ident"
                } else {
                    panic!(String::from("Expected a comp type ident"));
                }
            } else if expectedToken == "ident" {
                println!("Found iden {}", token);
                current_comp.ident = Ident {name: token};
                expectedToken = "mod"
            } else if expectedToken == "mod" {
                // mod creates new comparison
                

                if is_mod(&mods, &token) == false {
                    panic!("expected modifier");
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
                println!("Found new comparison: {:?}", current_comp);
                expectedToken = "ident";
            }

            token = String::new();
            continue;
        } 

        token.push(c);
    }
    if expectedToken != "ident" {
        panic!(String::from("Wrong state"));
    }

    println!("Found final ident {}", token);
    current_comp.ident = Ident {name: token};

    return root;
}


#[derive(Debug, Copy, Clone)]
enum CompType {
    Equal,
    Contains,
    No
}

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
