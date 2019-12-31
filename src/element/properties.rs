#[derive(Clone)]
pub struct Prop {
   name: String,
   value: String,
}

impl Prop {
   pub fn new<N: Into<String>>(name: N, value: String) -> Self {
      Prop {
         name: name.into(),
         value: value
      }
   }
   
   pub fn value(&mut self, value: String) {
      self.value = value;
   }
}

impl std::fmt::Display for Prop {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(
         f, 
         "{}: {};",
         self.name,
         self.value
      )
   }
}

#[derive(Clone)]
pub enum PropName {
   AlignContent,
   AlignItems,
   AlignSelf,
   All,
   Animation,
   AnimationDelay,
   AnimationDirection,
   AnimationDuration,
   AnimationFillMode,
   AnimationIterationCount,
   AnimationName,
   AnimationPlayState,
   AnimationTimingFunction,
   BackfaceVisibility,
   Background,
   BackgroundAttachment,
   BackgroundBlendMode,
   BackgroundClip,
   BackgroundColor,
   BackgroundImage,
   BackgroundOrigin,
   BackgroundPosition,
   BackgroundRepeat,
   BackgroundSize,
   Border,
   BorderBottom,
   BorderBottomColor,
   BorderBottomLeftRadius,
   BorderBottomRightRadius,
   BorderBottomStyle,
   BorderBottomWidth,
   BorderCollapse,
   BorderColor,
   BorderImage,
   BorderImageOutset,
   BorderImageRepeat,
   BorderImageSlice,
   BorderImageSource,
   BorderImageWidth,
   BorderLeft,
   BorderLeftColor,
   BorderLeftStyle,
   BorderLeftWidth,
   BorderRadius,
   BorderRight,
   BorderRightColor,
   BorderRightStyle,
   BorderRightWidth,
   BorderSpacing,
   BorderStyle,
   BorderTop,
   BorderTopColor,
   BorderTopLeftRadius,
   BorderTopRightRadius,
   BorderTopStyle,
   BorderTopWidth,
   BorderWidth,
   Bottom,
   BoxDecorationBreak,
   BoxShadow,
   BoxSizing,
   BreakAfter,
   BreakBefore,
   BreakInside,
   CaptionSide,
   CaretColor,
   Clear,
   Clip,
   Color,
   ColumnCount,
   ColumnFill,
   ColumnGap,
   ColumnRule,
   ColumnRuleColor,
   ColumnRuleStyle,
   ColumnRuleWidth,
   ColumnSpan,
   ColumnWidth,
   Columns,
   Content,
   CounterIncrement,
   CounterReset,
   Cursor,
   Direction,
   Display,
   EmptyCells,
   Filter,
   Flex,
   FlexBasis,
   FlexDirection,
   FlexFlow,
   FlexGrow,
   FlexShrink,
   FlexWrap,
   Float,
   Font,
   FontFamily,
   FontFeatureSettings,
   FontKerning,
   FontLanguageOverride,
   FontSize,
   FontSizeAdjust,
   FontStretch,
   FontStyle,
   FontSynthesis,
   FontVariant,
   FontVariantAlternates,
   FontVariantCaps,
   FontVariantEastAsian,
   FontVariantLigatures,
   FontVariantNumeric,
   FontVariantPosition,
   FontWeight,
   Grid,
   GridArea,
   GridAutoColumns,
   GridAutoFlow,
   GridAutoRows,
   GridColumn,
   GridColumnEnd,
   GridColumnGap,
   GridColumnStart,
   GridGap,
   GridRow,
   GridRowEnd,
   GridRowGap,
   GridRowStart,
   GridTemplate,
   GridTemplateAreas,
   GridTemplateColumns,
   GridTemplateRows,
   HangingPunctuation,
   Height,
   Hyphens,
   ImageRendering,
   Isolation,
   JustifyContent,
   Left,
   LetterSpacing,
   LineBreak,
   LineHeight,
   ListStyle,
   ListStyleImage,
   ListStylePosition,
   ListStyleType,
   Margin,
   MarginBottom,
   MarginLeft,
   MarginRight,
   MarginTop,
   MaxHeight,
   MaxWidth,
   MinHeight,
   MinWidth,
   MixBlendMode,
   ObjectFit,
   ObjectPosition,
   Opacity,
   Order,
   Orphans,
   Outline,
   OutlineColor,
   OutlineOffset,
   OutlineStyle,
   OutlineWidth,
   Overflow,
   OverflowWrap,
   OverflowX,
   OverflowY,
   Padding,
   PaddingBottom,
   PaddingLeft,
   PaddingRight,
   PaddingTop,
   PageBreakAfter,
   PageBreakBefore,
   PageBreakInside,
   Perspective,
   PerspectiveOrigin,
   PointerEvents,
   Position,
   Quotes,
   Resize,
   Right,
   ScrollBehavior,
   TabSize,
   TableLayout,
   TextAlign,
   TextAlignLast,
   TextCombineUpright,
   TextDecoration,
   TextDecorationColor,
   TextDecorationLine,
   TextDecorationStyle,
   TextIndent,
   TextJustify,
   TextOrientation,
   TextOverflow,
   TextShadow,
   TextTransform,
   TextUnderlinePosition,
   Top,
   Transform,
   TransformOrigin,
   TransformStyle,
   Transition,
   TransitionDelay,
   TransitionDuration,
   TransitionProperty,
   TransitionTimingFunction,
   UnicodeBidi,
   UserSelect,
   VerticalAlign,
   Visibility,
   WhiteSpace,
   Widows,
   Width,
   WordBreak,
   WordSpacing,
   WordWrap,
   WritingMode,
   ZIndex,
}

