#include "utility.h"
#include "vm.h"


Code newPush(int i) {
  return Code(PUSH, i);
}

Code newPlus() {
  return Code(PLUS);
}

Code newMult() {
  return Code(MULT);
}


Optional<int> VM::run() {
  // always start with an empty stack
  stack<int> d;
  s.swap(d);

  for(int i = 0; i < code.size(); i++) {
    switch(code[i].kind) {
    case PUSH:
      s.push(code[i].val);
      break;
    case MULT: {
      int right = s.top();
      s.pop();
      int left = s.top();
      s.pop();
      s.push(left * right);
      break;
    }
    case PLUS: {
      int right = s.top();
      s.pop();
      int left = s.top();
      s.pop();
      s.push(left + right);
      break;
    }
    }
  }

  if(s.empty())
    return nothing<int>();

  return just<int>(s.top());
} // run