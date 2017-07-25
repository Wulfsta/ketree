#[macro_use] extern crate ketos;
extern crate ketree;

use std::collections::HashMap;
use ketree::TreeBuilder;
use ketos::{
    BuiltinModuleLoader,
    CompileError,
    Context,
    Error,
    Module, 
    ModuleBuilder, 
    ModuleLoader,
    Name,
    Scope,
};
use ketree::{
    Tree,
    Expression,
};

#[test]
fn test_treebuilder() {
    let mut builder = TreeBuilder::from_file("tests/ketreetest")
        .expect("Failed to create TreeBuilder");

    builder.set_prologue(TreeModuleLoader::prologue());
    builder.set_epilogue(TreeModuleLoader::epilogue());

    let mut result = builder.use_box_and_name(Box::new(TreeModuleLoader.chain(BuiltinModuleLoader)), "tree")
        .expect("Failed to build tree.");

    assert_eq!(Some(&"x".to_string()), result.1.get("x"));

    result.0.reduce();

    let mut vars = HashMap::<String, f32>::with_capacity(1);
    vars.insert("x".to_string(), 2.0);

    assert_eq!(7.0, result.0.accumulate(&vars).unwrap());
}

type NumT = f32;

#[derive(Debug)]
pub struct TreeModuleLoader;

impl TreeModuleLoader {
    #[inline]
    pub fn prologue<'a>() -> &'a str {
        "(use treebuilder :all)"
    }

    #[inline]
    pub fn epilogue<'a>() -> &'a str {
        ""
    }
}

impl ModuleLoader for TreeModuleLoader {
    fn load_module(&self, name: Name, ctx: Context) -> Result<Module, Error> {
        let load_custom = ctx.scope().with_name(name, |name| name == "treebuilder");

        if load_custom {
            Ok(load_mod(ctx.scope()))
        } else {
            Err(From::from(CompileError::ModuleError(name)))
        }
    }
}

fn load_mod(scope: &Scope) -> Module {
    scope.register_struct_value::<Tree<NumT>>();

    ketos_fn!{ scope => "con" => fn con(c: NumT) -> Tree<NumT> }
    ketos_fn!{ scope => "plus" => fn plus(lhs: &Tree<NumT>, rhs: &Tree<NumT>) -> Tree<NumT> }

    ModuleBuilder::new("treebuilder", scope.clone()).finish()
}

fn con(c: NumT) -> Result<Tree<NumT>, Error> {
    Ok(Tree::new(Expression::Constant(c)))
}

fn plus(lhs: &Tree<NumT>, rhs: &Tree<NumT>) -> Result<Tree<NumT>, Error> {

    fn int_plus(v: Vec<NumT>) -> NumT {
        v.iter().fold(0.0, |sum, i| sum + i)
    }

    let mut bt = Tree::new(Expression::Operator(int_plus));
    bt.link(vec![lhs.clone(), rhs.clone()]);
    Ok(bt)
}

