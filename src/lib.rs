mod tags;
mod elements;

use elements::Element;

pub trait HtmlAttribute {
    type Error;
    fn key(&self) -> &[u8];
    fn value(&self) -> Option<Result<&str, Self::Error>>;
}

pub trait HtmlElement<A>
    where A: IntoIterator,
          A::Item: HtmlAttribute
{
    fn name(&self) -> &[u8];
    fn attributes(&self) -> A;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
