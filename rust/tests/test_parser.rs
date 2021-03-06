use minicompiler::parser::Parser;

fn pp(input: &str) -> String {
    Parser::new(input).parse().unwrap().smart_pretty(false)
}

#[test]
fn parse_pretty_spaces() {
    assert_eq!(pp("1"), "1");
    assert_eq!(pp("1 + 0"), "1+0");
    assert_eq!(pp("1 + (0)"), "1+0");
    assert_eq!(pp("1 + 2 * 0"), "1+2*0");
    assert_eq!(pp("1 * 2 + 0"), "1*2+0");
    assert_eq!(pp("(1+ 2) * 0"), "(1+2)*0");
    assert_eq!(pp("(1 + 2) *0  +2"), "(1+2)*0+2");
}

#[test]
fn parse_pretty_parens() {
    assert_eq!(pp("2*((0*1)*2) + (2*2)*1 + 0 + 0*0"), "2*0*1*2+2*2*1+0+0*0");
    assert_eq!(pp("( 1 + 1 )"), "1+1");
    assert_eq!(pp("(2) + (2*2+0) * (2)+2+ (2*2)*1"), "2+(2*2+0)*2+2+2*2*1");
    assert_eq!(pp("(2*1*2) + (2) + 2 * (2+1)*(0*1+0)"), "2*1*2+2+2*(2+1)*(0*1+0)");
    assert_eq!(pp("(((((2)))))"), "2");
    assert_eq!(pp("((((((((((2)*2)*2)*2)*2)*2)*2)*2)*2)*2)*2"), "2*2*2*2*2*2*2*2*2*2*2");
    assert_eq!(pp("((((2))+1))*((1))"), "(2+1)*1");
    assert_eq!(pp("(((((1))+(2))*((0*2)+(2*2+1))))"), "(1+2)*(0*2+2*2+1)");
}

#[test]
#[should_panic]
fn parse_pretty_invalid_panic() {
    pp("+ 1 1 2 3 5 8");
}

#[test]
fn parse_invalid_err() {
    assert!(Parser::new("+ 1 1 2 3 5 8").parse().is_err());
    assert!(Parser::new(") (1+2)").parse().is_err());
    assert!(Parser::new("( 1+1 ").parse().is_err());
    assert!(Parser::new("").parse().is_err());
}

#[test]
fn parse_err_messages() {
    assert_eq!(Parser::new("").parse().err(), Some("Unexpected token 'EOS' at column 0".to_string()));
    assert_eq!(Parser::new("(1+2) * )").parse().err(), Some("Unexpected token ')' at column 9".to_string()));
    assert_eq!(Parser::new("(1+1").parse().err(), Some("Unclosed parenthesis at column 1".to_string()));
}