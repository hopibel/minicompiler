<!-- LTeX: language=de-DE -->

Projektarbeit: Mini-Compiler in Rust
====================================

Aufgabe: Mini-Compiler C++11 Projekt in Rust portieren

Dieses Projekt dokumentiert den Prozess des Portierens eines Parsers, eines Compilers und einer virtuellen Maschine für einfache arithmetische Ausdrücke von C++ nach Rust.

Code: [https://github.com/hopibel/minicompiler](https://github.com/hopibel/minicompiler)

Basiert auf: [Parser/Interpreter/Compiler für arithmetische Ausdrücke](https://sulzmann.github.io/SoftwareProjekt/schein-neu.html#(5))

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

### Projekt-Setup

Das C++ Projekt wird mithilfe eines Makefiles kompiliert, worin die Compiler-Argumente definiert werden. Der Nachteil ist, dass Makefiles oft manuell geschrieben werden und ein reines Build-Tool ohne Unterstützung für die Verwaltung von Dependencies sind.

Cargo ist das offizielle Bau-System und Paketmanager von Rust. Cargo kümmert sich um alles, vom Herunterladen externer Dependencies ("crates") bis zum Build-Prozess. Mit `cargo new --bin/--lib <name>` kann man sofort ein minimales, erstellbares Projekt erstellen. Die Projektstruktur wird durch die Verzeichnisstruktur und die Modulimporte definiert, anstatt durch eine manuell gepflegte Konfigurationsdatei. Standardmäßig erzeugt Cargo die Datei `src/main.rs` für ausführbare Programme und `src/lib.rs` für Bibliotheken.

### AST

Der Parser konvertiert Code in einen Baum von Ausdrücken. Jeder interne Knoten repräsentiert eine Operation (Addition oder Multiplikation) und seine Kinder sind die Argumente der Operation, die entweder Literale oder ein anderer Ausdruck sein können.

In C++ können wir den AST als Baum von `Exp`-Objekten darstellen, wobei wir für jeden Ausdruckstyp eine Subklasse erzeugen. Wegen Polymorphismus verwenden wir `shared_ptr<Exp>` als Typ für den AST.

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

Rust hat weder Klassen noch Vererbung. Stattdessen hat es Traits als Abstraktionsmechanismus. Traits definieren, welche Funktionalitäten (Methoden) ein bestimmter Typ haben soll. Traits ähneln Interfaces in anderen Sprachen wie Java, weisen jedoch Unterschiede auf, z.B. die Möglichkeit, Standardimplementierungen zu definieren.

Für den AST definieren wir ein `Exp`-Trait mit den gewünschten Methoden und implementieren das Trait für jeden Ausdruckstyp. Knoten in der Baumstruktur haben den Typ `Box<dyn Exp>`, was ungefähr dem `std::unique_ptr<Exp>` in C++ entspricht. In beiden Sprachen verwenden wir einen Pointer, da die Größe des Typs zur Kompilierzeit bekannt sein muss. Hier bedeutet `dyn Exp`, dass der `Box` auf einen Wert verweist, dessen Typ das `Exp`-Trait implementiert.

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

Eine alternative Implementierung verwendet Enums statt Traits. In Rust kann jede Enum-Variante zusätzliche Daten enthalten. Dieser Datentyp wird in anderen Sprachen normalerweise als Tagged Union bezeichnet. Statt für jeden Ausdrucktyp einzeln Methoden zu implementieren, würde es hier nur eine Implementierung für das gesamte Enum geben, und das korrekte Verhalten würde durch eine Fallunterscheidung auf die Enum-Variante mit `match` ausgewählt (ähnlich wie `switch` in C++).

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

Der Tokenizer liest Code und erzeugt einen Strom von Tokens für den Parser. In C++ verwenden wir einen Enum für die Tokens und einen `switch`, um Symbole in die entsprechenden Tokens zu übersetzen.

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

Die Rust-Implementierung funktioniert fast genauso mit einer `match`-Anweisung. In dieser Implementierung werden Symbole auch in Enum-Werte übersetzt. Der Wert wird in einem optionalen Typ `Option<Token>` eingewickelt, wobei ein Wert von `None` ein ungültiges Symbol darstellt.

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

Der Parser liest den Strom von Tokens und erzeugt einen AST. Als rekursiver Abstiegsparser wird der AST durch rekursive Anwendung der Erweiterungsregeln der Grammatik erzeugt, die als einzelne Funktionen implementiert sind. In C++ bündeln wir die Grammatikregeln in einer `Parser`-Klasse, die einen Tokenizer enthält, um auf den Tokenstrom zugreifen zu können. Jede Regel gibt ein optionaler Typ zurück, das den geparsten AST enthält, oder `null` bei einem Syntaxfehler.

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

Die Rust-Implementierung für den Parser ist mehr oder weniger identisch. Der Hauptunterschied besteht darin, dass sie ein `Result<T, Err>` zurückgibt, das entweder den normalen Rückgabewert oder eine Fehlermeldung enthalten kann. Hier geben wir die AST als `Box<dyn Exp>` oder einen String mit einer ausführlichen Fehlermeldung zurück. Dies könnte in der C++-Implementierung durch Rückgabe eines benutzerdefinierten Ergebnisobjekts erreicht werden, aber Rust macht es durch Bereitstellung eines Standard-Typs bequemer. Exceptions sind eine andere Alternative, aber nicht immer geeignet, weil sie den Programmfluss unterbrechen und einen größeren Leistungsimpact haben können.

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

### Virtuelle Maschine

Die virtuelle Maschine führt sequentiell die Opcodes eines kompilierten Ausdrucks aus. Anweisungen legen Werte auf den Stack oder nehmen sie vom Stack, und das Ergebnis des Ausdrucks bleibt oben auf dem Stack. Auf den ersten Blick scheint ein Enum die perfekte Wahl für die Opcodes zu sein. Die Anweisung `PUSH` braucht aber ein Argument für den auf den Stack zu legenden Wert. Dies zwingt uns, eine `struct` oder eine Klasse zu verwenden, um den Opcode und ein optionales Argument zu halten.

```cpp
typedef enum {
    OP_PUSH, OP_PLUS, OP_MULT
} OpCode_t;

class Code {
public:
    OpCode_t kind;
    int val; // optional argument
    ...
};

class VM {
private:
    std::vector<Code> code;
    std::stack<int> s;
public:
    VM(std::vector<Code> c) : code(c) {}
    Optional<int> run() {
        s = std::stack<int>(); // reset stack
        for(int i = 0; i < code.size(); i++) {
            switch(code[i].kind) {
            case OP_PUSH:
                s.push(code[i].val);
                break;
            case OP_MULT:
                int right = s.top(); s.pop();
                int left = s.top(); s.pop();
                s.push(left * right);
                break;
            case OP_PLUS: ...
            }
        }
        return s.empty() ? nothing<int>() : just<int>(s.top());
    } // run
};
```

Erinnern Sie sich, dass Enum-Varianten in Rust zusätzliche Werte speichern können. Dies ermöglicht es uns, Argumente zusammen mit ihrem Opcode direkt in das Enum zu speichern. Ansonsten ist die Logik gleich, wobei wir wieder den `Result`-Typ verwenden, um bessere Fehlermeldungen zu erhalten.

```rust
pub enum Code {
    Push(i32), Plus, Mult,
}

pub struct VM {
    code: Vec<Code>,
    stack: Vec<i32>,
}

impl VM {
    ...

    // Result instead of Option to clearly communicate error
    pub fn run(&mut self) -> Result<i32, String> {
        ...
        // enumerate to provide line number in errors
        for code in self.code.iter().enumerate() {
            match code.1 {
                Code::Push(val) => { self.stack.push(*val); }
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
                Code::Mult => { ... }
            }
        }
        ...
    }
}
```

### Compiler

Die Kompilierung übersetzt den AST in eine Reihe von Opcodes für die virtuelle Maschine. Operanden müssen auf den Stack geschoben werden, bevor eine Anweisung ausgeführt wird, was durch Durchlaufen des AST und Übersetzung von Knoten zu Opcodes in Postfix-Reihenfolge erreicht wird. Zum Beispiel ergibt der Ausdruck "1+2" die Opcode-Sequenz `Push (1), Push (2), Plus`.

In C++ und Rust kompilieren wir die linken, rechten und Wurzelknoten des AST rekursiv und fügen die resultierenden Opcode-Sequenzen in dieser gleichen Reihenfolge zusammen, um den finalen kompilierten Ausdruck zu erhalten.

C++:

```cpp
std::vector<Code>& IntExp::toCode(std::vector<Code>& code) {
    code.push_back(newPush(val));
    return code;
}

std::vector<Code>& PlusExp::toCode(std::vector<Code>& code) {
    e1->toCode(code);
    e2->toCode(code);
    code.push_back(newPlus());
    return code;
}
```

Rust:

```rust
impl Exp for IntExp {
    fn compile(&self) -> Vec<vm::Code> {
        vec![vm::Code::Push(self.val)]
    }
}

impl Exp for PlusExp {
    fn compile(&self) -> Vec<vm::Code> {
        let mut code = self.l.compile();
        code.append(&mut self.r.compile());
        code.push(vm::Code::Plus);
        code
    }
}
```

Literatur
---------

- The Rust Programming Language: https://doc.rust-lang.org/book/
    - DE: Die Programmiersprache Rust: https://rust-lang-de.github.io/rustbook-de/
