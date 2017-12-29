// Generated from grammar.ron
use tree::{SyntaxKind, SyntaxInfo};

pub const IDENT: SyntaxKind = SyntaxKind(0);
pub const WHITESPACE: SyntaxKind = SyntaxKind(1);

static INFOS: [SyntaxInfo; 2] = [
    SyntaxInfo { name: "IDENT" },
    SyntaxInfo { name: "WHITESPACE" },
];

pub(crate) fn syntax_info(kind: SyntaxKind) -> &'static SyntaxInfo {
    &INFOS[kind.0 as usize]
}