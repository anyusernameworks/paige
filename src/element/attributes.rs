/// HTML attribute enum.
#[derive(Clone)]
pub enum Attr {
   Accept(String),
   AcceptCharset(String),
   AccessKey(String),
   Action(String),
   Align(String),
   Alt(String),
   Async(String),
   AutoComplete(String),
   AutoFocus(String),
   AutoPlay(String),
   BackgroundColor(String),
   Border(String),
   Charset(String),
   Checked(String),
   Cite(String),
   Class(String),
   Color(String),
   Columns(String),
   ColumnSpan(String),
   Content(String),
   ContentEditable(String),
   Controls(String),
   Coords(String),
   Data(String),
   DateTime(String),
   Default_(String),
   Defer(String),
   Dir(String),
   DirName(String),
   Disabled(String),
   Download(String),
   Draggable(String),
   DropZone(String),
   EncodingType(String),
   For(String),
   Form(String),
   FormAction(String),
   Headers(String),
   Height(String),
   Hidden(String),
   High(String),
   Href(String),
   Hreflang(String),
   HttpEquivalent(String),
   Id(String),
   Identifier(String),
   IsMap(String),
   Kind(String),
   Label(String),
   Language(String),
   List(String),
   Loop(String),
   Low(String),
   Max(String),
   MaxLength(String),
   Media(String),
   Method(String),
   Min(String),
   Multiple(String),
   Muted(String),
   Name(String),
   NoValidate(String),
   OnAbort(String),
   OnAfterPrint(String),
   OnBeforePrint(String),
   OnBeforeUnload(String),
   OnBlur(String),
   OnCanPlay(String),
   OnCanPlayThrough(String),
   OnChange(String),
   OnClick(String),
   OnContextMenu(String),
   OnCopy(String),
   OnCueChange(String),
   OnCut(String),
   OnDoubleClick(String),
   OnDrag(String),
   OnDragEnd(String),
   OnDragEnter(String),
   OnDragLeave(String),
   OnDragOver(String),
   OnDragStart(String),
   OnDrop(String),
   OnDurationChange(String),
   OnEmptied(String),
   OnEnded(String),
   OnError(String),
   OnFocus(String),
   OnHashChange(String),
   OnInput(String),
   OnInvalid(String),
   OnKeyDown(String),
   OnKeyPress(String),
   OnKeyUp(String),
   OnLoad(String),
   OnLoadedData(String),
   OnLoadedMetadata(String),
   OnLoadStart(String),
   OnMouseDown(String),
   OnMouseMove(String),
   OnMouseOut(String),
   OnMouseOver(String),
   OnMouseUp(String),
   OnMouseWheel(String),
   OnOffline(String),
   OnOnline(String),
   OnPageHide(String),
   OnPageShow(String),
   OnPaste(String),
   OnPause(String),
   OnPlay(String),
   OnPlaying(String),
   OnPopState(String),
   OnProgress(String),
   OnRateChange(String),
   OnReset(String),
   OnResize(String),
   OnScroll(String),
   OnSearch(String),
   OnSeeked(String),
   OnSeeking(String),
   OnSelect(String),
   OnStalled(String),
   OnStorage(String),
   OnSubmit(String),
   OnSuspend(String),
   OnTimeUpdate(String),
   OnToggle(String),
   OnUnload(String),
   OnVolumeChange(String),
   OnWaiting(String),
   OnWheel(String),
   Open(String),
   Optimum(String),
   Pattern(String),
   Placeholder(String),
   Poster(String),
   Preload(String),
   ReadOnly(String),
   Relative(String),
   Required(String),
   Reversed(String),
   Rows(String),
   RowSpan(String),
   Sandbox(String),
   Scope(String),
   Selected(String),
   Shape(String),
   Size(String),
   Sizes(String),
   Span(String),
   Spellcheck(String),
   Source(String),
   SourceDoc(String),
   SourceLanguage(String),
   SourceSet(String),
   Start(String),
   Step(String),
   Style(String),
   TabIndex(String),
   Target(String),
   Title(String),
   Translate(String),
   Type(String),
   UseMap(String),
   Value(String),
   Width(String),
   Wrap(String),
}
impl Attr {
   pub fn into_string(&self) -> String {
      match self {
         Attr::Accept(val) => format!("accept='{}'", val),
         Attr::AcceptCharset(val) => format!("accept-charset='{}'", val),
         Attr::AccessKey(val) => format!("accesskey='{}'", val),
         Attr::Action(val) => format!("action='{}'", val),
         Attr::Align(val) => format!("align='{}'", val),
         Attr::Alt(val) => format!("alt='{}'", val),
         Attr::Async(val) => format!("async='{}'", val),
         Attr::AutoComplete(val) => format!("autocomplete='{}'", val),
         Attr::AutoFocus(val) => format!("autofocus='{}'", val),
         Attr::AutoPlay(val) => format!("autoplay='{}'", val),
         Attr::BackgroundColor(val) => format!("bgcolor='{}'", val),
         Attr::Border(val) => format!("border='{}'", val),
         Attr::Charset(val) => format!("charset='{}'", val),
         Attr::Checked(val) => format!("checked='{}'", val),
         Attr::Cite(val) => format!("cite='{}'", val),
         Attr::Class(val) => format!("class='{}'", val),
         Attr::Color(val) => format!("color='{}'", val),
         Attr::Columns(val) => format!("cols='{}'", val),
         Attr::ColumnSpan(val) => format!("colspan='{}'", val),
         Attr::Content(val) => format!("content='{}'", val),
         Attr::ContentEditable(val) => format!("contenteditable='{}'", val),
         Attr::Controls(val) => format!("controls='{}'", val),
         Attr::Coords(val) => format!("coords='{}'", val),
         Attr::Data(val) => format!("data='{}'", val),
         Attr::DateTime(val) => format!("datetime='{}'", val),
         Attr::Default_(val) => format!("default='{}'", val),
         Attr::Defer(val) => format!("defer='{}'", val),
         Attr::Dir(val) => format!("dir='{}'", val),
         Attr::DirName(val) => format!("dirname='{}'", val),
         Attr::Disabled(val) => format!("disabled='{}'", val),
         Attr::Download(val) => format!("download='{}'", val),
         Attr::Draggable(val) => format!("draggable='{}'", val),
         Attr::DropZone(val) => format!("dropzone='{}'", val),
         Attr::EncodingType(val) => format!("enctype='{}'", val),
         Attr::For(val) => format!("for='{}'", val),
         Attr::Form(val) => format!("form='{}'", val),
         Attr::FormAction(val) => format!("formaction='{}'", val),
         Attr::Headers(val) => format!("headers='{}'", val),
         Attr::Height(val) => format!("height='{}'", val),
         Attr::Hidden(val) => format!("hidden='{}'", val),
         Attr::High(val) => format!("high='{}'", val),
         Attr::Href(val) => format!("href='{}'", val),
         Attr::Hreflang(val) => format!("hreflang='{}'", val),
         Attr::HttpEquivalent(val) => format!("http-equiv='{}'", val),
         Attr::Id(val) => format!("id='{}'", val),
         Attr::Identifier(val) => format!("id='{}'", val),
         Attr::IsMap(val) => format!("ismap='{}'", val),
         Attr::Kind(val) => format!("kind='{}'", val),
         Attr::Label(val) => format!("label='{}'", val),
         Attr::Language(val) => format!("lang='{}'", val),
         Attr::List(val) => format!("list='{}'", val),
         Attr::Loop(val) => format!("loop='{}'", val),
         Attr::Low(val) => format!("low='{}'", val),
         Attr::Max(val) => format!("max='{}'", val),
         Attr::MaxLength(val) => format!("maxlength='{}'", val),
         Attr::Media(val) => format!("media='{}'", val),
         Attr::Method(val) => format!("method='{}'", val),
         Attr::Min(val) => format!("min='{}'", val),
         Attr::Multiple(val) => format!("multiple='{}'", val),
         Attr::Muted(val) => format!("muted='{}'", val),
         Attr::Name(val) => format!("name='{}'", val),
         Attr::NoValidate(val) => format!("novalidate='{}'", val),
         Attr::OnAbort(val) => format!("onabort='{}'", val),
         Attr::OnAfterPrint(val) => format!("onafterprint='{}'", val),
         Attr::OnBeforePrint(val) => format!("onbeforeprint='{}'", val),
         Attr::OnBeforeUnload(val) => format!("onbeforeunload='{}'", val),
         Attr::OnBlur(val) => format!("onblur='{}'", val),
         Attr::OnCanPlay(val) => format!("oncanplay='{}'", val),
         Attr::OnCanPlayThrough(val) => format!("oncanplaythrough='{}'", val),
         Attr::OnChange(val) => format!("onchange='{}'", val),
         Attr::OnClick(val) => format!("onclick='{}'", val),
         Attr::OnContextMenu(val) => format!("oncontextmenu='{}'", val),
         Attr::OnCopy(val) => format!("oncopy='{}'", val),
         Attr::OnCueChange(val) => format!("oncuechange='{}'", val),
         Attr::OnCut(val) => format!("oncut='{}'", val),
         Attr::OnDoubleClick(val) => format!("ondblclick='{}'", val),
         Attr::OnDrag(val) => format!("ondrag='{}'", val),
         Attr::OnDragEnd(val) => format!("ondragend='{}'", val),
         Attr::OnDragEnter(val) => format!("ondragenter='{}'", val),
         Attr::OnDragLeave(val) => format!("ondragleave='{}'", val),
         Attr::OnDragOver(val) => format!("ondragover='{}'", val),
         Attr::OnDragStart(val) => format!("ondragstart='{}'", val),
         Attr::OnDrop(val) => format!("ondrop='{}'", val),
         Attr::OnDurationChange(val) => format!("ondurationchange='{}'", val),
         Attr::OnEmptied(val) => format!("onemptied='{}'", val),
         Attr::OnEnded(val) => format!("onended='{}'", val),
         Attr::OnError(val) => format!("onerror='{}'", val),
         Attr::OnFocus(val) => format!("onfocus='{}'", val),
         Attr::OnHashChange(val) => format!("onhashchange='{}'", val),
         Attr::OnInput(val) => format!("oninput='{}'", val),
         Attr::OnInvalid(val) => format!("oninvalid='{}'", val),
         Attr::OnKeyDown(val) => format!("onkeydown='{}'", val),
         Attr::OnKeyPress(val) => format!("onkeypress='{}'", val),
         Attr::OnKeyUp(val) => format!("onkeyup='{}'", val),
         Attr::OnLoad(val) => format!("onload='{}'", val),
         Attr::OnLoadedData(val) => format!("onloadeddata='{}'", val),
         Attr::OnLoadedMetadata(val) => format!("onloadedmetadata='{}'", val),
         Attr::OnLoadStart(val) => format!("onloadstart='{}'", val),
         Attr::OnMouseDown(val) => format!("onmousedown='{}'", val),
         Attr::OnMouseMove(val) => format!("onmousemove='{}'", val),
         Attr::OnMouseOut(val) => format!("onmouseout='{}'", val),
         Attr::OnMouseOver(val) => format!("onmouseover='{}'", val),
         Attr::OnMouseUp(val) => format!("onmouseup='{}'", val),
         Attr::OnMouseWheel(val) => format!("onmousewheel='{}'", val),
         Attr::OnOffline(val) => format!("onoffline='{}'", val),
         Attr::OnOnline(val) => format!("ononline='{}'", val),
         Attr::OnPageHide(val) => format!("onpagehide='{}'", val),
         Attr::OnPageShow(val) => format!("onpageshow='{}'", val),
         Attr::OnPaste(val) => format!("onpaste='{}'", val),
         Attr::OnPause(val) => format!("onpause='{}'", val),
         Attr::OnPlay(val) => format!("onplay='{}'", val),
         Attr::OnPlaying(val) => format!("onplaying='{}'", val),
         Attr::OnPopState(val) => format!("onpopstate='{}'", val),
         Attr::OnProgress(val) => format!("onprogress='{}'", val),
         Attr::OnRateChange(val) => format!("onratechange='{}'", val),
         Attr::OnReset(val) => format!("onreset='{}'", val),
         Attr::OnResize(val) => format!("onresize='{}'", val),
         Attr::OnScroll(val) => format!("onscroll='{}'", val),
         Attr::OnSearch(val) => format!("onsearch='{}'", val),
         Attr::OnSeeked(val) => format!("onseeked='{}'", val),
         Attr::OnSeeking(val) => format!("onseeking='{}'", val),
         Attr::OnSelect(val) => format!("onselect='{}'", val),
         Attr::OnStalled(val) => format!("onstalled='{}'", val),
         Attr::OnStorage(val) => format!("onstorage='{}'", val),
         Attr::OnSubmit(val) => format!("onsubmit='{}'", val),
         Attr::OnSuspend(val) => format!("onsuspend='{}'", val),
         Attr::OnTimeUpdate(val) => format!("ontimeupdate='{}'", val),
         Attr::OnToggle(val) => format!("ontoggle='{}'", val),
         Attr::OnUnload(val) => format!("onunload='{}'", val),
         Attr::OnVolumeChange(val) => format!("onvolumechange='{}'", val),
         Attr::OnWaiting(val) => format!("onwaiting='{}'", val),
         Attr::OnWheel(val) => format!("onwheel='{}'", val),
         Attr::Open(val) => format!("open='{}'", val),
         Attr::Optimum(val) => format!("optimum='{}'", val),
         Attr::Pattern(val) => format!("pattern='{}'", val),
         Attr::Placeholder(val) => format!("placeholder='{}'", val),
         Attr::Poster(val) => format!("poster='{}'", val),
         Attr::Preload(val) => format!("preload='{}'", val),
         Attr::ReadOnly(val) => format!("readonly='{}'", val),
         Attr::Relative(val) => format!("rel='{}'", val),
         Attr::Required(val) => format!("required='{}'", val),
         Attr::Reversed(val) => format!("reversed='{}'", val),
         Attr::Rows(val) => format!("rows='{}'", val),
         Attr::RowSpan(val) => format!("rowspan='{}'", val),
         Attr::Sandbox(val) => format!("sandbox='{}'", val),
         Attr::Scope(val) => format!("scope='{}'", val),
         Attr::Selected(val) => format!("selected='{}'", val),
         Attr::Shape(val) => format!("shape='{}'", val),
         Attr::Size(val) => format!("size='{}'", val),
         Attr::Sizes(val) => format!("sizes='{}'", val),
         Attr::Span(val) => format!("span='{}'", val),
         Attr::Spellcheck(val) => format!("spellcheck='{}'", val),
         Attr::Source(val) => format!("src='{}'", val),
         Attr::SourceDoc(val) => format!("srcdoc='{}'", val),
         Attr::SourceLanguage(val) => format!("srclang='{}'", val),
         Attr::SourceSet(val) => format!("srcset='{}'", val),
         Attr::Start(val) => format!("start='{}'", val),
         Attr::Step(val) => format!("step='{}'", val),
         Attr::Style(val) => format!("style='{}'", val),
         Attr::TabIndex(val) => format!("tabindex='{}'", val),
         Attr::Target(val) => format!("target='{}'", val),
         Attr::Title(val) => format!("title='{}'", val),
         Attr::Translate(val) => format!("translate='{}'", val),
         Attr::Type(val) => format!("type='{}'", val),
         Attr::UseMap(val) => format!("usemap='{}'", val),
         Attr::Value(val) => format!("value='{}'", val),
         Attr::Width(val) => format!("width='{}'", val),
         Attr::Wrap(val) => format!("wrap='{}'", val),
      }.into()
   }
}

impl std::fmt::Display for Attr {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "{}", self.into_string())
   }
}