use crate::tokenizer::{Token, Tokenizer};
use trees::Tree;

#[derive(Debug, Clone)]
pub enum NodeParamValueKind {
    String(String),
}

impl From<&str> for NodeParamValueKind {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl ToString for NodeParamValueKind {
    fn to_string(&self) -> String {
        match self {
            Self::String(v) => v.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Node {
    Command { value: String },
    Param { value: NodeParamValueKind },
}

pub struct Parser;

impl Parser {
    pub fn parse(&self, token: &Token, tokenizer: &mut Tokenizer) -> Tree<Node> {
        let mut cmd = Tree::new(Node::Command {
            value: token.value.to_string(),
        });

        cmd.push_back(Tree::new(Node::Param {
            value: NodeParamValueKind::from(token.value.as_str()),
        }));

        while let Some(token) = tokenizer.tokenize() {
            if token.value.chars().nth(0) == Some('\n') {
                break;
            }

            let value = NodeParamValueKind::from(token.value.as_str());
            let child = Tree::new(Node::Param { value });
            cmd.push_back(child);
        }

        cmd
    }
}
