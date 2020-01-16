/// HTML tags
pub enum Tag {
   A,
   Abbr,
   Acronym,
   Address,
   Applet,
   Area,
   Article,
   Aside,
   Audio,
   B,
   Base,
   Basefont,
   Bdi,
   Bdo,
   Big,
   Blockquote,
   Body,
   Br,
   Button,
   Canvas,
   Caption,
   Center,
   Cite,
   Code,
   Col,
   Colgroup,
   Data,
   Datalist,
   Dd,
   Del,
   Details,
   Dfn,
   Dialog,
   Dir,
   Div,
   Dl,
   Dt,
   Em,
   Embed,
   Fieldset,
   FigCaption,
   Figure,
   Font,
   Footer,
   Form,
   Frame,
   Frameset,
   H1,
   Head,
   Header,
   Hr,
   Html,
   I,
   IFrame,
   Img,
   Input,
   Ins,
   Kbd,
   Label,
   Legend,
   Li,
   Link,
   Main,
   Map,
   Mark,
   Meta,
   Meter,
   Nav,
   Noframes,
   Noscript,
   Object,
   Ol,
   Optgroup,
   Option_,
   Output,
   P,
   Param,
   Picture,
   Pre,
   Progress,
   Q,
   Rp,
   Rt,
   Ruby,
   S,
   Samp,
   Script,
   Section,
   Select,
   Small,
   Source,
   Span,
   Strike,
   Strong,
   Style,
   Sub,
   Summary,
   Sup,
   Svg,
   Table,
   Tbody,
   Td,
   Template,
   TextArea,
   Tfoot,
   Th,
   Thead,
   Time,
   Title,
   Tr,
   Track,
   Tt,
   U,
   Ul,
   Var,
   Video,
   Wbr,
}

impl std::convert::From<Tag> for String {
   fn from(tag: Tag) -> Self {
      match tag {
         Tag::A => "a",
         Tag::Abbr => "abbr",
         Tag::Acronym => "acronym",
         Tag::Address => "address",
         Tag::Applet => "applet",
         Tag::Area => "area",
         Tag::Article => "article",
         Tag::Aside => "aside",
         Tag::Audio => "audio",
         Tag::B => "b",
         Tag::Base => "base",
         Tag::Basefont => "basefont",
         Tag::Bdi => "bdi",
         Tag::Bdo => "bdo",
         Tag::Big => "big",
         Tag::Blockquote => "blockquote",
         Tag::Body => "body",
         Tag::Br => "br",
         Tag::Button => "button",
         Tag::Canvas => "canvas",
         Tag::Caption => "caption",
         Tag::Center => "center",
         Tag::Cite => "cite",
         Tag::Code => "code",
         Tag::Col => "col",
         Tag::Colgroup => "colgroup",
         Tag::Data => "data",
         Tag::Datalist => "datalist",
         Tag::Dd => "dd",
         Tag::Del => "del",
         Tag::Details => "details",
         Tag::Dfn => "dfn",
         Tag::Dialog => "dialog",
         Tag::Dir => "dir",
         Tag::Div => "div",
         Tag::Dl => "dl",
         Tag::Dt => "dt",
         Tag::Em => "em",
         Tag::Embed => "embed",
         Tag::Fieldset => "fieldset",
         Tag::FigCaption => "figcaption",
         Tag::Figure => "figure",
         Tag::Font => "font",
         Tag::Footer => "footer",
         Tag::Form => "form",
         Tag::Frame => "frame",
         Tag::Frameset => "frameset",
         Tag::H1 => "h1",
         Tag::Head => "head",
         Tag::Header => "header",
         Tag::Hr => "hr",
         Tag::Html => "html",
         Tag::I => "i",
         Tag::IFrame => "iframe",
         Tag::Img => "img",
         Tag::Input => "input",
         Tag::Ins => "ins",
         Tag::Kbd => "kbd",
         Tag::Label => "label",
         Tag::Legend => "legend",
         Tag::Li => "li",
         Tag::Link => "link",
         Tag::Main => "main",
         Tag::Map => "map",
         Tag::Mark => "mark",
         Tag::Meta => "meta",
         Tag::Meter => "meter",
         Tag::Nav => "nav",
         Tag::Noframes => "noframes",
         Tag::Noscript => "noscript",
         Tag::Object => "object",
         Tag::Ol => "ol",
         Tag::Optgroup => "optgroup",
         Tag::Option_ => "option",
         Tag::Output => "output",
         Tag::P => "p",
         Tag::Param => "param",
         Tag::Picture => "picture",
         Tag::Pre => "pre",
         Tag::Progress => "progress",
         Tag::Q => "q",
         Tag::Rp => "rp",
         Tag::Rt => "rt",
         Tag::Ruby => "ruby",
         Tag::S => "s",
         Tag::Samp => "samp",
         Tag::Script => "script",
         Tag::Section => "section",
         Tag::Select => "select",
         Tag::Small => "small",
         Tag::Source => "source",
         Tag::Span => "span",
         Tag::Strike => "strike",
         Tag::Strong => "strong",
         Tag::Style => "style",
         Tag::Sub => "sub",
         Tag::Summary => "summary",
         Tag::Sup => "sup",
         Tag::Svg => "svg",
         Tag::Table => "table",
         Tag::Tbody => "tbody",
         Tag::Td => "td",
         Tag::Template => "template",
         Tag::TextArea => "textarea",
         Tag::Tfoot => "tfoot",
         Tag::Th => "th",
         Tag::Thead => "thead",
         Tag::Time => "time",
         Tag::Title => "title",
         Tag::Tr => "tr",
         Tag::Track => "track",
         Tag::Tt => "tt",
         Tag::U => "u",
         Tag::Ul => "ul",
         Tag::Var => "var",
         Tag::Video => "video",
         Tag::Wbr => "wbr",
      }.into()
   }
}