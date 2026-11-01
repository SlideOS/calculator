#[derive(Debug, Clone, PartialEq)]
enum Token{
    Number(f64),
    Plus,
    Minus,
    Star,
    Slash,
    LeftParen,
    RightParen,
    Eof,
}
fn tokenize(input: &str) -> Result<Vec<Token>, &'static str>{
    let mut tokens: Vec<Token>=Vec::new();
    let mut string_number = String::new();
    let mut number: f64;
    
    for c in input.chars(){
        if c.is_whitespace(){
            continue;
            }
        if c.is_ascii_digit()|| c == '.'{
            string_number.push(c);
            continue;
        }else{
                if !string_number.is_empty(){
                    number = string_number
                    .parse::<f64>()
                    .map_err(|_| "Número invalido")?;
                    
                    tokens.push(Token::Number(number));
                    string_number.clear();
                }
            }
        match c{
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '*' => tokens.push(Token::Star),
            '/' => tokens.push(Token::Slash),
            '(' => tokens.push(Token::LeftParen),
            ')' => tokens.push(Token::RightParen),
            _ => return Err("Caractere desconhecido"),
            }
        }
        if !string_number.is_empty(){
            number = string_number
            .parse::<f64>()
            .map_err(|_| "Número invalido")?;
            
            tokens.push(Token::Number(number));
        }
        tokens.push(Token::Eof);
        Ok(tokens)
    }