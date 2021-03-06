mod bibtex;
mod language;
mod latex;
mod lsp_kind;
mod text;

pub use self::bibtex::*;
pub use self::language::*;
pub use self::latex::*;
pub use self::lsp_kind::*;
pub use self::text::*;

use texlab_distro::{Language, Resolver};
use texlab_protocol::Uri;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SyntaxTree {
    Latex(Box<LatexSyntaxTree>),
    Bibtex(Box<BibtexSyntaxTree>),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SyntaxTreeContext<'a> {
    pub resolver: &'a Resolver,
    pub uri: &'a Uri,
}

impl SyntaxTree {
    pub fn parse(context: SyntaxTreeContext, text: &str, language: Language) -> Self {
        match language {
            Language::Latex => SyntaxTree::Latex(Box::new(LatexSyntaxTree::parse(context, text))),
            Language::Bibtex => SyntaxTree::Bibtex(Box::new(text.into())),
        }
    }
}
