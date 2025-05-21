import time
from hrun import H, Statement, Expr

h = H()
h.run(
    [
        Statement.let("a", Expr.literal(10.0)),
        Statement.let("b", Expr.binary_op(Expr.literal(-1), "*", Expr.ident("a"))),
        Statement.if_(
            Expr.greater_than(Expr.ident("a"), Expr.ident("b")),
            [Statement.let("c", Expr.literal("Then!"))],
            [Statement.let("c", Expr.literal("Else!"))],
        ),
    ]
)
print(h.get("c"))