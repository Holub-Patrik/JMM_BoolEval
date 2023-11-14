use inkwell::{
    builder::Builder, context::Context, execution_engine::JitFunction, types::IntType,
    values::AnyValue, values::IntValue, OptimizationLevel, IntPredicate,
};

use crate::{Compile, Node, Operator, Result};

type JitFunc = unsafe extern "C" fn() -> i32;

pub struct Jit;

impl Compile for Jit {
    type Output = Result<i32>;

    fn from_ast(ast: Vec<Node>) -> Self::Output {
        let context = Context::create();
        let module = context.create_module("calculator");

        let builder = context.create_builder();

        let execution_engine = module
            .create_jit_execution_engine(OptimizationLevel::None)
            .unwrap();

        let i32_type = context.i32_type();
        let fn_type = i32_type.fn_type(&[], false);

        let function = module.add_function("jit", fn_type, None);
        let basic_block = context.append_basic_block(function, "entry");

        builder.position_at_end(basic_block);

        for node in ast {
            let recursive_builder = RecursiveBuilder::new(i32_type, &builder);
            let return_value = recursive_builder.build(&node);
            let _ = builder.build_return(Some(&return_value));
        }
        println!(
            "Generated LLVM IR: {}",
            function.print_to_string().to_string()
        );

        unsafe {
            let jit_function: JitFunction<JitFunc> = execution_engine.get_function("jit").unwrap();

            Ok(jit_function.call())
        }
    }
}

struct RecursiveBuilder<'a> {
    i32_type: IntType<'a>,
    builder: &'a Builder<'a>,
}

impl<'a> RecursiveBuilder<'a> {
    pub fn new(i32_type: IntType<'a>, builder: &'a Builder) -> Self {
        Self { i32_type, builder }
    }
    pub fn build(&self, ast: &Node) -> IntValue {
        match ast {
            Node::UnaryExpr { child } => {
                let child = self.build(child);
                child.const_not()
            }
            Node::BinaryExpr { op, lhs, rhs } => {
                let left = self.build(lhs);
                let right = self.build(rhs);

                match op {
                    Operator::And => self
                        .builder
                        .build_and(left, right, "and_temp")
                        .unwrap(),
                    Operator::Or => self
                        .builder
                        .build_or(left, right, "or_temp")
                        .unwrap(),
                    Operator::Eq => self
                        .builder
                        .build_int_compare(IntPredicate::EQ, left, right, "eq_temp")
                        .unwrap(),
                    Operator::Imp => self
                        .builder
                        .build_int_compare(IntPredicate::ULE, left, right, "impl_temp")
                        .unwrap(),
                }
            }
        }
    }
}
