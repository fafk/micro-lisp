use crate::Token;
use std::cmp::Ordering;

impl std::ops::Add<Token> for Token {
    type Output = Token;

    fn add(self, rhs: Token) -> Self::Output {
        if let Token::Int(i) = &self {
            if let Token::Int(int_rhs) = &rhs {
                return Token::Int(int_rhs + i);
            }
        }
        panic!("you can add only integers");
    }
}

impl std::ops::Sub<Token> for Token {
    type Output = Token;

    fn sub(self, rhs: Token) -> Self::Output {
        if let Token::Int(i) = &self {
            if let Token::Int(int_rhs) = &rhs {
                return Token::Int(i - int_rhs);
            }
        }
        panic!("you can subtract only integers");
    }
}

impl std::ops::Mul<Token> for Token {
    type Output = Token;

    fn mul(self, rhs: Token) -> Self::Output {
        if let Token::Int(i) = &self {
            if let Token::Int(int_rhs) = &rhs {
                return Token::Int(int_rhs * i);
            }
        }
        panic!("you can multiply only integers");
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Token::Open => match other {
                Token::Open => true,
                _ => false,
            },
            Token::Close => match other {
                Token::Open => true,
                _ => false,
            },
            Token::Int(i1) => match other {
                Token::Int(i2) => i1 == i2,
                _ => false,
            },
            Token::Symbol(s1) => match other {
                Token::Symbol(s2) => s1 == s2,
                _ => false,
            },
            Token::List(l1) => match other {
                Token::List(l2) => l1 == l2,
                _ => false,
            },
            Token::True => match other {
                Token::True => true,
                _ => false,
            },
            Token::False => match other {
                Token::False => true,
                _ => false,
            },
        }
    }
}

impl PartialOrd for Token {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Token {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Token::Int(i1) => match other {
                Token::Int(i2) => i1.cmp(i2),
                _ => panic!("comparison works only for numbers and symbols"),
            },
            Token::Symbol(s1) => match other {
                Token::Symbol(s2) => s1.cmp(s2),
                _ => panic!("comparison works only for numbers and symbols"),
            },
            _ => panic!("comparison works only for numbers and symbols"),
        }
    }
}
