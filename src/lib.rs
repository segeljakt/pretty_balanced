use std::fmt::{Display, Formatter, Result, Write};

pub struct PrettyBalancedDisplay(String);

pub trait PrettyBalanced {
    fn pretty_balanced(&self) -> PrettyBalancedDisplay;
}

impl<T: Display> PrettyBalanced for T {
    fn pretty_balanced(&self) -> PrettyBalancedDisplay {
        PrettyBalancedDisplay(self.to_string())
    }
}

fn newline(f: &mut Formatter, indent: usize) -> Result {
    f.write_char('\n')?;
    for _ in 0..indent {
        write!(f, "    ")?
    }
    Ok(())
}

impl Display for PrettyBalancedDisplay {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut indent = 0;
        let PrettyBalancedDisplay(string) = self;
        for ch in string.chars() {
            match ch {
                '{' | '(' | '<' | '[' => {
                    indent += 1;
                    f.write_char(ch)?;
                    newline(f, indent)?;
                }
                '}' | ')' | '>' | ']' => {
                    indent -= 1;
                    newline(f, indent)?;
                    f.write_char(ch)?;
                }
                ';' => {
                    f.write_char(ch)?;
                    newline(f, indent)?;
                }
                _ => {f.write_char(ch)?},
            }
            
        }
        Ok(())
    }
}
