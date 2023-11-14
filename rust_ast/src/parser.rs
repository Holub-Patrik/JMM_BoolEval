#![allow(clippy::upper_case_acronyms, clippy::result_large_err)]

use pest::{self, Parser};

use crate::ast::{Node, Operator};

// ANCHOR: parser
#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
struct BoolEval;
// ANCHOR_END: parser

// ANCHOR: parse_source
pub fn parse(source: &str) -> std::result::Result<Vec<Node>, pest::error::Error<Rule>> {
    let mut ast = vec![];
    let pairs = BoolEval::parse(Rule::Program, source)?;
    for pair in pairs {
        if let Rule::Expr = pair.as_rule() {
            ast.push(build_ast_from_expr(pair));
        }
    }
    Ok(ast)
}
// ANCHOR_END: parse_source

fn build_ast_from_expr(pair: pest::iterators::Pair<Rule>) -> Node {
    match pair.as_rule() {
        Rule::Expr => build_ast_from_expr(pair.into_inner().next().unwrap()),
        Rule::UnaryExpr => {
            let mut pair = pair.into_inner();
            let child = pair.next().unwrap();
            let child = build_ast_from_term(child);
            parse_unary_expr(child)
        }
        Rule::BinaryExpr => {
            let mut pair = pair.into_inner();
            let lhspair = pair.next().unwrap();
            let mut lhs = build_ast_from_term(lhspair);
            let op = pair.next().unwrap();
            let rhspair = pair.next().unwrap();
            let mut rhs = build_ast_from_term(rhspair);
            let mut retval = parse_binary_expr(op, lhs, rhs);
            loop {
                let pair_buf = pair.next();
                if let Some(op) = pair_buf {
                    lhs = retval;
                    rhs = build_ast_from_term(pair.next().unwrap());
                    retval = parse_binary_expr(op, lhs, rhs);
                } else {
                    return retval;
                }
            }
        }
        unknown => panic!("Unknown expr: {:?}", unknown),
    }
}

fn build_ast_from_term(pair: pest::iterators::Pair<Rule>) -> Node {
    match pair.as_rule() {
        Rule::Expr => build_ast_from_expr(pair),
        Rule::Term => build_ast_from_term(pair),
        unknown => panic!("Unknown term: {:?}", unknown),
    }
}

fn parse_unary_expr(child: Node) -> Node {
    Node::UnaryExpr {
        child: Box::new(child),
    }
}

fn parse_binary_expr(pair: pest::iterators::Pair<Rule>, lhs: Node, rhs: Node) -> Node {
    Node::BinaryExpr {
        op: match pair.as_str() {
            "&" => Operator::And,
            "|" => Operator::Or,
            "=>" => Operator::Imp,
            "<=>" => Operator::Eq,
            _ => unreachable!(),
        },
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
    }
}
