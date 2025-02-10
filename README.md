# 🐑 `shear`: a library for trimming excess contents from things

this repository contains a library containing reusable facilities for trimming
various types to proscribed sizes.

this library also represents an experiment investigating the ergonomics of
control flow abstractions in Rust, in the style of Haskell's `Control.Monad`
library.

## background

i originally stumbled upon this idea when implementing a text-based
user-interface (tui), presenting some information as ascii tables within an
[`ncurses`] application.

[`ncurses`]: https://invisible-island.net/ncurses/

### an example: table rendering

for our purposes, we will imagine a table with two columns: `id`, `value`. the
`id` column should be four characters wide, and the `value` column should be
18 characters wide.

given a list like `[(1, "first"), (2, "second")]`, we might render:

```text,ignore
+----+------------------+
| id | value            |
+----+------------------+
| 01 | first            |
| 02 | second           |
+----+------------------+
```

now, suppose we encounter a row with a `"surprising long value"`? we might
truncate it with an ellipsis, rendering:

```text,ignore
+----+------------------+
| id | value            |
+----+------------------+
| 03 | surprising lo... |
+----+------------------+
```

further, we should _not_ truncate a value like `"has no room left"` that fits
the cell precisely, rendering:

```text,ignore
+----+------------------+
| id | value            |
+----+------------------+
| 04 | has no room left |
+----+------------------+
```

a relatively straight-forward way of implementing this rendering function
might look like the following:

```rust
fn main() {
    const PADDING: usize = 2;
    const WIDTH: usize = 18 - PADDING;

    // A simple value can be rendered.
    let first = "first";
    let expected = "first";
    assert_eq!(expected, render(first, WIDTH).as_str());

    // A long value can be truncated.
    let long = "surprising long value";
    let expected = "surprising lo...";
    debug_assert_eq!(expected.len(), WIDTH);
    assert_eq!(expected, render(long, WIDTH).as_str());

    // An exact fit is not truncated.
    let fits = "has no room left";
    debug_assert_eq!(expected.len(), WIDTH);
    assert_eq!(fits, render(fits, WIDTH).as_str());
}

/// Returns a trimmed string.
fn render(value: &str, width: usize) -> String {
    if value.len() <= width {
        return value.to_owned();
    }

    let mut out = value[0..width].to_owned();
    out.pop();
    out.pop();
    out.pop();
    out.extend("...".chars());

    out
}
```

initially, like many programs, this is not especially complicated.

astute readers may notice that above, i have described the width of table cells
in "characters". this is grounded upon an implicit assumption that we are
dealing with simple ascii text. suppose we wish to present unicode text
where characters can have different visual widths? suppose we wish to use a
different ellipsis, such as unicode's `…`?

or, suppose that we limit messages _vertically_, in order to render strings in
a table whose cells can contain more than one line? as new requirements or
features come into view, this function might quickly become more complicated.

### an example: finding a neighborhood in a graph

TK: graph example.

### bounding is a common problem

our example above belongs to a category of a very common problem in real-world
production software: bounding input.

while the example above concerned the logic to render a table in a text-based
interface of some sort, that code might look remarkably similar to the sort of
code one might write for a web server that limits the contents of a request
body before storing information in a database.

while the kind of input or dimension in which we measure it might change, this
task of limiting values is a relatively routine control flow sequence.

## reusable control flow abstraction

the core of this library is contained in `shear::iter`. this submodule provides
two things: (1) a `Limited` trait, which exposes hooks through which callers
can measure items and specify how to truncate long values, and (2) a
`LimitedIter<I>` iterator, which encapsulates a finite state machine
responsible for polling and possibly truncating an inner iterator.
