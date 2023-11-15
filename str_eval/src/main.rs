/* 
 * [lhs, rhs, ret, lsh, rhs, ret, .. ]
 * [op, op, op, op, op .. ]
 * lsh = num_of bitshifts to get the value
 * lhs, rhs, ret = cluster
 * (  ( ( ) ( ) ) (  (  )  )  )
 * 1  2 3 4 5 6 7 8  9  10 11 12
 * 12 7 4 3 6 4 2 11 10 9  8  1
 */ 

use regex::Regex;


pub enum Operators {
    NOT,
    AND,
    OR,
    IMPL,
    EQ,
    NULL,
}

impl Operators {
    fn eval(&self, lhs:bool, rhs:bool) -> bool {
        match self {
            Self::AND => lhs && rhs,
            Self::OR => lhs || rhs,
            Self::EQ => lhs == rhs,
            Self::IMPL => lhs <= rhs,
            Self::NOT => !rhs,
            Self::NULL => unreachable!(),
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
    
    let mut indx_pairs:Vec<(usize,usize)> = vec![(0,0);trim_input.len()];
    let mut indx_start:Vec<usize> = vec![];

    let mut i:usize = 0;
    for char in trim_input.chars(){
        match char {
            '(' => indx_start.push(i),
            ')' => {
                let pop_val = indx_start.pop().unwrap();
                indx_pairs[pop_val] = (pop_val,i);
            },
            _ => {},
            
        }
        i += 1;
    }
    //sort the indx_pairs
    for(start, end) in indx_pairs.clone() {
        indx_pairs.insert(end, (end,start));
    }
    indx_pairs.retain(|&x| x != (0,0));
    //small fix
    for (start, end) in &indx_pairs {
        println!("{} : {}", start, end)
    }
    
    let mut clusters:[(usize,usize, bool);64] = [(0,0,false);64]; 
    let mut ops:[&str;64] = ["";64];
    
    let mut cluster_indx = 0;
    for i in 0 .. indx_pairs.len()-1 {
        let (_, end) = indx_pairs[i];
        let (start, _) = indx_pairs[i+1];
        if end == start {
            clusters[cluster_indx] = (trim_input.chars().nth(start+1).unwrap(),trim_input.chars().nth(end-1).unwrap(),false);
            ops[cluster_indx] = (trim_input.chars().nth(start+2).unwrap())
        }
            
    }

    println!("Prep ends! Took:")
}
