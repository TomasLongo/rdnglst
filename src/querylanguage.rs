use std::result;

fn main() {
    let columns = [String::from("uno"), String::from("dos")].to_vec();
    let mods = [String::from("is"), String::from("and")].to_vec();
    let columnValue = [String::from("tomas"), String::from("maus")].to_vec();

    let modifier = parse_query("uno is tomas and tomas", &columns, &mods);

    println!("{}", eval(&modifier, &columns, &columnValue));
}

fn eval(modifier: &Modifier, cols: &Vec<String>, col_values: &Vec<String>) -> bool {
    match &modifier.right {
        Some(x) => {
            match modifier.t {
                ModType::And => return perform_comparison(&modifier.left, cols, col_values) && perform_comparison(&x, cols, col_values),
                ModType::Or => {
                    println!("Modifier or not covered yet");
                    return false;
                }
            }
        },
        None => return perform_comparison(&modifier.left, cols, col_values)
    }
}

fn perform_comparison(comp: &Comparison, cols: &Vec<String>, colValues: &Vec<String>) -> bool {
    match &comp.t {
        Equal => {
            println!("Performin equal comparison");
            let mut colValue: &String = &String::from("");
            if (comp.col.name == "uno") {
                colValue = &colValues[0];
            } else if (comp.col.name == "dos") {
                colValue = &colValues[1];
            }

            println!("comparing {} with {}", comp.ident.name, *colValue);
            return comp.ident.name == *colValue;
        }
    }
}

fn is_col(cols: &Vec<String>, token: &String) -> bool {
    return cols.iter().any(|x| x == token);
}

fn is_mod(mods: &Vec<String>, token: &String) -> bool {
    return mods.iter().any(|x| x == token);
}

// Gibt den root modifier zurueck
fn parse_query(q: &str, columns: &Vec<String>, mods: &Vec<String>) -> Modifier {
    let mut comp: Comparison = Comparison::default();
    let mut root: Modifier = Modifier::default();

    let mut token = String::new();

    // start state
    // states: col compType ident mod
    let mut expectedToken = "col";
    let mut currentModifier: &mut Modifier = &mut root;
    let mut current_comp: &mut Comparison = &mut currentModifier.left;

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
                

                if is_mod(mods, &token) == false {
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

struct Modifier {
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

#[derive(Debug)]
struct Column {
    name: String
}

