enum Operator {
    NOT,
    AND,
    OR,
    EQ,
    IMPL,
}

struct BoolTable {
    equation: String,
    curr_int: u128,
    tr_val_int: u128,
    bool_char: Vec<(String, Operator)>,
}

impl BoolTable {
    fn eval(&self, lhs:bool, rhs:bool, op:Operator) -> bool {
        match op {
            Operator::NOT => {
                return !rhs;
            },
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

    fn str_eval(&self, lhs:&str, rhs:&str, op:&str) {
        todo!();
    }

    fn get_val(&self, indx:i32) {
        todo!()
    }

    fn rec_eval(&self, equation:String) {
    }
}

fn main() {
}
