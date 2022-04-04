// (Top-down) Parser for exp

#ifndef __PARSER__
#define __PARSER__

#include "utility.h"
#include "ast.h"
#include "tokenizer.h"

class Parser {
  Tokenizer t;

  // E  ::= T E'
  Optional<EXP> parseE();  

  // E' ::= + T E' | 
  Optional<EXP> parseE2(EXP left);

  // T  ::= F T'
  Optional<EXP> parseT();

  // T' ::= * F T' |
  Optional<EXP> parseT2(EXP left);

  // F ::= N | (E)
  Optional<EXP> parseF(); 

  
public:
  Parser(string s) : t(Tokenizer(s)) { }

  Optional<EXP> parse() {
    Optional<EXP> e= parseE();

    return e;
  }
};




#endif // __PARSER__
