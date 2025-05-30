pub const TOKEN_CHARS: [char; 8] = ['>', '<', '+', '-', '.', ',', '[', ']'];

pub fn tokenize(source: impl Into<String>) -> Vec<char> {
    let source: String      = source.into();
    let source: Vec<char>   = source.chars()
        .filter(is_token)
        .collect();
    source
}

fn is_token(char: &char) -> bool {
    TOKEN_CHARS.contains(char)
}

#[cfg(test)]
mod test {
    use super::tokenize;

    #[test]
    fn test_tokenize_single() {
        let sources = vec!['>', '<', '+', '-', '.', ',', '[', ']'];
        let tokens_exp = vec!['>', '<', '+', '-', '.', ',', '[', ']'];
        let tokens_ret = sources.into_iter().flat_map(tokenize).collect::<Vec<char>>();

        assert_eq!(tokens_exp, tokens_ret);
    }

    #[test]
    fn test_tokenize_string() {
        let source = "+-><][.,";
        let tokens_exp = vec!['+', '-', '>', '<', ']', '[', '.', ','];
        let tokens_ret = tokenize(source);

        assert_eq!(tokens_exp, tokens_ret);
    }

    #[test]
    fn test_tokenize_with_comments() {
        let source = "hello+this-is/a[comment]";
        let tokens_exp = vec!['+', '-', '[', ']'];
        let tokens_ret = tokenize(source);

        assert_eq!(tokens_exp, tokens_ret);
    }
}
