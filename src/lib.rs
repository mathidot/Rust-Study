struct StrSplit<'a, D> {
    pub remainder: Option<&'a str>,
    pub delimeter: D,
}

impl<'a, D> StrSplit<'a, D> {
    pub fn new(remainder: &'a str, delimeter: D) -> Self {
        Self { remainder: Some(remainder), delimeter }
    }
}

impl<'a, D> Iterator for StrSplit<'a, D>
where
    D: Delimeter,
{
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder.as_mut()?;
        if let Some((start, end)) = self.delimeter.find_next(&remainder) {
            let token = &remainder[..start];
            *remainder = &remainder[end..];
            Some(token)
        } else {
            self.remainder.take()
        }
    }
}

pub trait Delimeter {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

impl Delimeter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self).map(|start| (start, start+self.len()))
    }
}

impl Delimeter for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.char_indices()
            .find(|(_, c)| c == self)
            .map(|(start, _)| (start, start + self.len_utf8()))
    }
}

fn until_char<'a>(s: &'a str, c: char) -> &str {
    let deli: &str = &format!("{}", c);
    return StrSplit::new(s, deli)
            .next()
            .expect("StrSplit always gives at least one result");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_str() {
        let haystack = "a b c d e";
        let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
        assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
    }

    #[test]
    fn test_split_tail_sapce() {
        let haystack = "a b c d e ";
        let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
        assert_eq!(letters, vec!["a", "b", "c", "d", "e", ""]);
    }

    #[test]
    fn test_until_char() {
        let s = "hello world";
        assert_eq!("hell", until_char(s, 'o'));
    }
}
