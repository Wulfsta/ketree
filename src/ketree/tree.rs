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
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::collections::HashMap;
use treeerror::{TreeError, TreeErrorKind};

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

// A backend that represents a symbolic expression.
#[derive(Clone, Debug, ForeignValue, FromValueClone, FromValueRef, StructValue)]
struct TreeBackend<T: 'static + Clone + Debug> {
    data: Expression<T>,
    links: OptionVecWrapper<Tree<T>>,
}

impl<T: 'static + Clone + Debug> TreeBackend<T> {
    // Constructor to create a Tree vertex with data.
    fn new(ex: Expression<T>) -> TreeBackend<T> {
        TreeBackend { data: ex, links: OptionVecWrapper::<Tree<T>>::wrap(None) }
    }
    
    // Links the provided Vector of Trees as branches to self.
    fn link(&mut self, b: Vec<Tree<T>>) {
        self.links = OptionVecWrapper::wrap(Some(b));
    }

    // Returns a reference to contained data.
    fn data(&self) -> &Expression<T> {
        &self.data
    }

    // Returns a reference to contained children.
    fn children(&self) -> &Option<Vec<Tree<T>>> {
        &self.links
    }
}

impl<T: 'static + Clone + Debug> From<TreeBackend<T>> for Value {
    // From for ketos::Value.
    fn from(v: TreeBackend<T>) -> Self {
        Value::new_foreign(v)
    }
}

// A struct that wraps Rc<T> with Ketos traits implemented.
#[derive(Clone, Debug)]
struct RcWrapper<T: 'static + Sized + Clone + Debug>(Rc<T>);

// Simple constructor.
impl<T: 'static + Sized + Clone + Debug> RcWrapper<T> {
    fn wrap(w: Rc<T>) -> RcWrapper<T> {
        RcWrapper(w)
    }
}

// Simple Deref.
impl<T: 'static + Sized + Clone + Debug> Deref for RcWrapper<T> {
    type Target = Rc<T>;

    fn deref(&self) -> &Rc<T> {
        &self.0
    }
}

// Simple DerefMut.
impl<T: 'static + Sized + Clone + Debug> DerefMut for RcWrapper<T> {
    fn deref_mut(&mut self) -> &mut Rc<T> {
        &mut self.0
    }
}

// From for ketos::Value.
impl<T: 'static + Sized + Clone + Debug> From<RcWrapper<T>> for Value {
    fn from(v: RcWrapper<T>) -> Self {
        Value::new_foreign(v)
    }
}

// Simple ketos::ForeignValue.
impl<T: 'static + Sized + Clone + Debug> ForeignValue for RcWrapper<T> {
    fn type_name(&self) -> &'static str { "RcWrapper" }
}

// Simple ketos::FromValue.
impl<T: 'static + Sized + Clone + Debug> FromValue for RcWrapper<T> {
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
                        Err(ExecError::expected("RcWrapper", &Value::Foreign(rc)))
                    }
                }
            }
            ref v => Err(ExecError::expected("RcWrapper", v))
        }
    }
}

// Simple ketos::FromValueRef
impl<'value, T: 'static + Sized + Clone + Debug> FromValueRef<'value> for &'value RcWrapper<T> {
    fn from_value_ref(v: &'value Value) -> Result<Self, ExecError> {
        if let Value::Foreign(ref fv) = *v {
            if let Some(v) = fv.downcast_ref() {
                return Ok(v);
            }
         }

        Err(
            ExecError::expected("RcWrapper", v))
    }
}

/// A Tree that represents a symbolic expression that can be constructed with Ketos.
#[derive(Clone, Debug, ForeignValue, FromValueClone, FromValueRef, StructValue)]
pub struct Tree<T: 'static + Sized + Clone + Debug> {
    t: RcWrapper<TreeBackend<T>>
}

