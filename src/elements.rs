use {HtmlAttribute, HtmlElement};

pub enum Element {
    Root,
    Li(Li)
}

impl Element {
    fn from_html<E, A>(e: E) -> Result<Element, <A::Item as HtmlAttribute>::Error> 
        where A: IntoIterator,
              A::Item: HtmlAttribute,
              E: HtmlElement<A>,
    {
        match e.name() {
            b"html" => Ok(Element::Root),
            b"li" => Ok(Element::Li(Li::from_html(e.attributes())?)),
            _ => unimplemented!(),
        }
    }
}

pub struct Li {
    pub value: Option<i64>,
}

impl Li {
    fn from_html<A>(attributes: A) -> Result<Self, <A::Item as HtmlAttribute>::Error> 
        where A: IntoIterator,
              A::Item: HtmlAttribute,
    {
        for a in attributes {
            match (a.key(), a.value()) {
                (b"value", Some(v)) => return Ok(Li { value: Some(v?.parse().unwrap()) }),
                _ => (),
            }
        }
        Ok(Li { value: None } )
    }
}

