<!-- LTeX: language=de-DE -->

Projektarbeit: Mini-Compiler in Rust
====================================

# (WIP)

// Aufgabe: mini-compiler C++11 projekt in Rust portieren

// Basis: [Parser/Interpreter/Compiler für arithmetische Ausdrücke](https://sulzmann.github.io/SoftwareProjekt/schein-neu.html#(5))

TODO: what does the project do

// expressions are tokenized and parsed into an abstract syntax tree which can be interpreted directly or compiled into opcodes for the included stack-based virtual machine

Usage
-----

### C++
```bash
cd cpp

# Build
## Windows
make
## Linux (ungetestet: MacOS)
make -f Makefile.linux

# Run tests
./testParser
./testVM

# Remove build files
## Windows
make clean
## Linux (ungetestet: MacOS)
make -f Makefile.linux clean
```

### Rust
```bash
cd rust

# Automatically build and run tests
cargo test

# Remove build files
cargo clean
```

Portierung von C++11 nach Rust
------------------------------

- NOTE: keep short. mainly use concrete examples for translation from C++ to Rust

### Projekt-Setup
TODO
- C++: Makefile schreiben
    - Manuell definieren, wie Programme kompiliert werden sollen
    - Reines Build-Tool.
- Rust: `cargo new --lib <name>`
    - Cargo: offizielles Bau-System und Paketmanager von Rust
    - cargo kümmert sich um alles
        - package manager: externe dependencies ("crates") automatisch heruntergeladen (vgl. npm, pip)
        - build
            - projekt-struktur durch Ordner-Hierarchie und Imports definieren (vgl. Makefile/cmake wo jede datei eingetragen wird)
                - `src/main.rs`: default executable file, weitere in `src/bin/`
                - `src/lib.rs`: default library file
        - tests: mehr dazu unten

### Testen
TODO
- C++: selbstgemachte test-util. inflexibel. alternativ: externe Bibliothek
- Rust: first-class support für unit und integration tests durch cargo
    - unit tests als private submodule: einzelnes Modul testen. innerhalb Modul können private Schnittstellen getestet werden
    - integration tests in `tests/`: tests laufen ausserhalb Modul, verwenden nur öffentliche Schnittstellen

### AST
TODO: translate

The parser converts code into a tree of expressions. Each internal node represents an operation (addition or multiplication) and its children are the operation's arguments, which can either be terminals representing integer literals or another expression.

In C++, we can represent the AST as a tree of `Exp` objects, with subclasses for each type of expression. Due to polymorphism, the type of our AST is `shared_ptr<Exp>`.

```cpp
class Exp {
public:
    virtual int eval() = 0;
    ...
};

class IntExp : public Exp {
    int val;
    ...
};

class PlusExp : public Exp {
    std::shared_ptr<Exp> e1; // left argument
    std::shared_ptr<Exp> e2; // right argument
    ...
};


class MultExp : public Exp {
    std::shared_ptr<Exp> e1; // left argument
    std::shared_ptr<Exp> e2; // right argument
    ...
};
```

Rust does not have classes or inheritance. Instead, the main abstraction mechanism is Traits, which define what behaviors (methods) a particular type has. Traits are similar to interfaces in other languages but with some differences such as being able to provide default implementations.

For the AST we define an `Exp` trait with the desired methods and implement the trait for each expression type. Child nodes in the tree have the type `Box<dyn Exp>`, which is roughly equivalent to a `std::unique_ptr<Exp>` in C++. In both languages we use a pointer because the type's size must be known at compile time. Here, `dyn Exp` signifies that the `Box` points to some value whose type implements the `Exp` trait.

```rust
pub trait Exp {
    fn eval(&self) -> i32;
    ...
}

pub struct IntExp {
    val: i32,
}
impl Exp for IntExp { /* Exp method implementations */ }

pub struct PlusExp {
    l: Box<dyn Exp>,
    r: Box<dyn Exp>,
}
impl Exp for PlusExp { /* Exp method implementations */ }

pub struct MultExp {
    l: Box<dyn Exp>,
    r: Box<dyn Exp>,
}
impl Exp for MultExp { /* Exp method implementations */ }
```

An alternative implementation uses Enums instead of Traits. In Rust, each enum variant can hold additional data. This data type is usually called a tagged union in other languages. Instead of implementing methods separately for each type of expression, there would only be one implementation for the whole enum and the correct behavior would be chosen by `match`-ing on the enum variant (similar to `switch` in C++).

```rust
pub enum Exp {
    IntExp(i32),
    PlusExp { l: Box<Exp>, r: Box<Exp>},
    MultExp { l: Box<Exp>, r: Box<Exp>},
}

impl Exp {
    pub fn eval(&self) -> i32 {
        match *self {
            IntExp(i) => { /* Int-specific behavior */ }
            PlusExp(l, r) => { /* Plus-specific behavior */ }
            MultExp(l, r) => { /* Mult-specific behavior */ }
        }
    }
    ...
}
```

### Tokenizer
TODO: translate

The tokenizer reads code and produces a stream of tokens for the parser. In C++ we use an enum for the tokens and a simple switch statement to match characters to their corresponding tokens.

