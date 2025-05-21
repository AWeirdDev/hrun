from typing import List, Literal
from .types import Identifier, Value


class H:
    """Represents the H runtime for hrun."""

    def __init__(self): ...
    def run(self, stmts: List[Statement]):
        """Runs a statement.

        Args:
            stmts (list[Statement]): A list of statements.
        """

    def get(self, ident: Identifier) -> Value:
        """Gets a value from the H runtime.

        **Technical details**: since Python is a dynamic language, while transcribing the Rust `Arc<T>` to
        Python values, some values may be **cloned** or copied. Performance may be affected.

        Args:
            ident (Identifier): The identifier.
        """

    def __repr__(self) -> str: ...


class Expr:
    """Represents an expression."""

    @staticmethod
    def literal(value: Value) -> "Expr":
        """Creates a literal expression.

        A literal expression contains a fixed value.

        Args:
            value (Value): The value.
        """

    @staticmethod
    def ident(ident: Identifier) -> "Expr":
        """Creates a ident (load) expression.

        It tells the H runtime to lookup a variable with the identifier.

        Args:
            ident (Identifier): The identifier.
        """

    @staticmethod
    def binary_op(a: Expr, o: Literal["+", "-", "*", "/"], b: Expr) -> "Expr":
        """Creates a binary operation expression.

        It tells the H runtime to perform binary operation on the two expressions.

        Args:
            a (Expr): Left hand-side expression.
            o (Literal["+", "-", "*", "/"]): The binary operator.
            b (Expr): Right hand-side expression.
        """

    @staticmethod
    def equals(a: Expr, b: Expr) -> "Expr":
        """Creates a `==` expression.

        The deduced **value** and **type** of `a` and `b` must match.

        Args:
            a (Expr): Left hand-side expression.
            b (Expr): Right hand-side expression.
        """

    @staticmethod
    def not_(item: Expr) -> "Expr":
        """Creates a `not` (`!`) expression.

        Args:
            item (Expr): The expression. Must be a boolean.
        """

    @staticmethod
    def greater_than(a: Expr, b: Expr) -> "Expr":
        """Creates a `>` expression.

        Args:
            a (Expr): An expression. Must be a number.
            b (Expr): An expression. Must be a number.
        """

    @staticmethod
    def less_than(a: Expr, b: Expr) -> "Expr":
        """Creates a `>` expression.

        Args:
            a (Expr): An expression. Must be a number.
            b (Expr): An expression. Must be a number.
        """


class Statement:
    """Represents a statement."""

    @staticmethod
    def let(ident: Identifier, expr: Expr) -> "Statement":
        """Creates a let statement.

        It tells the H runtime to create a variable on the said expression, which eventually
        gets deduced into a value.

        Args:
            ident (Identifier): The identifier.
            expr (Expr): The expression.
        """

    @staticmethod
    def if_(
        expr: Expr, then: List[Statement], otherwise: List[Statement]
    ) -> "Statement":
        """Creates a if/else branch statement.

        Args:
            expr (Expr): The expression. The value returned **must** be a boolean.
            then (list[Statement]): The `if [...] then...` branch.
            otheriwse (list[Statement]): The `...else` branch.
        """
