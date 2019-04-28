# pretty_balanced

A library for displaying code in human-readable format.

# Example

```
use pretty_balanced::PrettyBalanced;

fn main() {
    println!("{}", "if 3 { foo; bar } else { baz }".pretty_balanced());
}
```

Output:

```
if 3 {
     foo;
     bar
} else {
     baz
}
```

The formatting simply inserts newline and indentation after `(){}[]<>;` characters.
