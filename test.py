from typing import cast
from hrun import H, Statement, Expr, Value


def add(item: Value) -> Value:
    item = cast(list[int], item)
    a = item[0]
    b = item[1]
    return a + b


h = H()
h.run(
    [
        Statement.let("a", Expr.literal(10.0)),
        Statement.let("b", Expr.binary_op(Expr.literal(-1), "*", Expr.ident("a"))),
        Statement.fn("add", add),
        Statement.if_(
            Expr.greater_than(Expr.ident("a"), Expr.ident("b")),
            [Statement.let("c", Expr.literal("Then!"))],
            [Statement.let("c", Expr.literal("Else!"))],
        ),
        Statement.let(
            "d", Expr.call("add", Expr.vector([Expr.ident("a"), Expr.ident("b")]))
        ),
    ]
)
print(h.get("c"))
print(h.get("d"))
