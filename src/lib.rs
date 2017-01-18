//! Extract data from documentation strings.
//!
//! The expected format is described [here][1].
//!
//! [1]: https://scribbles.pascalhertleif.de/machine-readable-inline-markdown-code-cocumentation.html

#![deny(missing_docs, warnings, unsafe_code, missing_debug_implementations)]

extern crate pulldown_cmark;
#[macro_use] extern crate quick_error;

use pulldown_cmark::Parser;

mod types;
mod errors;
mod to_md;
mod extractors;

mod offset_parser;

pub use errors::ParseError;
pub use types::*;

/// Parse documentation and extract data
///
/// # Parameters
///
/// - `md`: Markdown string, needs to be parseable by `pulldown-cmark`
///
/// # Returns
///
/// A `Result`, which is either
///
/// - `Ok(DocBlock)`: A type that contains all extracted information (including
///     all unknown sections as `Custom` sections).
/// - `Err(ParseError)`: The first encountered error while parsing the
///     documentation string.
///
/// # Examples
///
/// Please excuse the weird way the input is formatting in this example.
/// Embedding Markdown strings in Rust code examples, which are just code blocks
/// in Markdown documentation strings inside a Rust program is kinda hard:
/// Rustdoc treads `#` at the beginning of code example line as a sign it
/// should omit the line from output. Sadly, this means I can't write Markdown
/// headlines as usual.
///
/// ```rust
/// # use self::docstrings::*;
/// assert_eq!(parse_md_docblock(
///     "Lorem ipsum\n\nDolor sit amet.\n\n# Parameters\n\n- `param1`: Foo\n- `param2`: Bar\n"
/// ).unwrap(),
///     DocBlock {
///         teaser: WithOffset::new("Lorem ipsum".into(), 0),
///         description: Some(WithOffset::new("Dolor sit amet.".into(), 12)),
///         sections: vec![
///             WithOffset::new(DocSection::Parameters(vec![
///                 ("param1".into(), "Foo".into()),
///                 ("param2".into(), "Bar".into())
///             ]), 32)
///         ]
///     }
/// );
/// ```
pub fn parse_md_docblock(md: &str) -> Result<DocBlock, ParseError> {
    let parser = offset_parser::OffsetParser(Parser::new(md));
    let mut md_events = parser.peekable();

    Ok(DocBlock {
        teaser: try!(extractors::teaser(&mut md_events)),
        description: try!(extractors::description(&mut md_events)),
        sections: try!(extractors::sections(&mut md_events)),
    })
}
