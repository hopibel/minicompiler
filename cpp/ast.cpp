#include <iostream>
#include <string>

using namespace std;

#include "ast.h"

int IntExp::eval() { return val;}

string IntExp::pretty() {
      return to_string(val);
}

int PlusExp::eval() { return e1->eval() + e2->eval(); }

string PlusExp::pretty() {
    string s("(");
    s.append(e1->pretty());
    s.append("+");
    s.append(e2->pretty());
    s.append(")");
    return s;
}


int MultExp::eval() { return e1->eval() * e2->eval(); }

string MultExp::pretty() {
    string s("(");
    s.append(e1->pretty());
    s.append("*");
    s.append(e2->pretty());
    s.append(")");
    return s;
  }

EXP newInt(int i) {
  return std::make_shared<IntExp>(i);
}

EXP newPlus(EXP l, EXP r) {
  return std::make_shared<PlusExp>(l,r);
}

EXP newMult(EXP l, EXP r) {
  return std::make_shared<MultExp>(l,r);
}