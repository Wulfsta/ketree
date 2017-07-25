//! A tree containing a symbolic expression that can be constructed using Ketos. 
//!
//! The tree can hold one of three types - an operator, a constant, or a
//!  variable. 

use ketos::{Value,
            FromValue,
            FromValueRef,
            ForeignValue,
            ExecError,
            };
use std::fmt::{self, Debug, Display};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::collections::HashMap;
use std::error;

// A struct that wraps Option<Vec<T>> with Ketos traits implemented.
#[derive(Clone, Debug)]
struct OptionVecWrapper<T: 'static + Sized + Clone + Debug>(Option<Vec<T>>);

// Simple constructor.
impl<T: 'static + Sized + Clone + Debug> OptionVecWrapper<T> {
    fn wrap(w: Option<Vec<T>>) -> OptionVecWrapper<T> {
        OptionVecWrapper(w)
    }
}

// Simple Deref.
impl<T: 'static + Sized + Clone + Debug> Deref for OptionVecWrapper<T> {
    type Target = Option<Vec<T>>;

    fn deref(&self) -> &Option<Vec<T>> {
        &self.0
    }
}

// Simple DerefMut.
impl<T: 'static + Sized + Clone + Debug> DerefMut for OptionVecWrapper<T> {
    fn deref_mut(&mut self) -> &mut Option<Vec<T>> {
        &mut self.0
    }
}

// From for ketos::Value.
impl<T: 'static + Sized + Clone + Debug> From<OptionVecWrapper<T>> for Value {
    fn from(v: OptionVecWrapper<T>) -> Self {
        Value::new_foreign(v)
    }
}

// Simple ketos::ForeignValue.
impl<T: 'static + Sized + Clone + Debug> ForeignValue for OptionVecWrapper<T> {
    fn type_name(&self) -> &'static str { "OptionVecWrapper" }
}

// Simple ketos::FromValue.
impl<T: 'static + Sized + Clone + Debug> FromValue for OptionVecWrapper<T> {
    fn from_value(v: Value) -> Result<Self, ExecError> {
        match v {
            Value::Foreign(fv) => {
                match ForeignValue::downcast_rc(fv) {
                    Ok(v) => {
                        match Rc::try_unwrap(v) {
                            Ok(v) => Ok(v),
                            Err(rc) => Ok((*rc).clone())
                        }
                    }
                    Err(rc) => {
                        Err(ExecError::expected("OptionVecWrapper", &Value::Foreign(rc)))
                    }
                }
            }
            ref v => Err(ExecError::expected("OptionVecWrapper", v))
        }
    }
}

// Simple ketos::FromValueRef
impl<'value, T: 'static + Sized + Clone + Debug> FromValueRef<'value> for &'value OptionVecWrapper<T> {
    fn from_value_ref(v: &'value Value) -> Result<Self, ExecError> {
        if let Value::Foreign(ref fv) = *v {
            if let Some(v) = fv.downcast_ref() {
                return Ok(v);
            }
         }

        Err(
            ExecError::expected("OptionVecWrapper", v))
    }
}

/// Enumerator for the types of data that a Tree vertex might contain.
/// 
/// Operator contains a function of fn(Vec<T>) -> T, Variable contains a String, and Constant
///  contains a type.
#[derive(Clone, Debug, ForeignValue, FromValueClone, FromValueRef)]
pub enum Expression<T: 'static + Clone + Debug> {
    Operator(fn(Vec<T>) -> T),
    Variable(String),
    Constant(T),
}

impl<T: 'static + Clone + Debug> From<Expression<T>> for Value {
    /// From for ketos::Value.
    fn from(v: Expression<T>) -> Self {
        Value::new_foreign(v)
    }
}

/// Simple Error types for use with Tree.
#[derive(Debug)]
pub struct TreeError {
    kind: TreeErrorKind,
    message: String,
}

impl TreeError {
    /// Convenience function to create an error.
    pub fn create(tek: TreeErrorKind) -> TreeError {
        match tek {
            TreeErrorKind::VarNotFound => TreeError {
                kind: TreeErrorKind::VarNotFound,
                message: "Found variable in tree but not map".to_string(),
            },
            TreeErrorKind::TreeNotInScope => TreeError {
                kind: TreeErrorKind::TreeNotInScope,
                message: "Tree not defined in scope - use (define $name $tree)".to_string(),
            },
        }
    }
}

