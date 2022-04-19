#include "parser.h"
#include "ast.h"
#include "tokenizer.h"
#include "utility.h"

// E  ::= T E'
Optional<EXP> Parser::parseE() {

    Optional<EXP> t = parseT();
    if (t.isNothing())
        return t;

    return parseE2(t.fromJust());
}

// E' ::= + T E' |
Optional<EXP> Parser::parseE2(EXP left) {

    if (t.token == PLUS) {
        t.nextToken();

        Optional<EXP> right = parseT();
        if (right.isNothing())
            return right;

        return parseE2(newPlus(left, right.fromJust()));
    }

    return just<EXP>(left);
}

// T  ::= F T'
Optional<EXP> Parser::parseT() {

    Optional<EXP> f = parseF();
    if (f.isNothing())
        return f;

    return parseT2(f.fromJust());
}

// T' ::= * F T' |
Optional<EXP> Parser::parseT2(EXP left) {
    if (t.token == MULT) {
        t.nextToken();

        Optional<EXP> right = parseF();
        if (right.isNothing())
            return right;

        return parseT2(newMult(left, right.fromJust()));
    }

    return just<EXP>(left);
}

// F ::= N | (E)
Optional<EXP> Parser::parseF() {
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
