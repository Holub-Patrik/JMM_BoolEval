use std::fmt;
use std::collections::HashMap;
use regex::Regex;


pub enum Operator {
    AND,
    OR,
    EQ,
    IMPL,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operator::AND => write!(f, "&&"),
            Operator::OR => write!(f, "||"),
            Operator::EQ => write!(f, "=="),
            Operator::IMPL => write!(f, "=>"),
        }
    }
    
}

pub struct BoolTable<'a> {
    verbose: bool,
    equation:  &'a str,
    curr_int: u128,
    tr_val_int: u128,
    charset: HashMap<&'a str, Operator>,
    var_to_indx: HashMap<&'a str, usize>,
    re: Regex,
    pub result_table: Vec<Vec<bool>>,
}

pub fn new<'a>(verbose:bool, equation:&'a str, tr_val_int:u128, 
               charset:HashMap<&'a str, Operator>, 
               var_to_indx:HashMap<&'a str, usize>, re:Regex) -> BoolTable<'a> {
    BoolTable { 
        verbose,
        equation, 
        curr_int: 0, 
        tr_val_int, 
        charset, 
        var_to_indx, 
        re, 
        result_table: vec![]
    }
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

    fn str_eval(&self, lhs:&str, rhs:&str, op:&str) -> Result<bool, String> {
        let mut b_lhs:bool;
        let mut b_rhs:bool;
         
        if self.verbose {println!("str_eval(): Matching operator...");}
        let b_op:&Operator = match self.charset.get(op) {
            Some(op) => op,
            None => return Err("Operator not in charset!".to_string()),
        };
        if self.verbose {println!("str_eval(): Matched operator: {}", b_op);}

        if lhs.contains("(") {
            if self.verbose {println!("str_eval(): lhs impossible to evaluate, sending for breakdown");}
            b_lhs = match self.rec_eval(Some(lhs)) {
                Ok(val) => val,
                Err(err) => return Err(err),
            };
            if lhs.contains("!") {
                b_lhs = !b_lhs;
            }
        } else {
            if lhs.contains("!") {
                if self.verbose {println!("str_eval(not_lhs): Getting not value from {}...", lhs);}
                b_lhs = !self.get_val(&lhs[1 .. ]);
                if self.verbose {println!("str_eval(not_lhs): Got: {}", b_lhs);}
            } else {
                if self.verbose {println!("str_eval(normal_lhs): Getting value from {}...", lhs);}
                b_lhs = self.get_val(&lhs);
                if self.verbose {println!("str_eval(normal_lhs): Got: {}", b_lhs);}
            }
        }
        
        if rhs.contains("(") {
            if self.verbose {println!("str_eval(): rhs impossible to evaluate, sending for breakdown");}
            b_rhs = match self.rec_eval(Some(rhs)) {
                Ok(val) => val,
                Err(err) => return Err(err),
            };
            if rhs.contains("!") {
                b_rhs = !b_rhs;
            }
        } else {
            if rhs.contains("!") {
                if self.verbose {println!("str_eval(not_rhs): Getting not value from {}...", rhs);}
                b_rhs = !self.get_val(&rhs[1 .. ]);
                if self.verbose {println!("str_eval(not_rhs): Got: {}", b_rhs);}
            } else {
                if self.verbose {println!("str_eval(normal_rhs): Getting value from {}...", rhs);}
                b_rhs = self.get_val(&rhs);
                if self.verbose {println!("str_eval(normal_rhs): Got: {}", b_rhs);}
            }
        }
        
        if self.verbose {println!("str_eval(): Evaluating...");}
        let result:bool = self.eval(b_lhs, b_rhs, b_op);
        if self.verbose {println!("str_eval(): Result: {}", result);}
        return Ok(result);
    }

    fn get_val(&self, var:&str) -> bool {
        let indx = self.var_to_indx.get(var).unwrap();
        if self.verbose {println!("get_val(): Value: {} has the index: {} and I see binary: {:#010b}", var, indx, self.curr_int+self.tr_val_int);}
        let val = (self.curr_int+self.tr_val_int)>>indx &1 == 1;
        if self.verbose {println!("get_val(): Resulting value: {}", val);}
        return val;
    }

    pub fn rec_eval(&self, equation:Option<&str>) -> Result<bool, String> {
        let eq = equation.unwrap_or(self.equation);
        if self.verbose {println!("rec_eval(): Evaluation: {} With int repr: {:#010b}", eq, self.curr_int);}
        
        let Some(parts) = self.re.captures(&eq) else {
            if self.verbose {println!("invalid string!");}
            return Err("Invalid string!".to_string());
        };
        if self.verbose {println!("rec_eval(): evaluting:{} {} {}",&parts["lhs"], &parts["rhs"], &parts["op"]);}
        let answr = self.str_eval(&parts["lhs"], &parts["rhs"], &parts["op"]);
        return answr;
    }

    pub fn create_table(&mut self) -> Result<(), String> {
        let complete_add_value:u128 = !self.tr_val_int;
        let size = self.var_to_indx.len();

        for _ in 0 .. 2_u128.pow(size.try_into().unwrap()) {
            let mut line:Vec<bool> = vec![];
            if complete_add_value | self.curr_int > complete_add_value {
                self.curr_int += 1;
                continue;
            }
            let actual_val = (complete_add_value & self.curr_int) + self.tr_val_int;
            for i in 0 .. size {
                let curr_val:bool = (actual_val>>i &1)==1;
                line.push(curr_val);
            }
            if self.verbose {println!("----------Calling for recursive evaluation!----------");}
            let result = match self.rec_eval(None){
                Ok(val) => val,
                Err(err) => return Err(err),
            };
            
            if self.equation.chars().nth(0).unwrap() == '!'{
                line.push(!result)
            } else {
                line.push(result)
            }
            self.result_table.push(line);
            self.curr_int += 1;
            if self.verbose {println!("----------Recursive Evaluation done----------\n");}
        }
        Ok(())
    }
}
