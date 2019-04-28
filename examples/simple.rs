use pretty_balanced::PrettyBalanced;

fn main() {
    println!("{}", "if 3 { foo; bar } else { baz } ".pretty_balanced());
}