```cpp
typedef enum {
    EOS, ZERO, ONE, TWO, OPEN, CLOSE, PLUS, MULT
} Token_t;

class Tokenize {
    string s;
    int pos;
    ...
}

Token_t Tokenize::next() {
    while(1) {
        if(s.length() <= pos)
            return EOS;

        switch(s[pos++]) {
            case '0': return ZERO;
            case '1': return ONE;
            case '2': return TWO;
            case '(': return OPEN;
            case ')': return CLOSE;
            case '+': return PLUS;
            case '*': return MULT;
            default: break; // we simply skip all other symbols !
        }
    }
}; // next
```

The Rust version works almost exactly the same way using a `match` statement. In this implementation the `match` maps characters to enum values. The value is wrapped in an optional type `Option<Token>`, where a value  of `None` represents an invalid symbol.

```rust
pub enum Token {
    EOS, ZERO, ONE, TWO, OPEN, CLOSE, PLUS, MULT,
}

pub struct Tokenizer {
    pub token: Token,
    s: Vec<char>,
    pos: usize,
}

impl Tokenizer {
    ...
    pub fn next_token(&mut self) {
        loop {
            if self.pos >= self.s.len() {
                self.token = Token::EOS;
                break;
            }
            // match symbol to token
            let token = match self.s[self.pos] {
                '0' => Some(Token::ZERO),
                '1' => Some(Token::ONE),
                '2' => Some(Token::TWO),
                '(' => Some(Token::OPEN),
                ')' => Some(Token::CLOSE),
                '+' => Some(Token::PLUS),
                '*' => Some(Token::MULT),
                _ => None,
            };
            match token {
                Some(token) => { // set current token
                    self.token = token;
                    self.pos += 1;
                    break;
                },
                None => (),  // skip all other symbols
            }
            self.pos += 1;
        }
    }
    ...
}
```

### Parser
TODO: translate

The parser reads the stream of tokens and produces an AST. As a recursive descent parser, the AST is generated by recursively applying the grammar's expansion rules, which are implemented as individual functions. In C++ we bundle the grammar rules in a `Parser` class that holds a tokenizer to provide access to the token stream. Each rule returns an optional containing the parsed AST or null in case of a syntax error.

```cpp
class Parser {
    Tokenizer t;

    // E  ::= T E'
    Optional<EXP> parseE() {
        Optional<EXP> t = parseT();
        if (t.isNothing())
            return t;
        return parseE2(t.fromJust());
    }

    ...

    // F ::= N | (E)
    Optional<EXP> parseF() {
        switch (t.token) {
        case ZERO:
            t.nextToken();
            return just<EXP>(newInt(0));
        case ONE:
            t.nextToken();
            return just<EXP>(newInt(1));
        case TWO:
            t.nextToken();
            return just<EXP>(newInt(2));
        case OPEN: { // introduce new scope
            t.nextToken();
            Optional<EXP> e = parseE();
            if (e.isNothing())
                return e;
            if (t.token != CLOSE)
                return nothing<EXP>();
            t.nextToken();
            return e;
        }
        default:
            return nothing<EXP>();
        }
    }

public:
    Parser(string s) : t(Tokenizer(s)) { }
    Optional<EXP> parse() { return parseE(); }
};
```

The Rust implementation for the parser is more or less identical. The main difference is it returns a `Result<T, Err>`, which can either hold a normal return type or an error type. Here we return the AST as a `Box<dyn Exp>` or a string with a detailed error message. This could be accomplished in the C++ version by returning a custom result object, but Rust makes it more convenient by providing a standard `Result` type. Exceptions are another alternative but aren't always suitable because they interrupt program flow and have a larger performance impact.

```rust
pub struct Parser {
    t: Tokenizer,
}

impl Parser {
    // E ::= T E'
    pub fn parse(&mut self) -> Result<Box<dyn ast::Exp>, String> {
        self.parse_e()
    }

    // NOTE: Propagate errors up the call stack with ?
    fn parse_e(&mut self) -> Result<Box<dyn ast::Exp>, String> {
        let tok = self.parse_t()?;
        self.parse_e2(tok)
    }

    ...

    // F ::= N | (E)
    fn parse_f(&mut self) -> Result<Box<dyn ast::Exp>, String> {
        let pos = self.t.position();
        match self.t.token {
            Token::ZERO => {
                self.t.next_token();
                Ok(Box::new(ast::IntExp::new(0))) },
            Token::ONE => {
                self.t.next_token();
                Ok(Box::new(ast::IntExp::new(1))) },
            Token::TWO => {
                self.t.next_token();
                Ok(Box::new(ast::IntExp::new(2))) },
            Token::OPEN => {
                self.t.next_token();
                let e = self.parse_e()?;
                if self.t.token != Token::CLOSE {
                    Err(format!("Unclosed parenthesis at column {}", pos))
                } else {
                    self.t.next_token();
                    Ok(e)
                }
            },
            _ => Err(format!("Unexpected token '{}' at column {}", self.t.token.show(), pos)),
        }
    }
}
```

### Compiler
TODO

### Virtuelle Maschine
TODO
- C++
    - Opcodes sind *fast* als Enum darstellbar. Problem: PUSH braucht einen `int`, also müssen wir leider eine Klasse verwenden, die Opcode + optionaler int speichern kann
- Rust
    - Enum: eigentlich eine tagged union (Werte können unterschiedliche Typen sein + Enum weiß welcher Typ enthalten ist). Erlaubt uns einfach PUSH zusammen mit i32-Wert zu speichern, ohne eigene `struct` zu definieren

Literatur
---------

- // Rust book: https://doc.rust-lang.org/book/
- // Deutsche Übersetzung: https://rust-lang-de.github.io/rustbook-de/