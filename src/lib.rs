use pyo3::{ exceptions, prelude::*, types::PyNone };
use h::{ deduce, BinaryOperator, Expr, Identifier, Machine, Number, Statement, Value };

#[derive(FromPyObject)]
enum PyIdentifier {
    U(usize),
    S(String),
}

impl PyIdentifier {
    fn into_hident(self) -> Identifier {
        match self {
            Self::U(u) => Identifier::U(u),
            Self::S(s) => Identifier::S(s),
        }
    }
}

#[derive(FromPyObject, IntoPyObject)]
enum PyValue {
    #[allow(dead_code)] Null(Py<PyNone>),
    Boolean(bool),
    String(String),
    I64(i64),
    F64(f64),
}

impl PyValue {
    fn into_hvalue(self) -> Value {
        match self {
            Self::Boolean(b) => Value::boolean(b),
            Self::F64(f) => Value::float(f),
            Self::I64(i) => Value::int(i),
            Self::Null(_) => Value::Null,
            Self::String(s) => Value::String(s),
        }
    }
}

#[pyclass(name = "Statement")]
#[derive(Clone)]
struct PyStatement {
    stmt: Statement,
}

#[pymethods]
impl PyStatement {
    #[staticmethod]
    fn r#let(ident: PyIdentifier, pxpr: PyExpr) -> Self {
        let expr = pxpr.expr;
        Self { stmt: Statement::Let(ident.into_hident(), expr) }
    }

    #[staticmethod]
    fn if_(pxpr: PyExpr, mut then: Vec<PyStatement>, mut otherwise: Vec<PyStatement>) -> Self {
        let expr = pxpr.expr;
        Self {
            stmt: Statement::IfElse {
                condition: expr,
                then: then
                    .drain(..)
                    .map(|item| item.stmt)
                    .collect(),
                otherwise: otherwise
                    .drain(..)
                    .map(|item| item.stmt)
                    .collect(),
            },
        }
    }
}

#[pyclass(name = "Expr")]
#[derive(Clone)]
struct PyExpr {
    expr: Expr,
}

#[pymethods]
impl PyExpr {
    #[staticmethod]
    fn literal(value: PyValue) -> Self {
        let hv = value.into_hvalue();

        Self { expr: Expr::literal(hv) }
    }

    #[staticmethod]
    fn ident(id: PyIdentifier) -> Self {
        Self { expr: Expr::ident(id.into_hident()) }
    }

    #[staticmethod]
    fn binary_op(a: PyExpr, o: String, b: PyExpr) -> PyResult<Self> {
        let operator = match &*o {
            "+" => BinaryOperator::Add,
            "-" => BinaryOperator::Sub,
            "*" => BinaryOperator::Mul,
            "/" => BinaryOperator::Div,
            _ => {
                return Err(exceptions::PyValueError::new_err("Expected one of: +, -, *, /"));
            }
        };
        Ok(Self { expr: Expr::binary_op(a.expr, operator, b.expr) })
    }

    #[staticmethod]
    fn equals(a: PyExpr, b: PyExpr) -> Self {
        Self { expr: Expr::equals(a.expr, b.expr) }
    }

    #[staticmethod]
    fn not_(item: PyExpr) -> Self {
        Self { expr: Expr::not(item.expr) }
    }

    #[staticmethod]
    fn greater_than(a: PyExpr, b: PyExpr) -> Self {
        Self { expr: Expr::greater_than(a.expr, b.expr) }
    }

    #[staticmethod]
    fn less_than(a: PyExpr, b: PyExpr) -> Self {
        Self { expr: Expr::less_than(a.expr, b.expr) }
    }
}

#[pyclass(name = "H")]
struct PyH {
    machine: Machine,
}

#[pymethods]
impl PyH {
    #[new]
    fn py_new() -> Self {
        Self { machine: Machine::new() }
    }

    fn run(&self, mut stmts: Vec<PyStatement>) {
        deduce(
            &self.machine,
            stmts
                .drain(..)
                .map(|item| item.stmt)
                .collect()
        );
    }

    fn get(&self, py: Python<'_>, ident: PyIdentifier) -> PyResult<PyValue> {
        let t = self.machine.get(&ident.into_hident());
        match t.as_ref() {
            Value::Boolean(b) => Ok(PyValue::Boolean(*b)),
            Value::Null => Ok(PyValue::Null(py.None().extract::<Py<PyNone>>(py)?)),
            Value::Number(n) => {
                match n {
                    Number::Float(f) => Ok(PyValue::F64(*f)),
                    Number::Int(i) => Ok(PyValue::I64(*i)),
                }
            }
            Value::String(s) => Ok(PyValue::String(s.to_string())),
        }
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.machine)
    }
}

#[pymodule]
fn hrun(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyH>()?;
    m.add_class::<PyStatement>()?;
    m.add_class::<PyExpr>()?;
    Ok(())
}
