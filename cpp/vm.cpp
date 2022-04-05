#include "utility.h"
#include "vm.h"


Code newPush(int i) {
    return Code(OP_PUSH, i);
}

Code newPlus() {
    return Code(OP_PLUS);
}

Code newMult() {
    return Code(OP_MULT);
}


Optional<int> VM::run() {
    // always start with an empty stack
    std::stack<int> d;
    s.swap(d);

    for(int i = 0; i < code.size(); i++) {
        switch(code[i].kind) {
        case OP_PUSH:
            s.push(code[i].val);
            break;
        case OP_MULT: {
            int right = s.top();
            s.pop();
            int left = s.top();
            s.pop();
            s.push(left * right);
            break;
        }
        case OP_PLUS: {
            int right = s.top();
            s.pop();
            int left = s.top();
            s.pop();
            s.push(left + right);
            break;
        }
        }
    }

    if (s.empty()) {
        return nothing<int>();
    }

    return just<int>(s.top());
} // run

// menschenlesbare Darstellung von VM-Instruktion
std::string Code::toString() const {
    switch (kind) {
    case OP_PUSH:
        return "Push " + std::to_string(val) + ";";
    case OP_PLUS:
        return "Plus;";
    case OP_MULT:
        return "Mult;";
    default:
        return "UNK;";
    }
}