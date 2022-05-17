<!-- LTeX: language=de-DE -->

Projektarbeit: Mini-Compiler in Rust
====================================

# (WIP)

// Aufgabe: mini-compiler C++11 projekt in Rust portieren

// Basis: [Parser/Interpreter/Compiler für arithmetische Ausdrücke](https://sulzmann.github.io/SoftwareProjekt/schein-neu.html#(5))

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

### Projekt-Setup
- C++: Makefile schreiben
    - sehr DIY
- Rust: `cargo new --lib <name>`
    - cargo kümmert sich um alles
        - package manager: packages ("crates") automatisch heruntergeladen (vgl. npm, pip)
        - build
            - projekt-struktur implizit durch ordner-struktur definieren (vgl. Makefile/cmake wo jede datei manuell aufgelistet werden muss)
        - tests: mehr dazu unten

### Testen
- C++: selbstgemachte test-util. inflexibel. alternativ: externe Bibliothek
- Rust: first-class support für unit und integration tests durch cargo
    - unit tests als private submodul
    - integration tests in `tests/`

### AST
- C++: Klassenhierarchie. Abstrakte Basis-Klasse `Exp`. Polymorphie benötigt
- Rust: `Exp` als Trait. Ähnlich wie Java Interface oder C++20 Concepts aber flexibler
    - NOTE: Separation von Daten und Implementierung. Interface zu Java-Klasse hinzufügen muss Klassendefinition ändern. Traits können sogar externe Typen erweitern, keine Vererbung nötig (ist sogar nicht möglich in Rust)
    - Alternativ könnte Enums verwendet werden. Könnte sogar performanter sein (Box<dyn> pointer indirection vermieden). Enums werden aber für den Tokenizer verwendet, also werden hier Traits demonstriert

### Tokenizer
- Rust
    - pattern matching
    - Enum: eigentlich eine tagged union (werte können unterschiedliche Typen sein + enum weiß welcher Typ enthalten ist)

### Parser
- Exp dynamic (heap) vs static (stack): Rust variable lifetime specifier

### Compiler

### Virtuelle Maschine

### Fehler-Behandlung
- C++: Exceptions oder spezielle Rückgabewerte (hier wird Optional verwendet)
- Rust:
    - `Result<T, Err>`. Enum von Rückgabewert T und Fehlerwert Err. bsp `Result<Exp, String>`
    - `Result`s sind für behebbare Fehler. Err muss explizit behandelt werden (`match` oder `if let`)
        - vgl. Exceptions: Programmfluss wird unterbrochen durch Sprung zum `catch`-Block