// Stack-based VM

#ifndef __VM__
#define __VM__

#include <string>
#include <vector>
#include <stack>

#include "utility.h"


/*
Stack-based VM, instructions supported are:
    Push i
    Plus
    Mult
 */


typedef enum {
    OP_PUSH,
    OP_PLUS,
    OP_MULT

} OpCode_t;


class Code {
public:
    OpCode_t kind;
    int val;

    // Nullary VM code (PLUS, MULT)
    Code(OpCode_t o) : kind(o) {}
    // Unary VM code (Push i)
    Code(OpCode_t o, int i) : kind(o), val(i) {}

    std::string toString() const;
};

// Short-hands

Code newPush(int i);

Code newPlus();

Code newMult();

class VM {
private:
    std::vector<Code> code;
    std::stack<int> s;

public:
    VM(std::vector<Code> c) : code(c) {}

    Optional<int> run();
};

#endif // __VM__
