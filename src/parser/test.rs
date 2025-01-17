// TODO: Figure out how this works
#[cfg(test)]
use crate::{
    scanner::{tokenize, Token},
    Context,
};

use super::Parser;

fn get_parser(src: &str) -> Parser<impl Iterator<Item = Token>> {
    let mut context = Context::new();
    let tokens = tokenize(src, &mut context).into_iter().peekable();
    assert!(context.errors.is_empty());
    Parser::new(tokens)
}

fn utf8_to_string(buffer: &[u8]) -> Vec<&str> {
    std::str::from_utf8(&buffer)
        .expect("comes from a valid string, so it should be a valid string")
        .split('\n')
        .collect()
}

mod expressions {
    use crate::parser::test::get_parser;
    #[test]
    fn complex() {
        let expr_text = "(5+2)*-6 == 9";
        assert_eq!(
            get_parser(expr_text).expression().to_string_normal(),
            "(5 + 2) * -6 == 9"
        )
    }

    #[test]
    fn equalities() {
        let expr_text = "(5==2) == -6 != 9";
        assert_eq!(
            get_parser(expr_text).expression().to_string_normal(),
            "(5 == 2) == -6 != 9"
        )
    }

    #[test]
    fn literal() {
        let expr_text = "\"testing\"";
        assert_eq!(
            get_parser(expr_text).expression().to_string_normal(),
            "testing"
        )
    }
}

mod evaluate {
    use crate::{environment::Environment, literal::Literal, parser::test::get_parser};

    #[test]
    fn equality() {
        let expr_text = "(5+2)*-6 == 9";
        assert_eq!(
            get_parser(expr_text)
                .expression()
                .evaluate(&mut Environment::new()),
            Literal::False
        )
    }
    #[test]
    fn arithemtic() {
        let expr_text = "(5+2)*-6";
        assert_eq!(
            get_parser(expr_text)
                .expression()
                .evaluate(&mut Environment::new()),
            Literal::Number(-42.0)
        )
    }

    #[test]
    fn arithemtic2() {
        let expr_text = "2 - 3 + 2";
        assert_eq!(
            get_parser(expr_text)
                .expression()
                .evaluate(&mut Environment::new()),
            Literal::Number(1.0)
        )
    }

    #[test]
    fn relational() {
        let expr_text = "2 - 3 + 2 < 2";
        assert_eq!(
            get_parser(expr_text)
                .expression()
                .evaluate(&mut Environment::new()),
            Literal::True
        )
    }

    #[test]
    fn concatenation() {
        let expr_text = "\"Hello,\" + \" \" + \"World!\"";
        assert_eq!(
            get_parser(expr_text)
                .expression()
                .evaluate(&mut Environment::new()),
            Literal::String("Hello, World!".into())
        )
    }
}

mod run {
    use crate::{environment::Environment, parser::test::get_parser};

    #[test]
    fn declare() {
        let code = "var x = 5;
                    var x = x + 2;
                    print x;";
        let mut environment = Environment::new();

        let program = get_parser(code).parse();
        let mut buffer = Vec::<u8>::new();
        for statement in program {
            statement.execute(&mut environment, &mut buffer);
        }

        assert_eq!(buffer, b"7\n");
    }

    #[test]
    fn declare2() {
        let code = "var x = 5;
                    var y = x + 2;
                    print x + y;";
        let mut environment = Environment::new();

        let program = get_parser(code).parse();
        let mut buffer = Vec::<u8>::new();
        for statement in program {
            statement.execute(&mut environment, &mut buffer);
        }

        assert_eq!(buffer, b"12\n")
    }

    #[test]
    fn assign() {
        let code = "var x = 5;
                    x = x + 2;
                    print x;";
        let mut environment = Environment::new();

        let program = get_parser(code).parse();
        let mut buffer = Vec::<u8>::new();
        for statement in program {
            statement.execute(&mut environment, &mut buffer);
        }

        assert_eq!(buffer, b"7\n")
    }

    #[test]
    fn complex_assign() {
        let code = "
                    var x = 5; 
                    x = x + 2; // x is 7
                    print x - 2; // prints 5
                    var y = x - 10; // y is -3
                    print y == -3; 
                    print x - y;
                    ";
        let mut environment = Environment::new();

        let program = get_parser(code).parse();
        let mut buffer = Vec::<u8>::new();
        for statement in program {
            statement.execute(&mut environment, &mut buffer);
        }

        assert_eq!(buffer, b"5\ntrue\n10\n")
    }
}
mod should_not_work {

    use crate::{environment::Environment, parser::test::get_parser};

    #[test]
    #[should_panic]
    fn basic() {
        let code = "
                var x = 2
                print x
                x = x+1
                ";
        let mut environment = Environment::new();

        let program = get_parser(code).parse();
        let mut buffer = Vec::<u8>::new();
        for statement in program {
            statement.execute(&mut environment, &mut buffer);
        }
    }

    #[test]
    #[should_panic]
    fn undefined() {
        let code = "
                print(x);
                ";
        let mut environment = Environment::new();

        let program = get_parser(code).parse();
        let mut buffer = Vec::<u8>::new();
        for statement in program {
            statement.execute(&mut environment, &mut buffer);
        }
    }
}
mod block {
    use crate::{
        environment::Environment,
        parser::test::{get_parser, utf8_to_string},
    };

    #[test]
    fn basic() {
        let code = "
{
    var x = 2;
    print x;
}
            ";
        let mut environment = Environment::new();

        let program = get_parser(code).parse();
        let mut buffer = Vec::<u8>::new();
        for statement in program {
            statement.execute(&mut environment, &mut buffer);
        }

        assert_eq!(utf8_to_string(&buffer), vec!["2", ""])
    }