impl Display for TreeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[derive(Debug)]
pub enum TreeErrorKind {
    VarNotFound,
    TreeNotInScope,
}

impl error::Error for TreeError {
    fn description(&self) -> &str {
        &self.message
    }
}

/// A Tree that represents a symbolic expression that can be constructed with Ketos.
#[derive(Clone, Debug, ForeignValue, FromValueClone, FromValueRef, StructValue)]
pub struct Tree<T: 'static + Clone + Debug> {
    data: Expression<T>,
    links: OptionVecWrapper<Tree<T>>,
}

impl<T: 'static + Clone + Debug> Tree<T> {
    /// Constructor to create a Tree vertex with data.
    pub fn new(ex: Expression<T>) -> Tree<T> {
        Tree { data: ex, links: OptionVecWrapper::<Tree<T>>::wrap(None) }
    }
    
    /// Reduces the complexity of the tree by evaluating operators with branches that are 
    ///  constants.
    pub fn reduce(&mut self) {
        let mut all_const = true;
        if let &Expression::Operator(_) = &self.data {
            match *self.links {
                Some(ref mut kinks) => {
                    for k in kinks.iter_mut() {
                        k.reduce();
                    }
                    for k in kinks.iter() {
                        if let &Expression::Operator(_) = k.data() {
                            all_const = false;
                            break;
                        } else if let &Expression::Variable(_) = k.data() {
                            all_const = false;
                            break;
                        }
                    }
                },
                None => { panic!("Operator found no operands") },
            };
        } else {
            all_const = false;
        }
        if all_const {
            self.data = Expression::Constant(self.reduce_helper());
            self.links = OptionVecWrapper::<Tree<T>>::wrap(None);
        }
    }

    // A helper for reduce.
    fn reduce_helper(&self) -> T {
        match &self.data {
            &Expression::Operator(ref f) => match *self.links {
                Some(ref kinks) => {
                    f(kinks.iter().map(|n| n.reduce_helper()).collect())
                },
                None => { panic!("Operator found no operands") },
            },
            &Expression::Constant(ref c) => c.clone(),
            _ => { panic!("Found variable when none were expected") },
        }
    }

    // An internal helper, used in reduce.
    fn data(&self) -> &Expression<T> {
       &self.data
    }

    /// Evaluates the tree with the provided map of variables.
    ///
    /// 'vars' should be any HashMap that maps Variables in the tree to values.
    pub fn accumulate(&self, vars: &HashMap<String, T>) -> Result<T, TreeError> {
        match &self.data {
            &Expression::Operator(ref f) => match *self.links {
                Some(ref kinks) => {
                    //f(kinks.iter().map(|n| {
                    //    match n.accumulate(vars) {
                    //        Ok(nu) => nu,
                    //        Err(e) => { return Err(e); }, // Fails due to return from
                    //    }                                 //  closure; not the outer
                    //}).collect())                         //  function.

                    let mut minks = Vec::<T>::with_capacity(kinks.capacity());
                    for k in kinks.iter() {
                        let u = match k.accumulate(vars) {
                            Ok(n) => n,
                            Err(e) => { return Err(e); },
                        };
                        minks.push(u);
                    }
                    Ok(f(minks))
                },
                None => { panic!("Operator found no operands") },
            },
            &Expression::Variable(ref v) => match vars.get(v) {
                Some(val) => Ok(val.clone()),
                None => Err(TreeError::create(TreeErrorKind::VarNotFound)),
            },
            &Expression::Constant(ref c) => Ok(c.clone()),
        }
    }
 
    /// Links the provided Vector of Trees as branches to self.
    pub fn link(&mut self, b: Vec<Tree<T>>) {
        self.links = OptionVecWrapper::wrap(Some(b));
    }

    /// Clones and returns contained data.
    pub fn clone_data(&self) -> Expression<T> {
        self.data.clone()
    }

    /// Clones and returns contained links.
    pub fn clone_links(&self) -> Option<Vec<Tree<T>>> {
        let ref holder = *self.links;
        holder.clone()
    }
}

impl<T: 'static + Clone + Debug> From<Tree<T>> for Value {
    /// From for ketos::Value.
    fn from(v: Tree<T>) -> Self {
        Value::new_foreign(v)
    }
}

