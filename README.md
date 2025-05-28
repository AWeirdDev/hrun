# 🚁 hrun

![Rust](https://img.shields.io/badge/Rust-%23000000.svg?logo=rust&logoColor=white)

![PyPI Downloads (weekly)](https://badgen.net/pypi/dw/hrun-rt)
![Contributors](https://badgen.net/github/contributors/AWeirdDev/hrun)
![Release](https://badgen.net/github/release/AWeirdDev/hrun)

**`H`** is a simple runtime designed to be fast and memory-safe. Available on PyPI with the name [`hrun-rt`](https://pypi.org/project/hrun-rt) ("rt" - runtime).

You may find it useful for:
- Writing simple scripts
- Learning AST
- Running unsafe code (e.g., from AI models)

> **New!** — Now with functions, thanks to the `HFunction` trait update.

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

***

# Documentation
The following components are already **available** as docstrings in Python:

- `Expr`
- `Statement`
- `H` (The runtime)

## <kbd>type</kbd> `Value`
```python
type Value = str | int | float | bool | None | list
```
A value. (Discoverable as `PyValue` in `src/lib.rs`, `Value` in `crates/h/src/lib.rs`)

It is recommended to cast a value to another if you're certain about a type at runtime.

## <kbd>type</kbd> `Identifier`
```python
type Identifier = str | int
```
An identifier. Could be a string or an integer; both have advantages & disadvantages.

## ✨ Declaring functions
Functions are available since `v0.1.2`.

First, create a Python function:

```python
# We need cast() from typing because we're *certain* 
# what the type would be
from typing import cast

from hrun import Value

def add(items: Value) -> Value:
    # Since the H runtime side would be calling 
    # this like add([A, B]), items: list[int]
    items = cast(list[int], items)
    a = items[0]
    b = items[1]

    return a + b
```

Then, define a simple code structure:

```python
from hrun import Statement, Expr

code = [
    Statement.fn("add", add),  # Name this function as "add"
    Statement.let("a", Expr.literal(1)),
    Statement.let("b", Expr.literal(2)),
    Statement.let(
        "result",
        Expr.call(
            "add",
            Expr.vector([
                Expr.ident("a"),
                Expr.ident("b")
            ])
        )
    )
]
```

Finally, run it with the `H` runtime:

```python
from hrun import H

h = H()
h.run(code)

print(h.get("result"))  # Output: 3
```
