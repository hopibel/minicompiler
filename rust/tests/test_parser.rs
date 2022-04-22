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
fn parse_pretty_invalid_none() {
    assert!(Parser::new("+ 1 1 2 3 5 8").parse().is_none());
    assert!(Parser::new(") (1+2)").parse().is_none());
    assert!(Parser::new("").parse().is_none());
}