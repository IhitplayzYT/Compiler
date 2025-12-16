pub mod Frontend {
    use crate::Parser::PARSER::{self, Parser};
    use crate::Semantic_Analysis::Analyser::Semantilizer;
    use crate::Tokeniser::Tokeniser::Lexer;
    pub struct Frontend {
        parser: Parser,
        lexer: Lexer,
        Semantics: Semantilizer,
    }
}
