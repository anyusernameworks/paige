use super::*;

/// The Page struct.
/// Stores a list of nested HTML elements.
pub struct Page {
   pub children: Vec<El>,
}

impl Page {
   /// Returns a new Page. Takes a slice of El as input.
   pub fn new(children: &[El]) -> Self {
      Page {
         children: children.to_vec(),
      }
   }
   
   /// Pushes an El onto children field vec.
   pub fn add(mut self, child: El) -> Self {
      self.children.push(child);
      self
   }
   
   /// Allows for finding a child element by its id attribute.
   pub fn id_find(&self, id: &str) -> Option<&El> {
      
      for child in self.children.iter() {
         
         let find = child.id_find(id);
         
         match find {
            Some(_) => { return find; },
            None => (),
         }
      }
      
      None
   }
   
   /// Formats the Page for display or storage. Automatically
   /// prepends '<!DOCTYPE HTML> to the beginning of the file.
   pub fn format(&self, make_pretty: bool) -> String {
      
      let mut f = Formatter {
         buf: String::new(),
      };
      
      f.write("<!DOCTYPE HTML>");
      
      for child in self.children.iter() {
         f = child.format(f, 0, make_pretty);
      }
      
      f.buf
      
   }
}

#[cfg(test)]
mod tests {
   
   use super::*;
   
   #[test]
   fn add() {
      
      let html = El::paired(Tag::Html, &[]);
      
      let page = Page::new(&[html]);
      
      assert!(page.children.len() == 1);
   }
   
   // #[test]
   // fn id_find_some() {
      
   //    let html = El::paired(Tag::Html, &[
         
   //       El::paired(Tag::Div, &[])
   //       .attributes(&[(Attr::Id, "div")]),
         
   //    ]).attributes(&[(Attr::Id, "html")]);
      
   //    let page = Page::new(&[html]);
      
   //    if let None = page.id_find("div") {
   //       panic!();
   //    }
   // }
   
   // #[test]
   // fn id_find_depth_4() {
      
   //    let html = El::paired(Tag::Html, &[
   //       El::paired(Tag::Div, &[
   //          El::paired(Tag::Div, &[
   //             El::paired(Tag::Div, &[])
   //                .attributes(&[
   //                   (Attr::Id, "div"),
   //                   (Attr::Name, "xD")
   //                ]),
   //          ]),
   //          El::paired(Tag::Div, &[])
   //             .attributes(&[
   //                (Attr::Id, "div"),
   //                (Attr::Name, "xP")
   //             ]),
   //       ]),
   //    ])
   //    .attributes(&[(Attr::Id, "html")]);
      
   //    let page = Page::new(&[html]);
      
   //    assert_eq!(page.id_find("div").unwrap().attributes[1].value, "xD");
   // }
   
   // #[test]
   // #[should_panic]
   // fn id_find_none() {
      
   //    let html = El::paired(Tag::Html, &[
         
   //       El::paired(Tag::Div, &[])
   //       .attributes(&[(Attr::Id, "div")]),
         
   //    ])
   //       .attributes(&[
   //          (Attr::Id, "html"),
   //       ]);
      
   //    let page = Page::new(&[html]);
      
   //    if let None = page.id_find("htmk") {
   //       panic!();
   //    }
   // }
   
   #[test]
   fn format_bare_tags() {
      
      use Tag::*;
      
      let page = Page::new(&[
         El::paired(Html, &[
            El::paired(Head, &[
               El::paired(Style, &[]),
            ]),
            El::paired(Body, &[]),
         ])
      ]).format(false);
      
      assert_eq!(page, "<!DOCTYPE HTML><html><head><style></style></head><body></body></html>");
   }
   
   #[test]
   fn format_bare_tags_pretty() {
      
      use Tag::*;
      
      let page = Page::new(&[
         El::paired(Html, &[
            El::paired(Head, &[
               El::paired(Style, &[]),
            ]),
            El::paired(Body, &[]),
         ])
      ]).format(true);
      
      assert_eq!(page, "<!DOCTYPE HTML>\n<html>\n   <head>\n      <style></style>\n   </head>\n   <body></body>\n</html>");
   }
}

impl std::fmt::Display for Page {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      
      write!(f, "{}", self.format(true))?;
      
      Ok(())
      
   }
}