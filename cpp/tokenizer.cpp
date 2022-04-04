#include <iostream>
#include <string>
#include <vector>

using namespace std;

#include "tokenizer.h"


string showTok(Token_t t) {
  switch(t) {
  case EOS:   return "EOS";
  case ZERO:  return "ZERO";    
  case ONE:   return "ONE";
  case TWO:   return "TWO";      
  case OPEN:  return "OPEN";
  case CLOSE: return "CLOSE";
  case PLUS:  return "PLUS";
  case MULT:  return "MULT";
  }
  // NOTE: The (clang) compiler is able to figure out that
  // along all control-flow paths, a return statement will be reached.
}

Token_t Tokenize::next() {
    if(s.length() <= pos)
      return EOS;

    while(1) {

       if(s.length() <= pos)
         return EOS;
      
       switch(s[pos]) {
       case '0': pos++;
                 return ZERO;        
       case '1': pos++;
                 return ONE;
       case '2': pos++;
                 return TWO;
       case '(': pos++;
                 return OPEN;
       case ')': pos++;
                 return CLOSE;
       case '+': pos++;
                 return PLUS;
       case '*': pos++;
                 return MULT;         
       default:  // we simply skip all other symbols !
                 pos++;
             break;
       }
    }
} // next


vector<Token_t> Tokenize::scan() {
    vector<Token_t> v;
    Token_t t;

    do {
      t = next();
      v.push_back(t);
    }
    while(t != EOS);

    return v;
} // scan


string Tokenize::show() {
    vector<Token_t> v = this->scan();
    string s;

    for(int i=0; i < v.size(); i++) {
      s += showTok(v[i]);
      if(i+1 < v.size())
    s += ";" ;         // Add delimiter
    }
    return s;
} // show
