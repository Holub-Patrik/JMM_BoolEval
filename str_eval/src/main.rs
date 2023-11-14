/* 
 * [lhs, rhs, ret, lsh, rhs, ret, .. ]
 * [op, op, op, op, op .. ]
 * lsh = num_of bitshifts to get the value
 * lhs, rhs, ret = cluster
 */ 

use regex::Regex;


pub enum Operators {
    NOT,
    AND,
    OR,
    IMPL,
    EQ,
}

impl Operators {
    fn eval(&self, lhs:bool, rhs:bool) -> bool {
        match self {
            Self::AND => lhs && rhs,
            Self::OR => lhs || rhs,
            Self::EQ => lhs == rhs,
            Self::IMPL => lhs <= rhs,
            Self::NOT => !rhs
        }
    }
}

fn bob_builder() {
    
}

fn main() {
    let var_re:Regex = Regex::new(r"([0-9A-Za-z]+)").unwrap();

    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    
    println!("Prep begins!");

    let trim_input = input.trim();
    println!("{}", trim_input);

    let mut val_names:Vec<&str> = vec![];
    for (_,[var]) in var_re.captures_iter(&input).map(|c| c.extract()) {
        val_names.push(var)
    }
    let val_name_slice = &val_names;
    
    let mut indx_pairs:Vec<(usize,usize)> = vec![];
    let mut indx_start:Vec<usize> = vec![];

    let mut i:usize = 0;
    for char in trim_input.chars(){
        match char {
            '(' => indx_start.push(i),
            ')' => indx_pairs.push((indx_start.pop().unwrap(),i)),
            _ => {},
            
        }
        i += 1;
    }

    for (start, end) in &indx_pairs {
        println!("{} : {}", start, end)
    }

    println!("Prep ends! Took:")
}
