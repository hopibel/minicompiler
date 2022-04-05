#include <iostream>
#include <string>

#include "parser.h"
#include "vm.h"

void showVMRes(Optional<int> r) {
    if (r.isNothing()) {
        std::cout << "\nVM stack (top): empty";
    }

    std::cout << "\nVM stack (top):" << r.fromJust();
}

// Opcode Sequenz als menschenlesbarer string
std::string code_string(const std::vector<Code>& code) {
    std::string s = "";
    auto it = code.cbegin();
    while (it != code.cend()) {
        s += it->toString() + " ";
        ++it;
    }
    // no delimiter after last instruction
    if (code.size() > 0) {
        s += it->toString();
    }
    return s;
}

bool testCase(std::string program, std::string expected) {
    // default empty eval output
    std::string ast_out("nothing");
    std::string code_out("nothing");

    std::vector<Code> code;
    Optional<EXP> ast = Parser(program).parse();

    // AST interpretieren, in Opcodes umwandeln und mit VM ausführen
    if (ast.isJust()) {
        EXP expr = ast.fromJust();
        ast_out = std::to_string(expr->eval());
        code = expr->toCode(code);
        Optional<int> result = VM(code).run();
        if (result.isJust()) {
            code_out = std::to_string(result.fromJust());
        }
    }

    std::cout << "Test: " << program;
    std::cout << "\nCode: " << code_string(code);
    std::cout << "\nAST eval result: " << ast_out;
    std::cout << "\nVM eval result:  " << code_out;
    std::cout << "\nResult: ";
    if ((code_out == ast_out) && (code_out == expected)) {
        std::cout << "PASS\n\n";
        return true;
    } else {
        std::cout << "FAIL\n\n";
        return false;
    }
}

void testVM() {
    /*
    {
        std::vector<Code> vc{
            newPush(1),
            newPush(2),
            newPush(3),
            newMult(),
            newPlus()
        };

        Optional<int> res = VM(vc).run();

        showVMRes(res);
    }

    {
        std::vector<Code> vc{
            newPush(2),
            newPush(3),
            newPush(5),
            newPlus(),
            newMult()
        };

        Optional<int> res = VM(vc).run();

        showVMRes(res);
    }
    */

    int tests = 0;
    int pass = 0;

    tests++; pass += testCase("1 + 2 * (2+1)", "7");
    tests++; pass += testCase("2 * (2 + 1)", "6");

    tests++; pass += testCase("1", "1");
    tests++; pass += testCase("1+  1+   1+1+ 1 +1 + 1+  1", "8");
    tests++; pass += testCase("1 + 0 ", "1");
    tests++; pass += testCase("1 + 2 * 0 ", "1");
    tests++; pass += testCase("1 * 2 + 0 ", "2");
    tests++; pass += testCase("(1 + 2) * 0 ", "0");
    tests++; pass += testCase("(1 + 2) * 0 + 2", "2");

    tests++; pass += testCase("2*((2*1)*2) + (2*2)*1 + 0 + 0*0", "12");
    tests++; pass += testCase("(2) + (2*2+0) * (2)+2+ (2*2)*1", "16");
    tests++; pass += testCase("(2*1*2) + (2) + 2 * (2+1)*(0*1+1)", "12");
    tests++; pass += testCase("2*2*2*2*2*2*2*2*2*2*2", "2048");
    tests++; pass += testCase("(1+2)*(0*2+2*2+1)", "15");
    tests++; pass += testCase("((2+2) + (2*2+1)) * (1+1+2+1)", "45");

    tests++; pass += testCase("+ 1 1 2 3 5 8", "nothing");
    tests++; pass += testCase(") (1+2)", "nothing");
    tests++; pass += testCase("", "nothing");

    std::cout << "Test summary: " << pass << "/" << tests << " passed" << std::endl;
}


int main() {
    testVM();

    return 1;
}