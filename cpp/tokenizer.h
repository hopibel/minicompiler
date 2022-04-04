// Tokenizer for exp


#ifndef __TOKENIZER__
#define __TOKENIZER__

#include <iostream>
#include <string>
#include <vector>

using namespace std;

typedef enum {
  EOS,           // End of string
  ZERO,
  ONE,
  TWO,
  OPEN,
  CLOSE,
  PLUS,
  MULT
} Token_t;

string showTok(Token_t t);

// Elementary tokenize(r) class
class Tokenize {
  string s;
  int pos;
public:
  Tokenize(string s) {
         this->s = s;
     pos = 0;
  }

    // Scan throuh string, letter (symbol) by letter.
  Token_t next();
  vector<Token_t> scan();
  string show();  

};


// Wrapper class, provide the (current) token.
class Tokenizer : Tokenize {
public:
  Token_t token;
  Tokenizer(string s) : Tokenize(s) { token = next(); }
  void nextToken() {
    token = next();
  }
};

#endif // __TOKENIZER__
