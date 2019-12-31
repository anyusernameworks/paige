use super::*;

#[derive(Clone)]
pub struct HtmlTag {
   name: String,
   paired: bool,
   attributes: Vec<Attr>,
   style: Vec<Prop>,
   children: Vec<Element>,
}

impl HtmlTag {
   pub fn paired<N: Into<String>>(name: N, children: &[Element]) -> HtmlTag {
      HtmlTag {
         name: name.into(),
         paired: true,
         attributes: vec![],
         style: vec![],
         children: children.to_vec(),
      }
   }

   pub fn unpaired<N: Into<String>>(name: N) -> HtmlTag {
      HtmlTag {
         name: name.into(),
         paired: false,
         attributes: vec![],
         style: vec![],
         children: vec![],
      }
   }

   pub fn add_child(&mut self, child: Element) {
      match self.paired {
         true => self.children.push(child),
         false => panic!("Unpaired tags cannot have children"),
      }
   }

   pub fn add_attribute(&mut self, attribute: Attr) {
      self.attributes.push(attribute);
   }
}

impl std::fmt::Display for HtmlTag {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      
      write!(f, "\n<{}", self.name)?;
      
      if self.attributes.len() > 0 {
         write!(f, "{}", self.attributes.iter().map(|a| format!("{}", a) ).collect::<Vec<String>>().join(" "))?;
      }
      
      if self.style.len() > 0 {
         write!(f, "{}", self.style.iter().map(|p| format!("{}", p) ).collect::<Vec<String>>().join(" "))?;
      }
      
      write!(f, ">")?;
      
      if self.paired {
         
         for child in self.children.iter() {
            write!(f, "{}", child)?;
         }
         
         write!(f, "</{}>", self.name)?;
         
      }
      
      Ok(())
      
   }
}

pub enum TagName {
   DOCTYPE,
   a,
   abbr,
   acronym,
   address,
   applet,
   area,
   article,
   aside,
   audio,
   b,
   base,
   basefont,
   bdi,
   bdo,
   big,
   blockquote,
   body,
   br,
   button,
   canvas,
   caption,
   center,
   cite,
   code,
   col,
   colgroup,
   data,
   datalist,
   dd,
   del,
   details,
   dfn,
   dialog,
   dir,
   div,
   dl,
   dt,
   em,
   embed,
   fieldset,
   figcaption,
   figure,
   font,
   footer,
   form,
   frame,
   frameset,
   h1,
   head,
   header,
   hr,
   html,
   i,
   iframe,
   img,
   input,
   ins,
   kbd,
   label,
   legend,
   li,
   link,
   main,
   map,
   mark,
   meta,
   meter,
   nav,
   noframes,
   noscript,
   object,
   ol,
   optgroup,
   option,
   output,
   p,
   param,
   picture,
   pre,
   progress,
   q,
   rp,
   rt,
   ruby,
   s,
   samp,
   script,
   section,
   select,
   small,
   source,
   span,
   strike,
   strong,
   style,
   sub,
   summary,
   sup,
   svg,
   table,
   tbody,
   td,
   template,
   textarea,
   tfoot,
   th,
   thead,
   time,
   title,
   tr,
   track,
   tt,
   u,
   ul,
   var,
   video,
   wbr,
}

impl TagName {
   pub fn to_str(&self) -> &str {
      match self {
         TagName::DOCTYPE => "DOCTYPE",
         TagName::a => "a",
         TagName::abbr => "abbr",
         TagName::acronym => "acronym",
         TagName::address => "address",
         TagName::applet => "applet",
         TagName::area => "area",
         TagName::article => "article",
         TagName::aside => "aside",
         TagName::audio => "audio",
         TagName::b => "b",
         TagName::base => "base",
         TagName::basefont => "basefont",
         TagName::bdi => "bdi",
         TagName::bdo => "bdo",
         TagName::big => "big",
         TagName::blockquote => "blockquote",
         TagName::body => "body",
         TagName::br => "br",
         TagName::button => "button",
         TagName::canvas => "canvas",
         TagName::caption => "caption",
         TagName::center => "center",
         TagName::cite => "cite",
         TagName::code => "code",
         TagName::col => "col",
         TagName::colgroup => "colgroup",
         TagName::data => "data",
         TagName::datalist => "datalist",
         TagName::dd => "dd",
         TagName::del => "del",
         TagName::details => "details",
         TagName::dfn => "dfn",
         TagName::dialog => "dialog",
         TagName::dir => "dir",
         TagName::div => "div",
         TagName::dl => "dl",
         TagName::dt => "dt",
         TagName::em => "em",
         TagName::embed => "embed",
         TagName::fieldset => "fieldset",
         TagName::figcaption => "figcaption",
         TagName::figure => "figure",
         TagName::font => "font",
         TagName::footer => "footer",
         TagName::form => "form",
         TagName::frame => "frame",
         TagName::frameset => "frameset",
         TagName::h1 => "h1",
         TagName::head => "head",
         TagName::header => "header",
         TagName::hr => "hr",
         TagName::html => "html",
         TagName::i => "i",
         TagName::iframe => "iframe",
         TagName::img => "img",
         TagName::input => "input",
         TagName::ins => "ins",
         TagName::kbd => "kbd",
         TagName::label => "label",
         TagName::legend => "legend",
         TagName::li => "li",
         TagName::link => "link",
         TagName::main => "main",
         TagName::map => "map",
         TagName::mark => "mark",
         TagName::meta => "meta",
         TagName::meter => "meter",
         TagName::nav => "nav",
         TagName::noframes => "noframes",
         TagName::noscript => "noscript",
         TagName::object => "object",
         TagName::ol => "ol",
         TagName::optgroup => "optgroup",
         TagName::option => "option",
         TagName::output => "output",
         TagName::p => "p",
         TagName::param => "param",
         TagName::picture => "picture",
         TagName::pre => "pre",
         TagName::progress => "progress",
         TagName::q => "q",
         TagName::rp => "rp",
         TagName::rt => "rt",
         TagName::ruby => "ruby",
         TagName::s => "s",
         TagName::samp => "samp",
         TagName::script => "script",
         TagName::section => "section",
         TagName::select => "select",
         TagName::small => "small",
         TagName::source => "source",
         TagName::span => "span",
         TagName::strike => "strike",
         TagName::strong => "strong",
         TagName::style => "style",
         TagName::sub => "sub",
         TagName::summary => "summary",
         TagName::sup => "sup",
         TagName::svg => "svg",
         TagName::table => "table",
         TagName::tbody => "tbody",
         TagName::td => "td",
         TagName::template => "template",
         TagName::textarea => "textarea",
         TagName::tfoot => "tfoot",
         TagName::th => "th",
         TagName::thead => "thead",
         TagName::time => "time",
         TagName::title => "title",
         TagName::tr => "tr",
         TagName::track => "track",
         TagName::tt => "tt",
         TagName::u => "u",
         TagName::ul => "ul",
         TagName::var => "var",
         TagName::video => "video",
         TagName::wbr => "wbr",
      }
   }
}