use std::{ fmt::Debug, sync::Arc };

use dashmap::DashMap;

pub trait HFunction: HFunctionClone + Debug + Send + Sync {
    fn run(&self, args: Arc<Value>) -> Value;
}

pub trait HFunctionClone {
    fn clone_box(&self) -> Box<dyn HFunction>;
}

impl<T> HFunctionClone for T where T: 'static + HFunction + Clone {
    fn clone_box(&self) -> Box<dyn HFunction> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn HFunction> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

pub type BoxedHFunction = Box<dyn HFunction + 'static>;

#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Boolean(bool),
    String(String),
    Number(Number),
    Vector(Vec<Arc<Value>>),
    Function(BoxedHFunction),
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Function(_) => panic!("Cannot perform PartialEq for Value::Fn"),
            Self::Boolean(b) =>
                b.eq(
                    other
                        .as_bool()
                        .unwrap_or_else(|| panic!("Expected other item to be Value::Boolean"))
                ),
            Self::Null => other.is_null(),
            Self::Number(n) =>
                n.eq(
                    other
                        .as_number()
                        .unwrap_or_else(|| panic!("Expected other item to be Value::Number"))
                ),
            Self::String(s) =>
                s.eq(
                    other
                        .as_string()
                        .unwrap_or_else(|| panic!("Expected other item to be Value::String"))
                ),
            Self::Vector(v) =>
                v.eq(
                    other
                        .as_vector()
                        .unwrap_or_else(|| panic!("Expected other item to be Value::Vector"))
                ),
        }
    }
}

impl Value {
    pub fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    pub fn is_number(&self) -> bool {
        matches!(self, Self::Number(_))
    }

