#include <iostream>
#include <string>

#include "ast.h"
#include "vm.h"

int IntExp::eval() { return val; }

std::string IntExp::pretty() { return std::to_string(val); }

std::vector<Code>& IntExp::toCode(std::vector<Code>& code) {
    code.push_back(newPush(val));
    return code;
}

int PlusExp::eval() { return e1->eval() + e2->eval(); }

std::string PlusExp::pretty() {
    std::string s("(");
    s.append(e1->pretty());
    s.append("+");
    s.append(e2->pretty());
    s.append(")");
    return s;
}

std::vector<Code>& PlusExp::toCode(std::vector<Code>& code) {
    e1->toCode(code);
    e2->toCode(code);
    code.push_back(newPlus());
    return code; // TODO: void return instead?
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

std::vector<Code>& MultExp::toCode(std::vector<Code>& code) {
    e1->toCode(code);
    e2->toCode(code);
    code.push_back(newMult());
    return code;
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