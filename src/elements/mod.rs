mod attributes;
pub use attributes::*;

mod element;
pub use element::*;

mod properties;
pub use properties::*;

mod tags;
pub use tags::*;


#[derive(Clone)]
pub enum Up {
   Text(String),
   El(Element),
}

impl Up {
   pub fn tag(tag: Element) -> Up {
      Up::El(tag)
   }
   
   pub fn text<N: Into<String>>(text: N) -> Self {
      Up::Text(text.into())
   }
   
   pub fn paired<N: Into<String>>(name: N, children: &[Up]) -> Self {
      Up::El(Element::paired(name, children))
   }

   pub fn unpaired<N: Into<String>>(name: N) -> Self {
      Up::El(Element::unpaired(name))
   }
}

impl std::fmt::Display for Up {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self {
         Up::El(el) => write!(f, "{}", el),
         Up::Text(text) => write!(f, "{}", text)
      }
   }
}