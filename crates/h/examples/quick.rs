use h::{ deduce, ident, literal, BinaryOperator, Expr, Scope, Statement, Value };

fn main() {
    let machine = Scope::new();
    let code = vec![
        Statement::Let(
            ident(1),
            Expr::binary_op(
                literal(Value::float(1.23)),
                BinaryOperator::Sub,
                literal(Value::int(123))
            )
        ),
        Statement::Let(
            ident(1),
            Expr::binary_op(
                literal(Value::string("hello ")),
                BinaryOperator::Add,
                literal(Value::string("world!"))
            )
        )
    ];
    deduce(&machine, code);
    println!("{machine:?}");
}
