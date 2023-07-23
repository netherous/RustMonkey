
pub enum Token{
    // Identifiers + literals
    IDENT(String),
    INT(String), 
    // Operators
    ASSIGN ,
    PLUS, 
    // Delimiters
    COMMA, 
    SEMICOLON ,
    LPAREN, 
    RPAREN, 
    LBRACE ,
    RBRACE, 
    // Keywords
    FUNCTION, 
    LET, 
    //None match
    ILLEGAL, 
    EOF,

}
