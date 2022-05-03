use minicompiler::parser::Parser;
use minicompiler::vm::{Code, VM};

// (vm_result, ast_result)
fn eval_both(program: &str) -> (i32, i32) {
    let ast = Parser::new(program).parse().unwrap();
    let ast_result = ast.eval();
    let vm_result = VM::new(ast.compile()).run().unwrap();

    (vm_result, ast_result)
}

#[test]
fn precedence() {
    let (mut vm_result, mut ast_result) = eval_both("1 + 2 * (2+1)");
    assert_eq!(vm_result, ast_result);
    assert_eq!(vm_result, 1 + 2 * (2 + 1)); // 7

    (vm_result, ast_result) = eval_both("2 * (2 + 1)");
    assert_eq!(vm_result, ast_result);
    assert_eq!(vm_result, 2 * (2 + 1)); // 6

    (vm_result, ast_result) = eval_both("1 + 2 * 0 ");
    assert_eq!(vm_result, ast_result);
    assert_eq!(vm_result, 1 + 2 * 0); // 1

    (vm_result, ast_result) = eval_both("1 * 2 + 0 ");
    assert_eq!(vm_result, ast_result);
    assert_eq!(vm_result, 1 * 2 + 0); // 2
}

#[test]
fn constant() {
    let (vm_result, ast_result) = eval_both("1");
    assert_eq!(vm_result, ast_result);
    assert_eq!(vm_result, 1);
}

#[test]
fn long_expr() {
    let (mut vm_result, mut ast_result) = eval_both("1+  1+   1+1+ 1 +1 + 1+  1");
    assert_eq!(vm_result, ast_result);
    assert_eq!(vm_result, 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1); // 8

    (vm_result, ast_result) =
        eval_both("2*2*2*2*2*2*2*2*2*2*2*2*2*2*2*2*2*2*2*2*2*2*2*2*2*2*2*2*2*2");
    assert_eq!(vm_result, ast_result);
    assert_eq!(vm_result, 1 << 30); // 2^30 = 1073741824
}

#[test]
fn complicated_expr() {
    let (mut vm_result, mut ast_result) = eval_both("2*((2*1)*2) + (2*2)*1 + 0 + 0*0");
    assert_eq!(vm_result, ast_result);
    assert_eq!(vm_result, 2 * ((2 * 1) * 2) + (2 * 2) * 1 + 0 + 0 * 0); // 12

    (vm_result, ast_result) = eval_both("(2) + (2*2+0) * (2)+2+ (2*2)*1");
    assert_eq!(vm_result, ast_result);
    assert_eq!(vm_result, (2) + (2 * 2 + 0) * (2) + 2 + (2 * 2) * 1); // 16

    (vm_result, ast_result) = eval_both("(2*1*2) + (2) + 2 * (2+1)*(0*1+1)");
    assert_eq!(vm_result, ast_result);
    assert_eq!(vm_result, (2 * 1 * 2) + (2) + 2 * (2 + 1) * (0 * 1 + 1)); // 12

    (vm_result, ast_result) = eval_both("(1+2)*(0*2+2*2+1)");
    assert_eq!(vm_result, ast_result);
    assert_eq!(vm_result, (1 + 2) * (0 * 2 + 2 * 2 + 1)); // 15

    (vm_result, ast_result) = eval_both("((2+2) + (2*2+1)) * (1+1+2+1)");
    assert_eq!(vm_result, ast_result);
    assert_eq!(vm_result, ((2 + 2) + (2 * 2 + 1)) * (1 + 1 + 2 + 1)); // 45
}

#[test]
fn malformed() {
    let mut code = vec![Code::Plus, Code::Push(1), Code::Push(2)];
    assert!(VM::new(code).run().is_err());

    code = vec![Code::Push(1), Code::Push(2), Code::Plus, Code::Plus];
    assert!(VM::new(code).run().is_err());

    code = Vec::new();
    assert!(VM::new(code).run().is_err());
}
