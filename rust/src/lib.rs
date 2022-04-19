pub mod tokenizer;
pub mod parser;

// AST as recursive data structure
// alternative: enum Exp using pattern matching in eval implementation
mod ast {
    pub trait Exp {
        fn eval(&self) -> i32;
        fn pretty(&self) -> String;
        fn smart_pretty(&self, is_subexpression: bool) -> String;
        fn compile(&self) { todo!("Exp -> opcode sequence") }
    }

    struct IntExp {
        val: i32,
    }

    impl IntExp {
        fn new(val: i32) -> IntExp {
            IntExp { val }
        }
    }

    impl Exp for IntExp {
        fn eval(&self) -> i32 {
            self.val // NOTE: last expression (no semicolon) is returned implicitly
        }

        fn pretty(&self) -> String {
            self.val.to_string()
        }

        #[allow(unused_variables)]
        fn smart_pretty(&self, is_subexpression: bool) -> String {
            self.pretty()
        }
    }

    pub struct PlusExp {
        l: Box<dyn Exp>,
        r: Box<dyn Exp>,
    }

    impl PlusExp {
        pub fn new(l: Box<dyn Exp>, r: Box<dyn Exp>) -> Self {
            Self { l, r }
        }

        pub fn new_static(l: impl Exp + 'static, r: impl Exp + 'static) -> Self {
            Self { l: Box::new(l), r: Box::new(r) }
        }
    }

    impl Exp for PlusExp {
        fn eval(&self) -> i32 {
            self.l.eval() + self.r.eval()
        }

        fn pretty(&self) -> String {
            format!("({}+{})", self.l.pretty(), self.r.pretty())
        }

        fn smart_pretty(&self, is_subexpression: bool) -> String {
            let inner = format!("{}+{}", self.l.smart_pretty(false), self.r.smart_pretty(false));
            if is_subexpression {
                format!("({})", inner)
            } else {
                inner
            }
        }
    }

    pub struct MultExp {
        l: Box<dyn Exp>,
        r: Box<dyn Exp>,
    }

    impl MultExp {
        pub fn new(l: Box<dyn Exp>, r: Box<dyn Exp>) -> Self {
            Self { l, r }
        }

        pub fn new_static(l: impl Exp + 'static, r: impl Exp + 'static) -> Self {
            Self { l: Box::new(l), r: Box::new(r) }
        }
    }

    impl Exp for MultExp {
        fn eval(&self) -> i32 {
            self.l.eval() * self.r.eval()
        }

        fn pretty(&self) -> String {
            format!("({}*{})", self.l.pretty(), self.r.pretty())
        }

        #[allow(unused_variables)]
        fn smart_pretty(&self, is_subexpression: bool) -> String {
            format!("{}*{}", self.l.smart_pretty(true), self.r.smart_pretty(true))
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn eval_int() {
            let n = 42;
            let exp = IntExp { val: n };
            assert_eq!(exp.eval(), n);
        }

        #[test]
        fn eval_plus() {
            let exp = PlusExp::new_static(
                IntExp::new(2),
                IntExp::new(2)
            );
            assert_eq!(exp.eval(), 2 + 2);
        }

        #[test]
        fn eval_mult() {
            let exp = MultExp::new_static(
                IntExp::new(6),
                IntExp::new(9)
            );
            assert_ne!(exp.eval(), 42);
            assert_eq!(exp.eval(), 6 * 9);
        }

        #[test]
        fn pretty_print() {
            let exp = PlusExp::new_static(
                MultExp::new_static(
                    PlusExp::new_static(
                        IntExp::new(1),
                        IntExp::new(2)
                    ),
                    IntExp::new(0)
                ),
                IntExp::new(2)
            );
            assert_eq!(exp.pretty(), "(((1+2)*0)+2)")
        }

        #[test]
        // TODO: implement cpp parser + pretty print integration tests
        // NOTE: integration tests *outside* ast module using only public API
        fn smart_pretty_print() {
            let exp = PlusExp::new_static(
                MultExp::new_static(
                    PlusExp::new_static(
                        IntExp::new(1),
                        IntExp::new(2)
                    ),
                    IntExp::new(0)
                ),
                IntExp::new(2)
            );
            assert_eq!(exp.smart_pretty(false), "(1+2)*0+2")
        }
    }
}
