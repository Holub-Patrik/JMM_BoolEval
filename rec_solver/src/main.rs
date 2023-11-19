use regex::Regex;
use std::collections::HashMap;

enum Operator {
    AND,
    OR,
    EQ,
    IMPL,
}

struct BoolTable<'a> {
    equation:  &'a str,
    curr_int: u128,
    tr_val_int: u128,
    charset: HashMap<&'a str, Operator>,
    re: Regex,
}

impl BoolTable<'_> {
    fn eval(&self, lhs:bool, rhs:bool, op:&Operator) -> bool {
        match op {
            Operator::AND => {
                return lhs && rhs;
            },
            Operator::OR => {
                return lhs || rhs;
            },
            Operator::EQ => {
                return lhs == rhs;
            },
            Operator::IMPL => {
                return lhs <= rhs;
            }
        }
    }

    fn str_eval(&self, lhs:&str, rhs:&str, op:&str) -> bool {
        let mut b_lhs:bool = false;
        let mut b_rhs:bool = false;
        
        let b_op:&Operator = self.charset.get(op).unwrap();

        if lhs.contains("(") {
            b_lhs = self.rec_eval(Some(lhs));
            if lhs.contains("!") {
                b_lhs = !b_lhs;
            }
        } else {
            if lhs.contains("!") {
                b_lhs = !self.get_val(&lhs[1 .. ])
            } else {
                b_lhs = self.get_val(&lhs)
            }
        }
        
        if rhs.contains("(") {
            b_rhs = self.rec_eval(Some(rhs));
            if rhs.contains("!") {
                b_rhs = !b_rhs;
            }
        } else {
            if rhs.contains("!") {
                b_rhs = !self.get_val(&rhs[1 .. ])
            } else {
                b_rhs = self.get_val(&rhs)
            }
        }

        return self.eval(b_lhs, b_rhs, b_op)
    }

    fn get_val(&self, indx:&str) -> bool {
        todo!()
    }

    fn rec_eval(&self, equation:Option<&str>) -> bool {
        let eq = equation.unwrap_or(self.equation);
        
        let mut answr:bool = true;
        let Some(parts) = self.re.captures(&eq) else {
            println!("invalid string!");
            return false;
        };
        answr = self.str_eval(&parts["lhs"], &parts["rhs"], &parts["op"]);
        return answr;
    }
}

fn pf<T>(_: &T) -> &'static str {
    return std::any::type_name::<T>();
}

fn main() {
    let re = Regex::new(r"\((?<lhs>.+)(?<op>&&|==|\|\||=>)(?<rhs>.+)\)").unwrap();
    let mut usr_in:String = String::new(); 
    std::io::stdin().read_line(&mut usr_in).unwrap();
    let trim_in = usr_in.trim();
    
    let Some(parts) = re.captures(&trim_in) else {todo!()}; 
    println!("lhs: {} op: {} rhs: {}", &parts["lhs"], &parts["op"], &parts["rhs"]);


    let charset = HashMap::from([
        ("&&", Operator::AND),
        ("||", Operator::OR),
        ("==", Operator::EQ),
        ("=>", Operator::IMPL),
    ]);
    
    let answer = BoolTable {
        equation: trim_in,
        curr_int: 0,
        tr_val_int: 0,
        charset,
        re,
    };
}
