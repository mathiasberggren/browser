// FIX (alivenotions): Currently using Rcdom but move away from that since it is for test only

use html5ever::{parse_document, tokenizer, tree_builder, ParseOpts, Parser};
extern crate markup5ever_rcdom as rcdom;

pub fn parse_html() -> Parser<rcdom::RcDom> {
    let html5opts = ParseOpts {
        tokenizer: tokenizer::TokenizerOpts::default(),
        tree_builder: tree_builder::TreeBuilderOpts::default(),
    };
    parse_document(rcdom::RcDom::default(), html5opts)
}
