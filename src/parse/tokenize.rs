//! Tokenizes strings.

pub type MaybeToken<T, U> = (Option<Result<T, U>>, usize);

pub struct TokenStream<T: Clone, U: Clone> {
    expr: String,
    fwd_index: usize,
    rev_index: usize,
    next_rules: Vec<fn(&str) -> MaybeToken<T, U>>,
    back_rules: Vec<fn(&str) -> MaybeToken<T, U>>,
    on_exhaustion: U,
}

impl<T: Clone, U: Clone> TokenStream<T, U> {
    pub fn new(e: &str, 
               next_rules: Vec<fn(&str) -> MaybeToken<T, U>>, 
               back_rules: Vec<fn(&str) -> MaybeToken<T, U>>,
               on_exhaustion: U) -> TokenStream<T, U> {

        TokenStream { 
            expr: e.to_string(), 
            fwd_index: 0,
            rev_index: e.len(),
            next_rules: next_rules, 
            back_rules: back_rules, 
            on_exhaustion: on_exhaustion
        }
    }

    pub fn expr(&self) -> String {
        self.expr.clone()
    }

    pub fn fwd_index(&self) -> usize {
        self.fwd_index
    }

    pub fn rev_index(&self) -> usize {
        self.rev_index
    }

    pub fn on_exhaustion(&self) -> U {
        self.on_exhaustion.clone()
    }

    pub fn previous(&mut self) -> Option<Result<T, U>> {
        if self.fwd_index == 0 {
            return None
        } else {
            if self.expr[.. self.fwd_index].ends_with(|c: char| c.is_whitespace()) {
                self.fwd_index -= 1;
                self.previous()
            } else {
                let (token, len) = analyze(&self.expr[.. self.fwd_index], &self.back_rules, &self.on_exhaustion);      
                self.fwd_index -= len;
                token
            }
        }
    }

}

impl<T: Clone, U: Clone> Iterator for TokenStream<T, U> {
    type Item = Result<T, U>;

    fn next(&mut self) -> Option<Result<T, U>> {
        if self.fwd_index == self.expr.len() {
            return None
        } else {
            if self.expr[self.fwd_index ..].starts_with(|c: char| c.is_whitespace()) {
                self.fwd_index += 1;
                self.next()
            } else {
                let (token, len) = analyze(&self.expr[self.fwd_index ..], &self.next_rules, &self.on_exhaustion);      
                self.fwd_index += len;
                token
            }
        }
    }
}

impl<T: Clone, U: Clone> DoubleEndedIterator for TokenStream<T, U> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.rev_index == 0 {
            return None
        } else {
            if self.expr[..self.rev_index].ends_with(|c: char| c.is_whitespace()) {
                self.rev_index -= 1;
                self.next_back()
            } else {
                let (token, len) = analyze(&self.expr[.. self.rev_index], &self.back_rules, &self.on_exhaustion);
                self.rev_index -= len;
                token
            }
        }
    }
}

fn analyze<T, U: Clone>(expr: &str, funs: &Vec<fn(&str) -> MaybeToken<T, U>>, 
                            on_exhaustion: &U) -> MaybeToken<T, U> {

    for &fun in funs.iter() {
        let (token, len) = fun(expr);
        if token.is_some() {
            return (token, len)
        }
    }

    (Some(Err(on_exhaustion.clone())), 0)
}

impl<T: Clone, U: Clone> Clone for TokenStream<T, U> {
    fn clone(&self) -> TokenStream<T, U> {
        TokenStream {
            expr: self.expr.clone(),
            fwd_index: self.fwd_index.clone(),
            rev_index: self.rev_index.clone(),
            next_rules: self.next_rules.iter().map(|x| *x).collect::<Vec<fn(&str) -> MaybeToken<T, U>>>(),
            back_rules: self.back_rules.iter().map(|x| *x).collect::<Vec<fn(&str) -> MaybeToken<T, U>>>(),
            on_exhaustion: self.on_exhaustion.clone()
        }
    }
}
