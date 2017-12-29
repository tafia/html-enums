//! All HTML5 Tags

/// All valid HTML5 tags, grouped by category
pub enum Tag {
    /// html
    Root,
    /// metadata
    Meta(Metadata),
    /// section
    Section(Section),
    /// grouping
    Grouping(Grouping),
    /// texts
    Text(Text),
    Edit(Edit),
    Embedded(Embedded),
    Table(Table),
    Form(Form),
    Script(Script),
    Unknown(Vec<u8>),
}

/// Metadata tags
pub enum Metadata {
    /// head
    Head,
    /// title
    Title,
    /// base
    Base,
    /// link
    Link,
    /// meta
    Meta,
    /// style
    Style,
}

/// Section tags
pub enum Section {
    Body,
    Article,
    Section,
    Nav,
    Aside,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Header,
    Footer,
    Address,
}

/// Grouping tags
pub enum Grouping {
    P,
    Pre,
    BlockQuote,
    Ol,
    Ul,
    Li,
    Dl,
    Dt,
    Dd,
    Figure,
    FigCaption,
    Div,
    Main,
    Hr,
}

/// Text tags
pub enum Text {
    A,
    Em,
    Strong,
    Cite,
    Q,
    Dfn,
    Abbr,
    Data,
    Time,
    Code,
    Var,
    Samp,
    Kbd,
    Mark,
    Ruby,
    Rb,
    Rt,
    Rp,
    Rtc,
    Bdi,
    Bdo,
    Span,
    Br,
    Wbr,
    Small,
    I,
    B,
    U,
    S,
    Sub,
    Sup,
}

pub enum Edit {
    Ins,
    Del,
}

pub enum Embedded {
    Img,
    Embed,
    Object,
    Param,
    Video,
    Audio,
    Source,
    Track,
    Map,
    Area,
    IFrame,
}

pub enum Table {
    Table,
    Tr,
    Td,
    Th,
    Caption,
    TBody,
    THead,
    TFoot,
    ColGroup,
    Col,
}

pub enum Form {
    Form,
    Input,
    TextArea,
    Select,
    Option,
    OptGroup,
    DataList,
    Label,
    FieldSet,
    Legend,
    Button,
    Output,
    Progress,
    Meter,
    KeyGen,
}

pub enum Script {
    Script,
    NoScript,
    Template,
    Canvas,
}

