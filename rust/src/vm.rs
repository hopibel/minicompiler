use core::fmt;

// Stack-based VM
// Supported instructions:
// Push <int>
// Plus
// Mult

pub enum Code {
    Push(i32),
    Plus,
    Mult,
}

pub struct VM {
    code: Vec<Code>,
    stack: Vec<i32>,
}

impl VM {
    pub fn new(code: Vec<Code>) -> Self {
        Self {
            code,
            stack: Vec::new(), // NOTE: rust automatically infers concrete type (Vec<i32>)
        }
    }

    // Result instead of Option to clearly communicate error
    pub fn run(&mut self) -> Result<i32, String> {
        // always start with empty stack
        self.stack.clear();

        // NOTE: iter -> borrow elements of vec (compare into_vec, which consumes vec)
        // enumerate to provide line number in errors
        for code in self.code.iter().enumerate() {
            match code.1 {
                Code::Push(val) => {
                    self.stack.push(*val);
                }
                Code::Plus => {
                    if self.stack.len() < 2 {
                        return Err(format!(
                            "L{} PLUS: not enough values on stack. Expected two",
                            code.0
                        ));
                    }
                    let right = self.stack.pop().unwrap();
                    let left = self.stack.pop().unwrap();
                    self.stack.push(left + right);
                }
                Code::Mult => {
                    if self.stack.len() < 2 {
                        return Err(format!(
                            "L{} MULT: not enough values on stack. Expected two",
                            code.0
                        ));
                    }
                    let right = self.stack.pop().unwrap();
                    let left = self.stack.pop().unwrap();
                    self.stack.push(left * right);
                }
            }
        }

        if let Some(result) = self.stack.last() {
            Ok(*result)
        } else {
            Err(format!(
                "L{} No return value found on stack",
                self.code.len()
            ))
        }
    }
}

// VM state to_string
impl fmt::Display for VM {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let code_string = self
            .code
            .iter()
            .map(|c| {
                match c {
                    Code::Push(val) => format!("Push {}", val),
                    Code::Plus => "Plus".to_string(),
                    Code::Mult => "Mult".to_string(),
                }
            })
            .collect::<Vec<String>>()
            .join(";");

        let stack_string = self
            .stack
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(",");

        write!(f, "Code: {}", code_string)?;
        write!(f, "Stack: {}", stack_string)?;

        Ok(())
    }
}
