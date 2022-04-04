// AST for exp


#ifndef __AST__
#define __AST__

#include <iostream>
#include <string>
#include <memory>

class Exp {
public:
  virtual int eval() = 0;
  virtual std::string pretty() = 0;
  virtual std::string smartPretty(bool isSubexpression = false) = 0;
};

class IntExp : public Exp {
  int val;
  public:
  IntExp(int _val) { val = _val; }
  int eval() override;
  std::string pretty() override;
  std::string smartPretty(bool) override;
};

class PlusExp : public Exp {
  std::shared_ptr<Exp> e1;
  std::shared_ptr<Exp> e2;
  public:
  PlusExp(std::shared_ptr<Exp> _e1, std::shared_ptr<Exp> _e2) {
      e1 = _e1; e2 = _e2;
  }
  int eval() override;
  std::string pretty() override;
  std::string smartPretty(bool isSubexpression) override;
};


class MultExp : public Exp {
  std::shared_ptr<Exp> e1;
  std::shared_ptr<Exp> e2;
  public:
  MultExp(std::shared_ptr <Exp> _e1, std::shared_ptr<Exp> _e2) {
      e1 = _e1; e2 = _e2;
  }
  int eval() override;
  std::string pretty() override;
  std::string smartPretty(bool) override;
};

// Short-hands

// typedef std::shared_ptr<Exp> EXP;
using EXP = std::shared_ptr<Exp>;

EXP newInt(int i);

EXP newPlus(EXP l, EXP r);

EXP newMult(EXP l, EXP r);


#endif // __AST__