    pub fn as_number(&self) -> Option<&Number> {
        match self {
            Self::Number(n) => Some(n),
            _ => None,
        }
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn as_string(&self) -> Option<&String> {
        match self {
            Self::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn is_bool(&self) -> bool {
        matches!(self, Self::Boolean(_))
    }

    pub fn as_bool(&self) -> Option<&bool> {
        match self {
            Self::Boolean(b) => Some(b),
            _ => None,
        }
    }

    pub fn is_function(&self) -> bool {
        matches!(self, Self::Function(_))
    }

    pub fn as_function(&self) -> Option<&BoxedHFunction> {
        match self {
            Self::Function(f) => Some(f),
            _ => None,
        }
    }

    pub fn is_vector(&self) -> bool {
        matches!(self, Self::Vector(_))
    }

    pub fn as_vector(&self) -> Option<&Vec<Arc<Self>>> {
        match self {
            Self::Vector(v) => Some(v),
            _ => None,
        }
    }

    pub const fn null() -> Self {
        Self::Null
    }

    pub fn boolean(b: bool) -> Self {
        Self::Boolean(b)
    }

    pub fn string<K>(s: K) -> Self where String: From<K> {
        Self::String(String::from(s))
    }

    pub fn float(f: f64) -> Self {
        Self::Number(Number::Float(f))
    }

    pub fn int(i: i64) -> Self {
        Self::Number(Number::Int(i))
    }
}

impl AsRef<Value> for Value {
    fn as_ref(&self) -> &Value {
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Number {
    Float(f64),
    Int(i64),
}

macro_rules! exp {
    ($e:expr) => {
        $e
    };
}

macro_rules! noperator {
    ($name:ident, $op:tt) => {
        pub fn $name(&self, o: &Number) -> Number {
            match (self, o) {
                (Self::Float(a), Self::Int(b)) => Number::Float(exp!(*a $op (*b as f64))),
                (Self::Float(a), Self::Float(b)) => Number::Float(exp!(*a $op *b)),
                (Self::Int(a), Self::Float(b)) => Number::Float(exp!(*a as f64 $op *b)),
                (Self::Int(a), Self::Int(b)) => Number::Int(exp!(*a $op *b)),
            }
        }
    };
}

impl Number {
    noperator!(add, +);
    noperator!(sub, -);
    noperator!(mul, *);
    noperator!(div, /);
}

impl Number {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::Float(a), Self::Float(b)) => a.total_cmp(b),
            (Self::Float(a), Self::Int(b)) => a.total_cmp(&(*b as f64)),
            (Self::Int(a), Self::Int(b)) => a.cmp(b),
            (Self::Int(a), Self::Float(b)) =>
                match b.total_cmp(&(*a as f64)) {
                    std::cmp::Ordering::Greater => std::cmp::Ordering::Less,
                    std::cmp::Ordering::Less => std::cmp::Ordering::Greater,
                    a => a,
                }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Value),
    Ident(Identifier),
    BinaryOp(Box<Expr>, BinaryOperator, Box<Expr>),
    Equals(Box<Expr>, Box<Expr>),
    Not(Box<Expr>),
    GreaterThan(Box<Expr>, Box<Expr>),
    LessThan(Box<Expr>, Box<Expr>),
    Call(Identifier, Box<Expr>),
    Vector(Vec<Expr>),
}

impl Expr {
    /// Creates a literal expr
    pub fn literal(value: Value) -> Self {
        Self::Literal(value)
    }

    /// Creates an identifier expr
    pub fn ident(ident: Identifier) -> Self {
        Self::Ident(ident)
    }

    /// Creates a binary operation expr
    pub fn binary_op(a: Self, operator: BinaryOperator, b: Self) -> Self {
        Self::BinaryOp(Box::new(a), operator, Box::new(b))
    }

    /// Creates a equals expr
    pub fn equals(a: Self, b: Self) -> Self {
        Self::Equals(Box::new(a), Box::new(b))
    }

    /// Creates a not expr
    #[allow(clippy::should_implement_trait)]
    pub fn not(item: Self) -> Self {
        Self::Not(Box::new(item))
    }

    /// Creates a greater than expr
    pub fn greater_than(a: Self, b: Self) -> Self {
        Self::GreaterThan(Box::new(a), Box::new(b))
    }

    /// Creates a less than expr
    pub fn less_than(a: Self, b: Self) -> Self {
        Self::LessThan(Box::new(a), Box::new(b))
    }

    /// Calls a function
    pub fn call(ident: Identifier, args: Self) -> Self {
        Self::Call(ident, Box::new(args))
    }

    /// Creates a vector (i.e. `[expr_1, expr_2, expr_3, ...]`)
    pub fn vector(items: Vec<Self>) -> Self {
        Self::Vector(items)
    }
}

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
}

/// An identifier. Could be a `Identifier::U` (usize-based), or `Identifier::S` (String-based).
/// Choose one that fits your allocation preferences.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Identifier {
    /// A usize-based identifier.
    U(usize),

    /// A String-based identifier.
    S(String),
}

impl From<usize> for Identifier {
    fn from(value: usize) -> Self {
        Self::U(value)
    }
}

impl From<&str> for Identifier {
    fn from(value: &str) -> Self {
        Self::S(value.to_string())
    }
}

impl From<String> for Identifier {
    fn from(value: String) -> Self {
        Self::S(value)
    }
}

#[derive(Debug, Clone)]
pub enum Statement {
    Let(Identifier, Expr),
    IfElse {
        condition: Expr,
        then: Vec<Statement>,
        otherwise: Vec<Statement>, // are you expecting "else"?? nahhh
    },
    Fn(Identifier, BoxedHFunction),
}

#[derive(Debug)]
pub struct Machine {
    pub vars: DashMap<Identifier, Arc<Value>>,
}

impl Default for Machine {
    fn default() -> Self {
        Self::new()
    }
}

impl Machine {
    pub fn new() -> Self {
        Self {
            vars: DashMap::new(),
        }
    }

    pub fn set(&self, ident: Identifier, value: Arc<Value>) {
        self.vars.insert(ident, value);
    }

