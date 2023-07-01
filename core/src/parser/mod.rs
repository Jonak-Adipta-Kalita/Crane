#![allow(dead_code, unused_imports)]
mod tree;
use crate::lexer::Token;
use crate::lexer::TokenType;
use crate::throw;
use ansi_term::Colour::Red;
use tree::Node;
use tree::NodeType;
use tree::Tree;
//create a parser struct that uses peekable iterator for the tokens
#[derive(Debug, Clone)]
pub struct Parser {
    tokens: std::iter::Peekable<std::vec::IntoIter<Token>>,
    pub tree: Tree,
}

//initialize the parser struct
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: tokens.into_iter().peekable(),
            tree: Tree::new(),
        }
    }
    fn next(&mut self) -> Option<Token> {
        self.tokens.next()
    }
    fn peek(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }
}
//create functions for parsing
impl Parser {
    //parse the tokens
    pub fn parse(&mut self) {
        while let Some(token) = self.next() {
            let node = self.parse_token(token);
            self.tree.add_node(node);
        }
    }
    //parse a token
    fn parse_token(&mut self, token: Token) -> Node {
        match token.token_type {
            TokenType::Number(f) => Node::new(NodeType::Number, Some(f), token.line),
            TokenType::Str(s) => Node::new(NodeType::String, Some(s), token.line),
            TokenType::Identifier(s) => {
                //this could be a function call, a variable, or a keyword
                //check if the next token is a parenthesis
                if (*self).peek().unwrap().token_type == TokenType::LeftParen {
                    self.next();
                    //this is a function call
                    let mut node = Node::new(NodeType::FunctionCall, Some(s), token.line);
                    //parse the arguments
                    let mut args = Vec::new();
                    while let Some(token) = self.next() {
                        if token.token_type == TokenType::RightParen {
                            break;
                        }
                        let arg = self.parse_token(token);
                        args.push(arg);
                    }
                    //add the arguments to the node
                    for arg in args {
                        node.add_child(arg);
                    }
                    node
                } else {
                    Node::new(NodeType::Identifier, Some(s), token.line)
                }
            }
            //if the token is an operator, create a node with the operator type, left child, and right child
            TokenType::Operator(op) => Node::new(NodeType::Operator, Some(op), token.line),
            TokenType::Keyword(kw) => Node::new(NodeType::Keyword, Some(kw), token.line),
            _ => Node::new(NodeType::Err, None, token.line),
        }
    }
}