impl<T: 'static + Sized + Clone + Debug> Tree<T> {
    /// Constructor to create a Tree vertex with data.
    pub fn new(ex: Expression<T>) -> Tree<T> {
        Tree { t: RcWrapper::wrap(Rc::new(TreeBackend::new(ex))) }
    }
    
    /// Evaluates the tree with the provided map of variables.
    ///
    /// 'vars' should be any HashMap that maps Variables in the tree to values.
    pub fn accumulate(&self, vars: &HashMap<String, T>) -> Result<T, TreeError> {
        let mut yeta = Vec::<T>::new();
        for i in self.post_iter() {
            match i.data() {
                &Expression::Operator(ref f) => {
                    let minks = match i.children() {
                        &Some(ref m) => m,
                        &None => { panic!("Operator found no operands") },
                    };
                    let s = yeta.len() - minks.len();
                    let eta = yeta.split_off(s);
                    yeta.push(f(eta));
                },
                &Expression::Variable(ref v) => match vars.get(v) {
                    Some(val) => yeta.push(val.clone()),
                    None => { return Err(TreeError::create(TreeErrorKind::VarNotFound)); },
                },
                &Expression::Constant(ref c) => yeta.push(c.clone()),
            };
        }
        Ok(yeta.pop().unwrap())
    }
    
    /// Links the provided Vector of Trees as branches to self.
    pub fn link(&mut self, b: Vec<Tree<T>>) {
        Rc::make_mut(&mut self.t).link(b);
    }

    /// Returns a reference to contained data.
    pub fn data(&self) -> &Expression<T> {
        &self.t.data()
    }

    /// Returns a reference to contained children.
    pub fn children(&self) -> &Option<Vec<Tree<T>>> {
        &self.t.children()
    }

    /// Returns a TreePostIter for the tree.
    pub fn post_iter(&self) -> TreePostIter<T> {
        let mut ps = Vec::<usize>::new();
        ps.push(0);
        let mut ts = Vec::<&Tree<T>>::new();
        ts.push(self);
        TreePostIter {
            pos_stack: ps,
            tree_stack: ts,
        }
    }
}

impl<T: 'static + Clone + Debug> From<Tree<T>> for Value {
    /// From for ketos::Value.
    fn from(v: Tree<T>) -> Self {
        Value::new_foreign(v)
    }
}

/// An Iterator that iterates through the tree in post-order.
pub struct TreePostIter<'a, T: 'static + Clone + Debug> {
    pos_stack: Vec<usize>,
    tree_stack: Vec<&'a Tree<T>>,
}

impl<'a, T: 'static + Clone + Debug> Iterator for TreePostIter<'a, T> {
    type Item = &'a Tree<T>;

    // index magic
    /// Returns a reference to the next vertex of the tree in post-order.
    fn next(&mut self) -> Option<&'a Tree<T>> {
        loop {
            let minks = self.tree_stack[self.tree_stack.len() - 1].children();
            let clen = || -> usize {
                match minks {
                    &Some(ref vc) => vc.len(),
                    &None => 0,
                }
            };
            if self.pos_stack[self.pos_stack.len() - 1] < clen() {
                if let &Some(ref vc) = self.tree_stack[self.tree_stack.len() - 1].children() {
                    self.tree_stack.push(&vc[self.pos_stack[self.pos_stack.len() - 1]]);
                    self.pos_stack.push(0);
                    continue;
                }
                else {
                    panic!("Iteration failed unexpectedly");
                }
            }
            else if self.pos_stack[self.pos_stack.len() - 1] == clen() {
                let s = self.pos_stack.len() - 1;
                self.pos_stack[s] += 1;
                break;
            }
            self.tree_stack.pop();
            if 0 == self.tree_stack.len() {
                return None;
            }
            self.pos_stack.pop();
            let s = self.pos_stack.len() - 1;
            self.pos_stack[s] += 1;
        }
        Some(self.tree_stack[self.tree_stack.len() - 1])
    }
}