impl std::convert::Into<String> for PropName {
   fn into(self) -> String {
      match self {
         PropName::AlignContent => "align-content",
         PropName::AlignItems => "align-items",
         PropName::AlignSelf => "align-self",
         PropName::All => "all",
         PropName::Animation => "animation",
         PropName::AnimationDelay => "animation-delay",
         PropName::AnimationDirection => "animation-direction",
         PropName::AnimationDuration => "animation-duration",
         PropName::AnimationFillMode => "animation-fill-mode",
         PropName::AnimationIterationCount => "animation-iteration-count",
         PropName::AnimationName => "animation-name",
         PropName::AnimationPlayState => "animation-play-state",
         PropName::AnimationTimingFunction => "animation-timing-function",
         PropName::BackfaceVisibility => "backface-visibility",
         PropName::Background => "background",
         PropName::BackgroundAttachment => "background-attachment",
         PropName::BackgroundBlendMode => "background-blend-mode",
         PropName::BackgroundClip => "background-clip",
         PropName::BackgroundColor => "background-color",
         PropName::BackgroundImage => "background-image",
         PropName::BackgroundOrigin => "background-origin",
         PropName::BackgroundPosition => "background-position",
         PropName::BackgroundRepeat => "background-repeat",
         PropName::BackgroundSize => "background-size",
         PropName::Border => "border",
         PropName::BorderBottom => "border-bottom",
         PropName::BorderBottomColor => "border-bottom-color",
         PropName::BorderBottomLeftRadius => "border-bottom-left-radius",
         PropName::BorderBottomRightRadius => "border-bottom-right-radius",
         PropName::BorderBottomStyle => "border-bottom-style",
         PropName::BorderBottomWidth => "border-bottom-width",
         PropName::BorderCollapse => "border-collapse",
         PropName::BorderColor => "border-color",
         PropName::BorderImage => "border-image",
         PropName::BorderImageOutset => "border-image-outset",
         PropName::BorderImageRepeat => "border-image-repeat",
         PropName::BorderImageSlice => "border-image-slice",
         PropName::BorderImageSource => "border-image-source",
         PropName::BorderImageWidth => "border-image-width",
         PropName::BorderLeft => "border-left",
         PropName::BorderLeftColor => "border-left-color",
         PropName::BorderLeftStyle => "border-left-style",
         PropName::BorderLeftWidth => "border-left-width",
         PropName::BorderRadius => "border-radius",
         PropName::BorderRight => "border-right",
         PropName::BorderRightColor => "border-right-color",
         PropName::BorderRightStyle => "border-right-style",
         PropName::BorderRightWidth => "border-right-width",
         PropName::BorderSpacing => "border-spacing",
         PropName::BorderStyle => "border-style",
         PropName::BorderTop => "border-top",
         PropName::BorderTopColor => "border-top-color",
         PropName::BorderTopLeftRadius => "border-top-left-radius",
         PropName::BorderTopRightRadius => "border-top-right-radius",
         PropName::BorderTopStyle => "border-top-style",
         PropName::BorderTopWidth => "border-top-width",
         PropName::BorderWidth => "border-width",
         PropName::Bottom => "bottom",
         PropName::BoxDecorationBreak => "box-decoration-break",
         PropName::BoxShadow => "box-shadow",
         PropName::BoxSizing => "box-sizing",
         PropName::BreakAfter => "break-after",
         PropName::BreakBefore => "break-before",
         PropName::BreakInside => "break-inside",
         PropName::CaptionSide => "caption-side",
         PropName::CaretColor => "caret-color",
         PropName::Clear => "clear",
         PropName::Clip => "clip",
         PropName::Color => "color",
         PropName::ColumnCount => "column-count",
         PropName::ColumnFill => "column-fill",
         PropName::ColumnGap => "column-gap",
         PropName::ColumnRule => "column-rule",
         PropName::ColumnRuleColor => "column-rule-color",
         PropName::ColumnRuleStyle => "column-rule-style",
         PropName::ColumnRuleWidth => "column-rule-width",
         PropName::ColumnSpan => "column-span",
         PropName::ColumnWidth => "column-width",
         PropName::Columns => "columns",
         PropName::Content => "content",
         PropName::CounterIncrement => "counter-increment",
         PropName::CounterReset => "counter-reset",
         PropName::Cursor => "cursor",
         PropName::Direction => "direction",
         PropName::Display => "display",
         PropName::EmptyCells => "empty-cells",
         PropName::Filter => "filter",
         PropName::Flex => "flex",
         PropName::FlexBasis => "flex-basis",
         PropName::FlexDirection => "flex-direction",
         PropName::FlexFlow => "flex-flow",
         PropName::FlexGrow => "flex-grow",
         PropName::FlexShrink => "flex-shrink",
         PropName::FlexWrap => "flex-wrap",
         PropName::Float => "float",
         PropName::Font => "font",
         PropName::FontFamily => "font-family",
         PropName::FontFeatureSettings => "font-feature-settings",
         PropName::FontKerning => "font-kerning",
         PropName::FontLanguageOverride => "font-language-override",
         PropName::FontSize => "font-size",
         PropName::FontSizeAdjust => "font-size-adjust",
         PropName::FontStretch => "font-stretch",
         PropName::FontStyle => "font-style",
         PropName::FontSynthesis => "font-synthesis",
         PropName::FontVariant => "font-variant",
         PropName::FontVariantAlternates => "font-variant-alternates",
         PropName::FontVariantCaps => "font-variant-caps",
         PropName::FontVariantEastAsian => "font-variant-east-asian",
         PropName::FontVariantLigatures => "font-variant-ligatures",
         PropName::FontVariantNumeric => "font-variant-numeric",
         PropName::FontVariantPosition => "font-variant-position",
         PropName::FontWeight => "font-weight",
         PropName::Grid => "grid",
         PropName::GridArea => "grid-area",
         PropName::GridAutoColumns => "grid-auto-columns",
         PropName::GridAutoFlow => "grid-auto-flow",
         PropName::GridAutoRows => "grid-auto-rows",
         PropName::GridColumn => "grid-column",
         PropName::GridColumnEnd => "grid-column-end",
         PropName::GridColumnGap => "grid-column-gap",
         PropName::GridColumnStart => "grid-column-start",
         PropName::GridGap => "grid-gap",
         PropName::GridRow => "grid-row",
         PropName::GridRowEnd => "grid-row-end",
         PropName::GridRowGap => "grid-row-gap",
         PropName::GridRowStart => "grid-row-start",
         PropName::GridTemplate => "grid-template",
         PropName::GridTemplateAreas => "grid-template-areas",
         PropName::GridTemplateColumns => "grid-template-columns",
         PropName::GridTemplateRows => "grid-template-rows",
         PropName::HangingPunctuation => "hanging-punctuation",
         PropName::Height => "height",
         PropName::Hyphens => "hyphens",
         PropName::ImageRendering => "image-rendering",
         PropName::Isolation => "isolation",
         PropName::JustifyContent => "justify-content",
         PropName::Left => "left",
         PropName::LetterSpacing => "letter-spacing",
         PropName::LineBreak => "line-break",
         PropName::LineHeight => "line-height",
         PropName::ListStyle => "list-style",
         PropName::ListStyleImage => "list-style-image",
         PropName::ListStylePosition => "list-style-position",
         PropName::ListStyleType => "list-style-type",
         PropName::Margin => "margin",
         PropName::MarginBottom => "margin-bottom",
         PropName::MarginLeft => "margin-left",
         PropName::MarginRight => "margin-right",
         PropName::MarginTop => "margin-top",
         PropName::MaxHeight => "max-height",
         PropName::MaxWidth => "max-width",
         PropName::MinHeight => "min-height",
         PropName::MinWidth => "min-width",
         PropName::MixBlendMode => "mix-blend-mode",
         PropName::ObjectFit => "object-fit",
         PropName::ObjectPosition => "object-position",
         PropName::Opacity => "opacity",
         PropName::Order => "order",
         PropName::Orphans => "orphans",
         PropName::Outline => "outline",
         PropName::OutlineColor => "outline-color",
         PropName::OutlineOffset => "outline-offset",
         PropName::OutlineStyle => "outline-style",
         PropName::OutlineWidth => "outline-width",
         PropName::Overflow => "overflow",
         PropName::OverflowWrap => "overflow-wrap",
         PropName::OverflowX => "overflow-x",
         PropName::OverflowY => "overflow-y",
         PropName::Padding => "padding",
         PropName::PaddingBottom => "padding-bottom",
         PropName::PaddingLeft => "padding-left",
         PropName::PaddingRight => "padding-right",
         PropName::PaddingTop => "padding-top",
         PropName::PageBreakAfter => "page-break-after",
         PropName::PageBreakBefore => "page-break-before",
         PropName::PageBreakInside => "page-break-inside",
         PropName::Perspective => "perspective",
         PropName::PerspectiveOrigin => "perspective-origin",
         PropName::PointerEvents => "pointer-events",
         PropName::Position => "position",
         PropName::Quotes => "quotes",
         PropName::Resize => "resize",
         PropName::Right => "right",
         PropName::ScrollBehavior => "scroll-behavior",
         PropName::TabSize => "tab-size",
         PropName::TableLayout => "table-layout",
         PropName::TextAlign => "text-align",
         PropName::TextAlignLast => "text-align-last",
         PropName::TextCombineUpright => "text-combine-upright",
         PropName::TextDecoration => "text-decoration",
         PropName::TextDecorationColor => "text-decoration-color",
         PropName::TextDecorationLine => "text-decoration-line",
         PropName::TextDecorationStyle => "text-decoration-style",
         PropName::TextIndent => "text-indent",
         PropName::TextJustify => "text-justify",
         PropName::TextOrientation => "text-orientation",
         PropName::TextOverflow => "text-overflow",
         PropName::TextShadow => "text-shadow",
         PropName::TextTransform => "text-transform",
         PropName::TextUnderlinePosition => "text-underline-position",
         PropName::Top => "top",
         PropName::Transform => "transform",
         PropName::TransformOrigin => "transform-origin",
         PropName::TransformStyle => "transform-style",
         PropName::Transition => "transition",
         PropName::TransitionDelay => "transition-delay",
         PropName::TransitionDuration => "transition-duration",
         PropName::TransitionProperty => "transition-property",
         PropName::TransitionTimingFunction => "transition-timing-function",
         PropName::UnicodeBidi => "unicode-bidi",
         PropName::UserSelect => "user-select",
         PropName::VerticalAlign => "vertical-align",
         PropName::Visibility => "visibility",
         PropName::WhiteSpace => "white-space",
         PropName::Widows => "widows",
         PropName::Width => "width",
         PropName::WordBreak => "word-break",
         PropName::WordSpacing => "word-spacing",
         PropName::WordWrap => "word-wrap",
         PropName::WritingMode => "writing-mode",
         PropName::ZIndex => "z-index",
      }.into()
   }
}