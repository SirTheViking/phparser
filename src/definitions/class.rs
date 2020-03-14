use crate::lexer::Token;
use crate::definitions::{ FunctionDef, ExtractTokens };


pub struct ClassDef {
    name: String,
    methods: Vec<FunctionDef>
}


impl ClassDef {
    pub fn new() -> Self {
        ClassDef {
            name: String::with_capacity(20),
            methods: vec![]
        }
    }


    pub fn name(&self) -> &String {
        &self.name
    }


    pub fn methods(&self) -> Option<&Vec<FunctionDef>> {
        if self.methods.is_empty() {
            return None;
        }

        Some(&self.methods)
    }

    fn new_method(&mut self) {
        self.methods.push(FunctionDef::new());
    }


    fn last_method(&mut self) -> Option<&mut FunctionDef> {
        self.methods.last_mut()
    }
}


impl ExtractTokens for ClassDef {
    fn take(&mut self, token: Token) {
        match token {
            Token::ClassName(n) => self.name = n,

            Token::FunctionStart => self.new_method(),
            Token::FunctionEnd => (),

            _ => {
                if let Some(method) = self.last_method() {
                    method.take(token);
                }
            }
        }
    }
}