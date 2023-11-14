use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Operator {
    And,
    Or,
    Imp,
    Eq,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match &self {
            Operator::And => write!(f, "&"),
            Operator::Or => write!(f, "|"),
            Operator::Imp => write!(f, "=>"),
            Operator::Eq => write!(f, "<=>"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
    UnaryExpr{
        child: Box<Node>,
    },
    BinaryExpr {
        op: Operator,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
}


impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match &self {
            Node::UnaryExpr { child } => write!(f, "!{}", child),
            Node::BinaryExpr { op, lhs, rhs } => write!(f, "{} {} {}", lhs, op, rhs), 
        }
    }
}
