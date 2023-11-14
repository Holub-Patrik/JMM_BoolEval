use crate::{Compile, Node, Operator, Result};
pub struct Interpreter;

impl Compile for Interpreter {
    type Output = Result<bool>;

    fn from_ast(ast: Vec<Node>) -> Self::Output {
        let mut ret = true;
        let evaluator = Eval::new();
        for node in ast {
            ret = ret && evaluator.eval(&node);
        }
        Ok(ret)
    }
}
// ANCHOR_END: interpreter

// ANCHOR: interpreter_recursive
struct Eval;

impl Eval {
    pub fn new() -> Self {
        Self
    }
    // ANCHOR: interpreter_eval
    pub fn eval(&self, node: &Node) -> bool{
        match node {
            Node::UnaryExpr { child } => {
                let child = self.eval(child);
                child != true
            }
            Node::BinaryExpr { op, lhs, rhs } => {
                let lhs_ret = self.eval(lhs);
                let rhs_ret = self.eval(rhs);

                match op {
                    Operator::And => lhs_ret && rhs_ret,
                    Operator::Or => lhs_ret || rhs_ret,
                    Operator::Imp => lhs_ret <= rhs_ret,
                    Operator::Eq => lhs_ret == rhs_ret,
                }
            }
        }
    }
    // ANCHOR_END: interpreter_eval
}
