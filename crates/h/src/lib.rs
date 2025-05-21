use std::sync::Arc;

use dashmap::DashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Boolean(bool),
    String(String),
    Number(Number),
}

impl Value {
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
}

#[derive(Debug)]
pub struct Machine {
    pub vars: DashMap<Identifier, Arc<Value>>,
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
        self.vars.get(ident).expect(&format!("Value cannot be found: {:?}", ident)).clone()
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
