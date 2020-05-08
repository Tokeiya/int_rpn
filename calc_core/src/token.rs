use std::fmt;
use std::fmt::Formatter;

pub enum OperatorTypes{
    Add,
    Subtract,
    Multiply,
    Divide
}

pub enum Token
{
    Integer(i32),
    Operator(OperatorTypes)
}

impl fmt::Display for Token{

    fn fmt(&self, f: &mut Formatter) -> fmt::Result {

        fn operator(op:&OperatorTypes)->char{
            match op {
                OperatorTypes::Add => '+',
                OperatorTypes::Subtract => '-',
                OperatorTypes::Multiply => '*',
                OperatorTypes::Divide => '/'
            }
        };

        match self {
            Token::Integer(i)=>write!(f,"{}",i),
            Token::Operator(o)=>write!(f,"{}",operator(o))
        }
    }
}

#[cfg(test)]
mod tests{
    use super::Token;
    use super::OperatorTypes;

    #[test]
    fn token_display_test(){

        fn assert(target:Token,expected:&str){
            assert_eq!(format!("{}",target),expected);
        }

        assert(Token::Integer(42),"42");
        assert(Token::Integer(-114514),"-114514");

        assert(Token::Operator(OperatorTypes::Divide),"/");
        assert(Token::Operator(OperatorTypes::Add),"+");
        assert(Token::Operator(OperatorTypes::Multiply),"*");
        assert(Token::Operator(OperatorTypes::Subtract),"-");
    }

}