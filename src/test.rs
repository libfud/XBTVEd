fn main() {

    let expr = "\"~/foo/bar\"";
    if expr.starts_with("\"") {
        /* c == '"' is kind of a bad assumption, but I haven't really encountered *
         * many quotes in filenames. I should probably come back and try to find  *
         * a better solution later */

        let close = match expr.chars().skip(1).position(|c: char| c == '"') {
            Some(x) =>{
                println!("{}", x);
                x
            }
            None => 1 //return ParseError::BadToken("Cannot find closing quote!".to_string())
        };
        let location = expr.slice_chars(1, close + 1);
        println!("{}", location);
    } 
}