    #[test]
    fn nested() {
        let code = "
            var a = \"global a\";
            var b = \"global b\";
            var c = \"global c\";
            {
                var a = \"outer a\";
                var b = \"outer b\";
                {
                    var a = \"inner a\";
                    print a;
                    print b;
                    print c;
                }
                print a;
                print b;
                print c;
            }
            print a;
            print b;
            print c;
            ";
        let mut environment = Environment::new();

        let program = get_parser(code).parse();
        let mut buffer = Vec::<u8>::new();
        for statement in program {
            statement.execute(&mut environment, &mut buffer);
        }
        let output = vec![
            "inner a", "outer b", "global c", //
            "outer a", "outer b", "global c", //
            "global a", "global b", "global c", //
            "",
        ];
        assert_eq!(utf8_to_string(&buffer), output)
    }

    #[test]
    fn blocks() {
        let code = "
            var i = 1;
            {
                print i;
                i = i + 1;
                print i;
            }
            print i;
            ";
        let mut environment = Environment::new();

        let program = get_parser(code).parse();
        let mut buffer = Vec::<u8>::new();
        for statement in program {
            statement.execute(&mut environment, &mut buffer);
        }
        let output = vec!["1", "2", "2", ""];
        assert_eq!(utf8_to_string(&buffer), output)
    }
}

mod control_flow {
    use crate::{
        environment::Environment,
        parser::test::{get_parser, utf8_to_string},
    };

    #[test]
    fn outputs() {
        let code = "
                print true or true;
                print true or false;
                print false or true;
                print false or false;
                print true and true;
                print true and false;
                print false and true;
                print false and false;
            ";
        let mut environment = Environment::new();

        let program = get_parser(code).parse();
        let mut buffer = Vec::<u8>::new();
        for statement in program {
            statement.execute(&mut environment, &mut buffer);
        }

        let output = vec![
            "true", "true", "true", "false", "true", "false", "false", "false", "",
        ];
        assert_eq!(utf8_to_string(&buffer), output)
    }

    #[test]
    fn short_circuiting() {
        let code = "
                print \"hi\" or 2; 
                print nil or \"yes\"; 
            ";
        let mut environment = Environment::new();

        let program = get_parser(code).parse();
        let mut buffer = Vec::<u8>::new();
        for statement in program {
            statement.execute(&mut environment, &mut buffer);
        }

        let output = vec!["hi", "yes", ""];
        assert_eq!(utf8_to_string(&buffer), output)
    }

    #[test]
    fn just_if() {
        let code = "
                if (true) {
                print \"true\";
                }
            ";
        let mut environment = Environment::new();

        let program = get_parser(code).parse();
        let mut buffer = Vec::<u8>::new();
        for statement in program {
            statement.execute(&mut environment, &mut buffer);
        }

        let output = vec!["true", ""];
        assert_eq!(utf8_to_string(&buffer), output)
    }
    #[test]
    fn if_then_else() {
        let code = "
                if (true) {
                print \"true\";
                } else {
                print \"false\"; 
                }
            ";
        let mut environment = Environment::new();

        let program = get_parser(code).parse();
        let mut buffer = Vec::<u8>::new();
        for statement in program {
            statement.execute(&mut environment, &mut buffer);
        }

        let output = vec!["true", ""];
        assert_eq!(utf8_to_string(&buffer), output)
    }

    #[test]
    fn if_then_else_alt() {
        let code = "
                if (false) {
                print \"true\";
                } else {
                print \"false\"; 
                }
            ";
        let mut environment = Environment::new();

        let program = get_parser(code).parse();
        let mut buffer = Vec::<u8>::new();
        for statement in program {
            statement.execute(&mut environment, &mut buffer);
        }

        let output = vec!["false", ""];
        assert_eq!(utf8_to_string(&buffer), output)
    }

    #[test]
    fn if_then_else_no_brackets() {
        let code = "
                if (true) 
                print \"true\";
                 else 
                print \"false\"; 
                
            ";
        let mut environment = Environment::new();

        let program = get_parser(code).parse();
        let mut buffer = Vec::<u8>::new();
        for statement in program {
            statement.execute(&mut environment, &mut buffer);
        }

        let output = vec!["true", ""];
        assert_eq!(utf8_to_string(&buffer), output)
    }

    #[test]
    fn nested() {
        let code = "
                if (true) 
                    if (false)
                        print \"true then false\";
                     else 
                        print \"true then true\";
                else
                    print \"unreachable\"; 
                
            ";
        let mut environment = Environment::new();

        let program = get_parser(code).parse();
        let mut buffer = Vec::<u8>::new();
        for statement in program {
            statement.execute(&mut environment, &mut buffer);
        }

        let output = vec!["true then true", ""];
        assert_eq!(utf8_to_string(&buffer), output)
    }

    #[test]
    fn while_loop() {
        let code = "
                  var i = 0;
                  while (i < 10) {
                    print i;
                    i = i + 1;
                  }
                
            ";
        let mut environment = Environment::new();

        let program = get_parser(code).parse();
        let mut buffer = Vec::<u8>::new();
        for statement in program {
            statement.execute(&mut environment, &mut buffer);
        }

        let output = vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", ""];
        assert_eq!(utf8_to_string(&buffer), output)
    }

    #[test]
    fn for_loop() {
        let code = "
                  for(var i = 0; i < 10; i = i+1) {
                    print i;
                  }
                
            ";
        let mut environment = Environment::new();

        let program = get_parser(code).parse();
        let mut buffer = Vec::<u8>::new();
        for statement in program {
            statement.execute(&mut environment, &mut buffer);
        }

        let output = vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", ""];
        assert_eq!(utf8_to_string(&buffer), output)
    }
}
