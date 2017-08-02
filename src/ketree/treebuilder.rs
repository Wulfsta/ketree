//! A convenience struct to create symbolic trees.

use ketos::{
    Error,
    Interpreter,
    ModuleLoader,
    FromValue,
};
use std::collections::HashSet;
use std::io::prelude::*;
use std::io::BufReader;
use std::fmt::Debug;
use std::fs::File;
use std::rc::Rc;
use std::cell::RefCell;
use tree::{
    Tree,
    Expression,
};
use treeerror::{
    TreeError,
    TreeErrorKind,
};

/// Contains the code to be executed as a prologue, body, and epilogue.
pub struct TreeBuilder {
    kcode: String,
    prologue: String,
    epilogue: String,
}

impl TreeBuilder {
    /// Creates a new TreeBuilder with provided 'body' code.
    pub fn new(body: &str) -> TreeBuilder {
        TreeBuilder {
            kcode: body.to_string(),
            prologue: "".to_string(),
            epilogue: "".to_string(),
        }
    }

    /// Creates a new TreeBuilder from a file.
    pub fn from_file(path: &str) -> Result<TreeBuilder, ::std::io::Error> {
        let kf = File::open(path)?;
        let mut reader = BufReader::new(kf);
        let mut kfiledata = String::new();

        reader.read_to_string(&mut kfiledata)?;

        Ok(TreeBuilder {
            kcode: kfiledata,
            prologue: "".to_string(),
            epilogue: "".to_string(),
        })
    }

    /// Sets the prologue to be executed.
    pub fn set_prologue(&mut self, s: &str) {
        self.prologue = s.to_string();
    }

    /// Sets the epilogue to be executed.
    pub fn set_epilogue(&mut self, s: &str) {
        self.epilogue = s.to_string();
    }

    /// Consumes a Box containing a ketos::ModuleLoader and takes a name to look for that the tree
    ///  is assigned to (using define in the body of Ketos code).
    ///
    /// Returns a tuple containing the Tree and a HashSet of variable names.
    pub fn use_box_and_name<T, B>(&self, cml: Box<B>, tree_name: &str) -> Result<(Tree<T>, HashSet<String>), Error> 
        where T: 'static + Clone + Debug,
              B: 'static + ModuleLoader
    {
        self.use_opts(cml, None, tree_name)
    }

    /// Consumes a Box containing a ketos::ModuleLoader, consumes a HashSet<String> (to allow
    ///  for the selection of the hashing algorithm), and takes a name to look for that the tree
    ///  is assigned to (using define in the body of Ketos code).
    ///
    /// Returns a tuple containing the Tree and the consumed HashSet filled with variable names.
    pub fn use_box_set_and_name<T, B>(&self, cml: Box<B>, var_set: HashSet<String>, tree_name: &str) -> Result<(Tree<T>, HashSet<String>), Error> 
        where T: 'static + Clone + Debug,
              B: 'static + ModuleLoader
    {
        self.use_opts(cml, Some(var_set), tree_name)
    }

    // Helper function.
    fn use_opts<T, B>(&self, cml: Box<B>, var_set: Option<HashSet<String>>, tree_name: &str) -> Result<(Tree<T>, HashSet<String>), Error> 
        where T: 'static + Clone + Debug,
              B: 'static + ModuleLoader
    {
        let result: (Tree<T>, HashSet<String>);
        {
            let interp = Interpreter::with_loader(cml);

            let vs = match var_set {
                Some(vset) => vset,
                None => HashSet::<String>::new(),
            };
            let varcont = Rc::new(RefCell::new(vs));
            let varcontc = varcont.clone();

            let var_fn = move |s: &str| -> Result<Tree<T>, Error> {
                varcontc.borrow_mut().insert(s.to_string());
                Ok(Tree::<T>::new(Expression::Variable(s.to_string())))
            };
            
            // It turns out that this macro just uses the Fn trait and clever syntax.
            ketos_fn!{ interp.scope() => "var" => fn var_fn(s: &str) -> Tree<T> }

            interp.run_code(&format!("{}\n{}\n{}", self.prologue, self.kcode, self.epilogue), None)?;

            let m = Tree::from_value(match interp.get_value(tree_name) {
                Some(n) => n,
                None => { return Err(Error::custom(TreeError::create(TreeErrorKind::TreeNotInScope))); },
            })?;

            result = (m, varcont.borrow().clone());
        }
        Ok(result)
    }
}

