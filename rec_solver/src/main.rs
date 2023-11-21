use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
mod bool_table;
use bool_table::Operator;
use std::env;


use std::io::Write;

fn _bool_table(size:u32, true_val_repr:u128) -> () {
    let stdout = std::io::stdout();
    let lock = stdout.lock();
    let mut w = std::io::BufWriter::new(lock);

    let mut expr_val:u128 = 0;
    let complete_add_value:u128 = !true_val_repr;

    for _ in 0 .. 2_u128.pow(size.try_into().unwrap()) {
        if complete_add_value | expr_val > complete_add_value {
            expr_val+=1;
            continue;
        }
        let actual_val = (complete_add_value & expr_val)+true_val_repr;
        //println!("actual val: {} and expr_val = {}",actual_val, expr_val);
        for i in 0 .. size {
            let curr:u128 = actual_val >> i&1;
            write!(&mut w, "{} ", curr.to_string()).unwrap();
        }
        writeln!(&mut w, "").unwrap();
        expr_val+=1;
        }
    println!("");
}

fn _pf<T>(_: &T) -> &'static str {
    return std::any::type_name::<T>();
}

fn pretty_print(table:Vec<Vec<bool>>, vars:Vec<&str>, equ:&str) {
    let mut max_width = 1;
    let eq_width = equ.len();
    
    for var in &vars {
        if var.len() > max_width { max_width = var.len() }
    }
    let total_len = vars.len()*3+vars.len()*max_width+eq_width+4;
    println!("{:-^1$}", "Input", total_len);
    let mut line = String::from("| ");
    for var in &vars {
        line.push_str(&format!("{:^1$} | ", var, max_width)); 
    }
    println!("{}{} |", line, equ);
    println!("{:-^1$}", "Boolean Table", total_len);
    for line in table {
        let mut write_line = String::from("| ");
        let mut peek_line = line.iter().peekable();
        while let Some(val) = peek_line.next() {
            let write_val = match val {
                true => "T",
                false => "F",
            };
            if peek_line.peek().is_none() {
                write_line.push_str(&format!("{:^1$} | ", write_val, eq_width));
            } else {
                write_line.push_str(&format!("{:^1$} | ", write_val, max_width));
            }
        }
        println!("{}", write_line)
    }
    println!("{:-<1$}", "", total_len);

}

fn main() {
    use std::time::Instant;
    let args: Vec<String> = env::args().collect();
    let verbose = args.contains(&String::from("-v"));
    let performace = args.contains(&String::from("-p"));
    lazy_static! {
        static ref RE:Regex = Regex::new(r"\((?<lhs>!?[a-zA-Z0-9_-]+|!?\(.+\))(?<op>&&|==|\|\||=>)(?<rhs>!?[a-zA-Z0-9_-]+|!?\(.+\))\)").unwrap();
        static ref VAR_RE:Regex = Regex::new(r"([a-zA-Z0-9_-]+)").unwrap();
    }
    let mut prep:Instant = Instant::now();
    let mut eval:Instant = Instant::now();
    println!("Please enter a valid boolean expression ");
    loop {
        print!(">> ");
        std::io::stdout().flush().unwrap();
        let mut usr_in:String = String::new(); 
        std::io::stdin().read_line(&mut usr_in).unwrap();
        
        if performace {
            println!("Preperation begins!");
            prep = Instant::now();
        }
        //initialize the charset, eventually can be defined from file
        let charset = HashMap::from([
            ("&&", Operator::AND),
            ("||", Operator::OR),
            ("==", Operator::EQ),
            ("=>", Operator::IMPL),
        ]);
        
        //take user input
        let trim_in = usr_in.trim();
        
        //get the equation and set vars
        let (equ, tr_vars) = match trim_in.split_once(",") {
            None => (trim_in, ""),
            Some(tup) => tup,
        };

        if verbose {
            println!("Found equation: {}", equ);
            if tr_vars != "" {
                println!("Found set true vals: {}", tr_vars);
            }
            let Some(parts) = RE.captures(&equ) else {todo!()}; 
            println!("lhs: {} op: {} rhs: {}", &parts["lhs"], &parts["op"], &parts["rhs"]);
        }

        let mut var_to_indx:HashMap<&str, usize> = HashMap::new();
        let mut i:usize = 0;
        let mut vars:Vec<&str> = vec![];
        for (_,[var]) in VAR_RE.captures_iter(&equ).map(|c| c.extract()) {
            //println!("Found var: {} and giving index: {}", var, i);
            match var_to_indx.entry(var) {
                std::collections::hash_map::Entry::Vacant(_) => {
                    vars.push(var);
                    var_to_indx.insert(var, i);
                    i += 1;
                },
                std::collections::hash_map::Entry::Occupied(_) => {},
            }
        }   

        if verbose {println!("Var to index: {:?}", var_to_indx);}

        let mut tr_val_int:u128 = 0;
        for (_,[var]) in VAR_RE.captures_iter(&tr_vars).map(|c| c.extract()) {
            let pow_val = match var_to_indx.get(var) {
                Some(v) => v,
                None => {println!("Incorect variable name when defining always true variables.\n Problematic name: {}", var);return},
            };
            tr_val_int += 2_u128.pow(*pow_val as u32)
        }
    
        if verbose {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
        }
        
        let mut table_struct = bool_table::new(verbose, equ, tr_val_int, charset, var_to_indx, RE.to_owned());
        if performace {
            let prep_took = prep.elapsed();
            println!("Preparations took: {:?}", prep_took);
            println!("Evaluation begins!");
            eval = Instant::now();
        }
        let build = table_struct.create_table();
        match build {
            Ok(_) => {
                if performace {
                    let eval_took = eval.elapsed();
                    println!("Evaluation took: {:?}", eval_took);
                }
                if verbose {println!("Hopefully created the table!");}
                pretty_print(table_struct.result_table, vars, equ)
            },
            Err(msg) => println!("{}", msg),
            
        }
    }
}
