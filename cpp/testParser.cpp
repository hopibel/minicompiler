#include <iostream>
#include <string>

#include "parser.h"
#include "ast.h"

void display(Optional<EXP> e) {
    if(e.isNothing()) {
        std::cout << "nothing \n";
    } else {
        std::cout << (e.fromJust())->pretty() << "\n";
    }
    return;
}

void testParserGood() {
    /*
    display(Parser("1").parse());

    display(Parser("1 + 0 ").parse());

    display(Parser("1 + (0) ").parse());

    display(Parser("1 + 2 * 0 ").parse());

    display(Parser("1 * 2 + 0 ").parse());
    */

    display(Parser("(1 + 2) * 0 ").parse());

    display(Parser("(1 + 2) * 0 + 2").parse());
}

void testParser() {
    testParserGood();
}

bool expEqual(Optional<EXP> a, Optional<EXP> b) {
    if (a.isJust() && b.isJust()) {
        return a.fromJust()->eval() == b.fromJust()->eval();
    } else {
        return a.isNothing() && b.isNothing();
    }
}

bool testCase(std::string input, std::string expected) {
    std::cout << "Test:     " << input;

    Optional<EXP> ast = Parser(input).parse();
    std::string naive, clever;
    if (ast.isNothing()) {
        naive = clever = "nothing";
    } else {
        naive = ast.fromJust()->pretty();
        clever = ast.fromJust()->smartPretty();
    }

    std::cout << "\nNaive:    " << naive;
    std::cout << "\nClever:   " << clever;
    std::cout << "\nExpected: " << expected;

    std::cout << "\neval(Naive):  ";
    auto naive_ast = Parser(naive).parse();
    std::cout << (naive_ast.isJust() ? std::to_string(naive_ast.fromJust()->eval()) : "nothing");

    std::cout << "\neval(Clever): ";
    auto clever_ast = Parser(clever).parse();
    std::cout << (clever_ast.isJust() ? std::to_string(clever_ast.fromJust()->eval()) : "nothing");

    std::cout << "\nResult: ";
    if ((clever == expected) && expEqual(naive_ast, clever_ast)) {
        std::cout << "PASS\n\n";
        return true;
    } else {
        std::cout << "FAIL";
        if (expected == "FAIL") {
            std::cout << " (expected)\n\n";
            return true;
        }
        std::cout << "\n\n";
        return false;
    }
}

void testPrettyPrint() {
    int tests = 0;
    int pass = 0;

    tests++; pass += testCase("1", "1");
    tests++; pass += testCase("1+  1+   1+1+ 1 +1 + 1+  1", "1+1+1+1+1+1+1+1");
    tests++; pass += testCase("1 + 0 ", "1+0");
    tests++; pass += testCase("1 + (0) ", "1+0");
    tests++; pass += testCase("1 + 2 * 0 ", "1+2*0");
    tests++; pass += testCase("1 * 2 + 0 ", "1*2+0");
    tests++; pass += testCase("(1 + 2) * 0 ", "(1+2)*0");
    tests++; pass += testCase("(1 + 2) * 0 + 2", "(1+2)*0+2");

    tests++; pass += testCase("2*((0*1)*2) + (2*2)*1 + 0 + 0*0", "2*0*1*2+2*2*1+0+0*0");
    tests++; pass += testCase("( 1 + 1 )", "1+1");
    tests++; pass += testCase("(2) + (2*2+0) * (2)+2+ (2*2)*1", "2+(2*2+0)*2+2+2*2*1");
    tests++; pass += testCase("(2*1*2) + (2) + 2 * (2+1)*(0*1+0)", "2*1*2+2+2*(2+1)*(0*1+0)");
    tests++; pass += testCase("(((((2)))))", "2");
    tests++; pass += testCase("((((((((((2)*2)*2)*2)*2)*2)*2)*2)*2)*2)*2", "2*2*2*2*2*2*2*2*2*2*2");
    tests++; pass += testCase("((((2))+1))*((1))", "(2+1)*1");
    tests++; pass += testCase("(((((1))+(2))*((0*2)+(2*2+1))))", "(1+2)*(0*2+2*2+1)");

    tests++; pass += testCase("+ 1 1 2 3 5 8", "nothing");
    tests++; pass += testCase(") (1+2)", "nothing");
    tests++; pass += testCase("", "nothing");

    std::cout << "Test summary: " << pass << "/" << tests << " passed" << std::endl;
}

int main() {
    // testParser();
    testPrettyPrint();
    
    return 0;
}
