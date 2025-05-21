# 🚁 hrun

![Rust](https://img.shields.io/badge/Rust-%23000000.svg?logo=rust&logoColor=white)

![PyPI Downloads (weekly)](https://badgen.net/pypi/dw/hrun-rt)
![Contributors](https://badgen.net/github/contributors/AWeirdDev/hrun)
![Release](https://badgen.net/github/release/AWeirdDev/hrun)

**`H`** is a simple runtime designed to be fast and memory-safe.

You may find it useful for:
- Writing simple scripts
- Learning AST
- Running unsafe code (e.g., from AI models)

First, create a new H runtime.

```python
from hrun import H, Statement, Expr

h = H()
# Machine { vars: {} }
```

Then, create your code statements:

<table>
<tr>
<th>hrun</th>
<th>Equivalent code</th>
</tr>
<tr>
<td>

```python
code = [
    Statement.let("a", Expr.literal(10.0)),
    Statement.let(
        "b",
        Expr.binary_op(
            Expr.literal(-1), "*", Expr.ident("a")
        )
    ),
    Statement.if_(
        Expr.greater_than(Expr.ident("a"), Expr.ident("b")),
        [Statement.let("c", Expr.literal("Yes!"))],
        [Statement.let("c", Expr.literal("Nope"))],
    ),
]
```

</td>
<td>

```python
a = 10.0
b = -1 * a

if a > b:
    c = "Yes!"
else:
    c = "Nope"







```

</td>
</tr>
</table>

Finally, run it and get the value of `c`!

```python
h.run(code)
print(h.get("c"))
# Console output: Yes!
```
