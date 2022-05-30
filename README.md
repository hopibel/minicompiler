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
- C++: selbstgemachte test-util. inflexibel. alternativ: externe Bibliothek
- Rust: first-class support für unit und integration tests durch cargo
    - unit tests als private submodule: einzelnes Modul testen. innerhalb Modul können private Schnittstellen getestet werden
    - integration tests in `tests/`: tests laufen ausserhalb Modul, verwenden nur öffentliche Schnittstellen

### AST
- C++: Klassenhierarchie. Abstrakte Basis-Klasse `Exp`. Polymorphie benötigt
    - abstract class Exp
        - class IntExp
        - class PlusExp
        - class MultExp
    - Baum von `shared_ptr<Exp>`
- Rust: `Exp` als Trait. Ähnlich wie Java Interface oder C++20 Concepts aber flexibler
    - Baum von `Box<dyn Exp>` (Analog zu C++ impl)
        - `Box`: smart pointer ähnlich wie `unique_ptr`
        - `Box<dyn Exp>` ist ein Trait-Objekt (Rusts Lösung für Polymorphie)
            - `dyn Exp` steht für eine Instanz eines beliebigen Typs, der den Trait Exp implementiert.
            - konkreter Typ erst zur Laufzeit bekannt. Trait-Objekte haben Pointer zu Daten und Pointer zu Lookup-Tabelle für Trait-Methoden (i.e., eine vtable)
            - `dyn` steht für "dynamic" (dispatch)
        - Warum `Box`?
            - Typgrößen müssen zur Kompilierzeit bekannt sein. Da der AST beliebig groß werden kann, verstecken wir Kindknoten hinter Pointern, weil sie konstante Größe haben.
    - NOTE: struct def und Trait impl sind separate Blöcke. vgl Java: Interface hinzufügen ändert die Klassendefinition. Traits können sogar externe Typen erweitern, keine Vererbung nötig (ist sogar nicht möglich in Rust)
    - Alternativ könnte Enums verwendet werden. Könnte sogar performanter sein (Box<dyn Exp> pointer indirection vermieden). Enums werden aber schon für den Tokenizer verwendet, also werden hier Traits demonstriert
    - NOTE: In Rust sind Variablen standardmäßig schreibgeschützt (vgl. C++ explizites `const`) und müssen explizit mit dem Keyword `mut` ("mutable") schreibbar gemacht werden
    - NOTE: Rust Ownership memory management model
        - Rules
            - every value has one and only one variable called its owner
            - the value is dropped when the owner goes out of scope
        - TODO: include code snippet
        - Zuweisung: Wert wird verschoben und die neue Variable ist jetzt sein Eigentümer
            - alte Variable ist jetzt ungültig. weitere Zugriffe geben Kompilierfehler
                - !!! dies erlaubt es Rust, viele Speicherfehler (bsp. Double-Free von Heap-Variablen) zur Kompilierzeit zu erkennen
            - Ausnahme: bei Typen mit `Copy` Trait bleibt originaler Eigentümer gültig nach Zuweisung
            - `Clone` Trait für tiefe Kopien
        - Reference (`&val`): `val` ausleihen, ohne Eigentümer zu werden. Erfordert zu Kompilierzeit dass Lifetime von Referenz nachweisbar kürzer-gleich Lifetime von Wert. Ungültige Referenz ist Kompilierfehler statt Runtime-Fehler

### Tokenizer
- Rust
    - Enum + pattern matching

### Parser
- Exp dynamic (heap) vs static (stack): Rust variable lifetime specifier

### Compiler

### Virtuelle Maschine
- C++
    - Opcodes sind *fast* als Enum darstellbar. Problem: PUSH braucht einen `int`, also müssen wir leider eine Klasse verwenden, die Opcode + optionaler int speichern kann
- Rust
    - Enum: eigentlich eine tagged union (Werte können unterschiedliche Typen sein + Enum weiß welcher Typ enthalten ist). Erlaubt uns einfach PUSH zusammen mit i32-Wert zu speichern, ohne eigene `struct` zu definieren

### Fehler-Behandlung
- C++: Exceptions oder spezielle Rückgabewerte (hier wird Optional verwendet)
- Rust:
    - `Result<T, Err>`. Enum von Rückgabewert T und Fehlerwert Err. bsp `Result<Exp, String>`
    - `Result`s sind für behebbare Fehler. Err muss explizit behandelt werden (`match` oder `if let`)
        - vgl. Exceptions: Programmfluss wird unterbrochen durch Sprung zum `catch`-Block

Literatur
---------

- // Rust book: https://doc.rust-lang.org/book/
- // DE translation: https://rust-lang-de.github.io/rustbook-de/