    pub fn get(&self, ident: &Identifier) -> Arc<Value> {
        self.vars
            .get(ident)
            .unwrap_or_else(|| panic!("Value cannot be found: {:?}", ident))
            .clone()
    }
}

pub fn deduce(machine: &Machine, statements: Vec<Statement>) {
    for stmt in statements {
        match stmt {
            Statement::Let(ident, expr) => {
                machine.set(ident, deduce_expr(machine, expr));
            }
            Statement::IfElse { condition, then, otherwise } => {
                let res = deduce_expr(machine, condition);
                if !res.is_bool() {
                    panic!("Expected deduced condition to be a bool, got other type");
                }
                if *res.as_bool().unwrap() {
                    deduce(machine, then);
                } else {
                    deduce(machine, otherwise);
                }
            }
            Statement::Fn(ident, function) => {
                machine.set(ident, Arc::new(Value::Function(function)));
            }
        }
    }
}

pub fn deduce_expr(machine: &Machine, expr: Expr) -> Arc<Value> {
    match expr {
        Expr::Ident(ident) => { machine.get(&ident) }
        Expr::Literal(lit) => { Arc::new(lit) }
        Expr::BinaryOp(a, op, b) => {
            let (va, vb) = (deduce_expr(machine, *a), deduce_expr(machine, *b));
            if va.is_number() && vb.is_number() {
                let (na, nb) = (va.as_number().unwrap(), vb.as_number().unwrap());
                let result = match op {
                    BinaryOperator::Add => na.add(nb),
                    BinaryOperator::Sub => na.sub(nb),
                    BinaryOperator::Mul => na.mul(nb),
                    BinaryOperator::Div => na.div(nb),
                };

                drop(va);
                drop(vb);

                Arc::new(Value::Number(result))
            } else if va.is_string() && vb.is_string() {
                let (sa, sb) = (va.as_string().unwrap(), vb.as_string().unwrap());

                let result = match op {
                    BinaryOperator::Add => format!("{sa}{sb}"),
                    _ => panic!("Unknown binary operation for Value::String"),
                };

                drop(va);
                drop(vb);

                Arc::new(Value::String(result))
            } else {
                panic!("Binary operator for unknown type")
            }
        }
        Expr::Equals(a, b) => {
            let (va, vb) = (deduce_expr(machine, *a), deduce_expr(machine, *b));
            Arc::new(Value::Boolean(va.eq(&vb)))
        }
        Expr::Not(item) => {
            let v = deduce_expr(machine, *item);
            if let Some(res) = v.as_bool() {
                Arc::new(Value::Boolean(*res))
            } else {
                panic!("The not expression must be used on a boolean value")
            }
        }
        Expr::GreaterThan(a, b) => {
            let (va, vb) = (deduce_expr(machine, *a), deduce_expr(machine, *b));
            if !va.is_number() || !vb.is_number() {
                panic!("Both left-hand and right-hand side values must be a number");
            }

            let (na, nb) = (va.as_number().unwrap(), vb.as_number().unwrap());
            match na.cmp(nb) {
                std::cmp::Ordering::Greater => Arc::new(Value::boolean(true)),
                _ => Arc::new(Value::boolean(false)),
            }
        }
        Expr::LessThan(a, b) => {
            let (va, vb) = (deduce_expr(machine, *a), deduce_expr(machine, *b));
            if !va.is_number() || !vb.is_number() {
                panic!("Both left-hand and right-hand side values must be a number");
            }

            let (na, nb) = (va.as_number().unwrap(), vb.as_number().unwrap());
            match na.cmp(nb) {
                std::cmp::Ordering::Less => Arc::new(Value::boolean(true)),
                _ => Arc::new(Value::boolean(false)),
            }
        }
        Expr::Call(ident, args) => {
            let va = deduce_expr(machine, *args);
            let function = machine.get(&ident);
            let function = function.as_function().unwrap();

            let result = function.run(va);
            Arc::new(result)
        }
        Expr::Vector(mut v) => {
            let items = v
                .drain(..)
                .map(|item| deduce_expr(machine, item))
                .collect::<Vec<_>>();
            Arc::new(Value::Vector(items))
        }
    }
}

// helper functions

/// Creates an identifier.
pub fn ident<K>(x: K) -> Identifier where Identifier: From<K> {
    Identifier::from(x)
}

/// Creates a literal expression.
pub fn literal(lit: Value) -> Expr {
    Expr::Literal(lit)
}
