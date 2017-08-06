#[macro_use] extern crate ketos;
extern crate ketree;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;
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
    TreeBuilder,
    Expression,
};

#[test]
fn test_treebuilder() {
    let kf = File::open("tests/ketreetest")
        .expect("Failed to find file");
    let mut reader = BufReader::new(kf);
    let mut kfiledata = String::new();

    reader.read_to_string(&mut kfiledata)
        .expect("Failed to read file");

    let builder = TreeBuilder::new(&kfiledata, TreeModuleLoader::prologue(), TreeModuleLoader::epilogue());

    let result = builder.use_box_and_name(Box::new(TreeModuleLoader.chain(BuiltinModuleLoader)), "tree")
        .expect("Failed to build tree");

    assert_eq!(Some(&"x".to_string()), result.1.get("x"));

    for i in result.0.post_iter() {
        println!("{:?}\n", i);
    }

    let mut vars = HashMap::<String, f32>::with_capacity(1);
    vars.insert("x".to_string(), 2.0);

    assert_eq!(6.0, result.0.accumulate(&vars).unwrap());
}

type NumT = f32;

#[derive(Debug)]
pub struct TreeModuleLoader;

impl TreeModuleLoader {
    #[inline]
    pub fn prologue<'a>() -> &'a str {
        "\n
        (use treebuilder :all) \n
        (let ((+ plus) (* mult)) (do \n"
    }

    #[inline]
    pub fn epilogue<'a>() -> &'a str {
        "))"
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
    ketos_fn!{ scope => "mult" => fn mult(lhs: &Tree<NumT>, rhs: &Tree<NumT>) -> Tree<NumT> }

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
    // Tree wraps Rc, so clone is not expensive.
    bt.link(vec![lhs.clone(), rhs.clone()]);
    Ok(bt)
}

fn mult(lhs: &Tree<NumT>, rhs: &Tree<NumT>) -> Result<Tree<NumT>, Error> {

    fn int_mult(v: Vec<NumT>) -> NumT {
        v.iter().fold(1.0, |m, i| m * i)
    }

    let mut bt = Tree::new(Expression::Operator(int_mult));
    // Just in case you didn't read the previous function, 
    //  Tree wraps Rc, so clone is not expensive.
    bt.link(vec![lhs.clone(), rhs.clone()]);
    Ok(bt)
}

