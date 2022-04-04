#include <iostream>
#include <string>

#include "ast.h"

int IntExp::eval() { return val; }

std::string IntExp::pretty() { return std::to_string(val); }

int PlusExp::eval() { return e1->eval() + e2->eval(); }

std::string PlusExp::pretty() {
    std::string s("(");
    s.append(e1->pretty());
    s.append("+");
    s.append(e2->pretty());
    s.append(")");
    return s;
}

int MultExp::eval() { return e1->eval() * e2->eval(); }

std::string MultExp::pretty() {
    std::string s("(");
    s.append(e1->pretty());
    s.append("*");
    s.append(e2->pretty());
    s.append(")");
    return s;
}

EXP newInt(int i) { return std::make_shared<IntExp>(i); }

EXP newPlus(EXP l, EXP r) { return std::make_shared<PlusExp>(l, r); }

EXP newMult(EXP l, EXP r) { return std::make_shared<MultExp>(l, r); }

// Idee: Klammern sind nur noetig, wenn wir Add vor Mult ausfuehren wollen.
// Also wir Klammern PlusExp die Kinder von MultExp sind (isSubexpression flag).

std::string IntExp::smartPretty(bool) { return pretty(); }

std::string PlusExp::smartPretty(bool isSubexpression) {
    std::string s = e1->smartPretty() + "+" + e2->smartPretty();
    if (isSubexpression) {
        s = "(" + s + ")";
    }
    return s;
}

std::string MultExp::smartPretty(bool isSubexpression) {
    std::string s = e1->smartPretty(true) + "*" + e2->smartPretty(true);
    return s;
}