impl<'a> From<&'a [u8]> for Tag {
    fn from(b: &'a [u8]) -> Tag {
        match b {
            b"html" => Tag::Root,
            b"head" => Tag::Meta(Metadata::Head),
            b"title" => Tag::Meta(Metadata::Title),
            b"base" => Tag::Meta(Metadata::Base),
            b"link" => Tag::Meta(Metadata::Link),
            b"meta" => Tag::Meta(Metadata::Meta),
            b"style" => Tag::Meta(Metadata::Style),
            b"body" => Tag::Section(Section::Body),
            b"article" => Tag::Section(Section::Article),
            b"section" => Tag::Section(Section::Section),
            b"nav" => Tag::Section(Section::Nav),
            b"aside" => Tag::Section(Section::Aside),
            b"h1" => Tag::Section(Section::H1),
            b"h2" => Tag::Section(Section::H2),
            b"h3" => Tag::Section(Section::H3),
            b"h4" => Tag::Section(Section::H4),
            b"h5" => Tag::Section(Section::H5),
            b"h6" => Tag::Section(Section::H6),
            b"header" => Tag::Section(Section::Header),
            b"footer" => Tag::Section(Section::Footer),
            b"address" => Tag::Section(Section::Address),
            b"p" => Tag::Grouping(Grouping::P),
            b"pre" => Tag::Grouping(Grouping::Pre),
            b"blockquote" => Tag::Grouping(Grouping::BlockQuote),
            b"ol" => Tag::Grouping(Grouping::Ol),
            b"ul" => Tag::Grouping(Grouping::Ul),
            b"li" => Tag::Grouping(Grouping::Li),
            b"dl" => Tag::Grouping(Grouping::Dl),
            b"dt" => Tag::Grouping(Grouping::Dt),
            b"dd" => Tag::Grouping(Grouping::Dd),
            b"figure" => Tag::Grouping(Grouping::Figure),
            b"figcaption" => Tag::Grouping(Grouping::FigCaption),
            b"div" => Tag::Grouping(Grouping::Div),
            b"main" => Tag::Grouping(Grouping::Main),
            b"hr" => Tag::Grouping(Grouping::Hr),
            b"a" => Tag::Text(Text::A),
            b"em" => Tag::Text(Text::Em),
            b"strong" => Tag::Text(Text::Strong),
            b"cite" => Tag::Text(Text::Cite),
            b"q" => Tag::Text(Text::Q),
            b"dfn" => Tag::Text(Text::Dfn),
            b"abbr" => Tag::Text(Text::Abbr),
            b"data" => Tag::Text(Text::Data),
            b"time" => Tag::Text(Text::Time),
            b"code" => Tag::Text(Text::Code),
            b"var" => Tag::Text(Text::Var),
            b"samp" => Tag::Text(Text::Samp),
            b"kbd" => Tag::Text(Text::Kbd),
            b"mark" => Tag::Text(Text::Mark),
            b"ruby" => Tag::Text(Text::Ruby),
            b"rb" => Tag::Text(Text::Rb),
            b"rt" => Tag::Text(Text::Rt),
            b"rp" => Tag::Text(Text::Rp),
            b"rtc" => Tag::Text(Text::Rtc),
            b"bdi" => Tag::Text(Text::Bdi),
            b"bdo" => Tag::Text(Text::Bdo),
            b"span" => Tag::Text(Text::Span),
            b"br" => Tag::Text(Text::Br),
            b"wbr" => Tag::Text(Text::Wbr),
            b"small" => Tag::Text(Text::Small),
            b"i" => Tag::Text(Text::I),
            b"b" => Tag::Text(Text::B),
            b"u" => Tag::Text(Text::U),
            b"s" => Tag::Text(Text::S),
            b"sub" => Tag::Text(Text::Sub),
            b"sup" => Tag::Text(Text::Sup),
            b"ins" => Tag::Edit(Edit::Ins),
            b"del" => Tag::Edit(Edit::Del),
            b"img" => Tag::Embedded(Embedded::Img),
            b"embed" => Tag::Embedded(Embedded::Embed),
            b"object" => Tag::Embedded(Embedded::Object),
            b"param" => Tag::Embedded(Embedded::Param),
            b"video" => Tag::Embedded(Embedded::Video),
            b"audio" => Tag::Embedded(Embedded::Audio),
            b"source" => Tag::Embedded(Embedded::Source),
            b"track" => Tag::Embedded(Embedded::Track),
            b"map" => Tag::Embedded(Embedded::Map),
            b"area" => Tag::Embedded(Embedded::Area),
            b"iframe" => Tag::Embedded(Embedded::IFrame),
            b"table" => Tag::Table(Table::Table),
            b"tr" => Tag::Table(Table::Tr),
            b"td" => Tag::Table(Table::Td),
            b"th" => Tag::Table(Table::Th),
            b"caption" => Tag::Table(Table::Caption),
            b"tbody" => Tag::Table(Table::TBody),
            b"thead" => Tag::Table(Table::THead),
            b"tfoot" => Tag::Table(Table::TFoot),
            b"colgroup" => Tag::Table(Table::ColGroup),
            b"col" => Tag::Table(Table::Col),
            b"form" => Tag::Form(Form::Form),
            b"input" => Tag::Form(Form::Input),
            b"textarea" => Tag::Form(Form::TextArea),
            b"select" => Tag::Form(Form::Select),
            b"option" => Tag::Form(Form::Option),
            b"optgroup" => Tag::Form(Form::OptGroup),
            b"datalist" => Tag::Form(Form::DataList),
            b"label" => Tag::Form(Form::Label),
            b"fieldset" => Tag::Form(Form::FieldSet),
            b"legend" => Tag::Form(Form::Legend),
            b"button" => Tag::Form(Form::Button),
            b"output" => Tag::Form(Form::Output),
            b"progress" => Tag::Form(Form::Progress),
            b"meter" => Tag::Form(Form::Meter),
            b"keygen" => Tag::Form(Form::KeyGen),
            b"script" => Tag::Script(Script::Script),
            b"noscript" => Tag::Script(Script::NoScript),
            b"template" => Tag::Script(Script::Template),
            b"canvas" => Tag::Script(Script::Canvas),
            _ => Tag::Unknown(b.into()),
        }
    }
}

impl<'a> From<&'a str> for Tag {
    fn from(b: &'a str) -> Tag {
        Tag::from(b.as_bytes())
    }
}
