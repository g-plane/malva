//! Format range implementation for Malva.
//!
//! This module provides the ability to format a specific range of CSS code
//! instead of the entire file. This is useful for editor integrations where
//! a user selects a portion of code and wants to format only that selection.

use crate::{
    config::FormatOptions, ctx::Ctx, doc_gen::DocGen, error::Error, line_bounds::LineBounds,
    state::State,
};
use raffia::{
    ParserBuilder, ParserOptions, Spanned, Syntax,
    ast::{Declaration, QualifiedRule, Statement, Stylesheet},
    pos::Span,
    token::Comment,
};
use std::ops::Range;
use tiny_pretty::{Doc, IndentKind, PrintOptions};

/// Result of formatting a range.
#[derive(Debug, Clone)]
pub struct FormatRangeResult {
    /// The formatted code.
    pub code: String,
    /// The actual range in source that should be replaced.
    /// This may be different from the input range because:
    /// - The range may be expanded to include complete statements
    /// - The range may be shrunk to avoid breaking incomplete statements
    pub range: Range<usize>,
}

/// Format a specific range of CSS code.
///
/// # Arguments
///
/// * `source` - The full source code
/// * `range` - The range to format (in bytes)
/// * `syntax` - The CSS syntax (Css, Scss, Sass, Less)
/// * `options` - Formatting options
///
/// # Returns
///
/// Returns `FormatRangeResult` containing the formatted code and the actual range
/// that should be replaced in the source.
///
/// # Errors
///
/// Returns an error if:
/// - The range is outside of the source file bounds
/// - The source code cannot be parsed
pub fn format_range(
    source: &str,
    range: Range<usize>,
    syntax: Syntax,
    options: &FormatOptions,
) -> Result<FormatRangeResult, Error> {
    // 1. Validate range
    if range.start > source.len() || range.end > source.len() {
        return Err(Error::RangeOutOfBounds {
            range,
            source_len: source.len(),
        });
    }

    if range.is_empty() {
        return Ok(FormatRangeResult {
            code: String::new(),
            range,
        });
    }

    // 2. Parse source
    let line_bounds = LineBounds::new(source);
    let mut comments = vec![];
    let mut parser = ParserBuilder::new(source)
        .syntax(syntax)
        .comments(&mut comments)
        .options(ParserOptions {
            try_parsing_value_in_custom_property: true,
            tolerate_semicolon_in_sass: true,
        })
        .build();

    let stylesheet = match parser.parse::<Stylesheet>() {
        Ok(stylesheet) => stylesheet,
        Err(error) => {
            let (line, col) = line_bounds.get_line_col(error.span.start);
            return Err(Error::Parser(error, line, col));
        }
    };

    // 3. Find the node(s) that contain the range and the reference span for indent calculation
    let (range_node, ref_span) = find_range_node(&stylesheet, &range, source);

    // 4. Calculate base indentation from reference span
    let base_indent = calculate_base_indent(ref_span, source, options);

    // 5. Format the node(s)
    let formatted_code = format_node_with_indent(
        &range_node,
        &comments,
        Some(source),
        line_bounds,
        syntax,
        options,
        base_indent,
    );

    // 6. Determine the actual range to replace
    let actual_span = range_node.span();

    Ok(FormatRangeResult {
        code: formatted_code,
        range: Range {
            start: actual_span.start,
            end: actual_span.end,
        },
    })
}

/// Represents a node or a list of sibling nodes that should be formatted together.
enum RangeNode<'a, 's> {
    /// No formatable node found in the range.
    Empty(usize),
    /// A single statement to format.
    SingleStatement(&'a Statement<'s>),
    /// A single declaration to format (inside a qualified rule).
    SingleDeclaration(&'a Declaration<'s>),
    /// Multiple sibling statements to format together.
    MultipleStatements(Vec<&'a Statement<'s>>),
    /// Multiple sibling declarations to format together (inside a qualified rule).
    MultipleDeclarations(Vec<&'a Declaration<'s>>),
}

impl RangeNode<'_, '_> {
    /// Get the span of this range node.
    fn span(&self) -> Span {
        match self {
            RangeNode::Empty(start) => Span {
                start: *start,
                end: *start,
            },
            RangeNode::SingleStatement(node) => node.span().clone(),
            RangeNode::SingleDeclaration(node) => node.span().clone(),
            RangeNode::MultipleStatements(nodes) => {
                let start = nodes.first().map_or(0, |n| n.span().start);
                let end = nodes.last().map_or(0, |n| n.span().end);
                Span { start, end }
            }
            RangeNode::MultipleDeclarations(nodes) => {
                let start = nodes.first().map_or(0, |n| n.span().start);
                let end = nodes.last().map_or(0, |n| n.span().end);
                Span { start, end }
            }
        }
    }
}

/// Find the node(s) that contain(s) the given range.
///
/// Returns a tuple of (`RangeNode`, `reference_span`) where:
/// - `RangeNode`: the node(s) to format
/// - `reference_span`: the span of the previous sibling node (or the first node itself if it's the first sibling)
///   This span is used to calculate the base indentation level.
///
/// Uses a two-pointer algorithm to find either:
/// - A single node that completely contains the range, or
/// - Multiple sibling nodes that together contain the range
///
/// For CSS, the minimum formatable unit is a "line-level" node:
/// - A Statement (rule, at-rule, etc.)
/// - A Declaration (inside a rule)
///
/// # Algorithm
///
/// 1. If the range covers the entire file, format everything
/// 2. Otherwise, search for statements that intersect with the range
/// 3. If a single statement fully contains the range:
///    - For `QualifiedRule`: search its declarations
///    - For other statements: format the entire statement
/// 4. If the range spans multiple statements: format all those statements
/// 5. Find the reference span for indent calculation:
///    - For a single node: use its previous sibling, or itself if it's the first
///    - For multiple nodes: use the previous sibling of the first node, or the first node itself if it's the first
fn find_range_node<'a, 's>(
    stylesheet: &'a Stylesheet<'s>,
    range: &Range<usize>,
    _source: &str,
) -> (RangeNode<'a, 's>, Span) {
    let statements = &stylesheet.statements;

    // Check if the range covers the entire file
    if range.start == 0 && range.end >= stylesheet.span().end {
        if let Some(first) = statements.first() {
            return (RangeNode::MultipleStatements(statements.iter().collect()), first.span().clone());
        }
        return (RangeNode::Empty(range.start), Span { start: 0, end: 0 });
    }

    // Find statements that intersect with the range
    let mut start_idx = None;
    let mut end_idx = None;

    for (idx, stmt) in statements.iter().enumerate() {
        let span = stmt.span();
        let stmt_range = Range {
            start: span.start,
            end: span.end,
        };

        if stmt_range.end > range.start && stmt_range.start < range.end {
            if start_idx.is_none() {
                start_idx = Some(idx);
            }
            end_idx = Some(idx);
        }
    }

    match (start_idx, end_idx) {
        (Some(start), Some(end)) => {
            let statements_slice = &statements[start..=end];

            if statements_slice.len() == 1 {
                let stmt = &statements_slice[0];

                if let Statement::QualifiedRule(rule) = stmt
                    && let Some(declarations) = try_narrow_to_declarations(rule, range)
                {
                    return declarations;
                }

                let ref_span = if start > 0 {
                    statements[start - 1].span().clone()
                } else {
                    stmt.span().clone()
                };

                return (RangeNode::SingleStatement(stmt), ref_span);
            }

            let ref_span = if start > 0 {
                statements[start - 1].span().clone()
            } else {
                statements[start].span().clone()
            };

            (RangeNode::MultipleStatements(statements_slice.iter().collect()), ref_span)
        }
        _ => (RangeNode::Empty(range.start), Span { start: range.start, end: range.start }),
    }
}

/// Try to narrow down a `QualifiedRule` to specific declarations.
///
/// Returns None if:
/// - The range doesn't fully contain any declaration
/// - We should format the entire rule instead
///
/// Returns Some((RangeNode, `reference_span`)) where `reference_span` is used for indent calculation.
fn try_narrow_to_declarations<'a, 's>(
    rule: &'a QualifiedRule<'s>,
    range: &Range<usize>,
) -> Option<(RangeNode<'a, 's>, Span)> {
    let block = &rule.block;

    if block.statements.is_empty() {
        return None;
    }

    let decls_in_block: Vec<_> = block
        .statements
        .iter()
        .filter_map(|stmt| match stmt {
            Statement::Declaration(decl) => Some(decl),
            _ => None,
        })
        .collect();

    if decls_in_block.is_empty() {
        return None;
    }

    let mut start_idx = None;
    let mut end_idx = None;

    for (idx, decl) in decls_in_block.iter().enumerate() {
        let span = decl.span();
        let decl_range = Range {
            start: span.start,
            end: span.end,
        };

        if decl_range.end > range.start && decl_range.start < range.end {
            if start_idx.is_none() {
                start_idx = Some(idx);
            }
            end_idx = Some(idx);
        }
    }

    match (start_idx, end_idx) {
        (Some(start), Some(end)) => {
            let decls: Vec<&Declaration> = decls_in_block[start..=end].to_vec();

            let ref_span = if start > 0 {
                decls_in_block[start - 1].span().clone()
            } else {
                decls_in_block[start].span().clone()
            };

            let range_node = if decls.len() == 1 {
                RangeNode::SingleDeclaration(decls[0])
            } else {
                RangeNode::MultipleDeclarations(decls)
            };

            Some((range_node, ref_span))
        }
        _ => None,
    }
}

/// Calculate the base indentation level from a reference span.
///
/// This function finds the line containing the span's start position and counts
/// the leading whitespace (indentation) before it. The span.start position
/// points to the first character of the node, so the indentation is the text
/// from the line start to span.start.
///
/// # Returns
///
/// Returns the base indentation level (in units of `indent_width`).
/// If the node is at the start of the file, returns 0.
fn calculate_base_indent(ref_span: Span, source: &str, options: &FormatOptions) -> usize {
    if ref_span.start == 0 {
        return 0;
    }

    let before_text = &source[..ref_span.start];

    let line_start = match before_text.rfind('\n') {
        Some(pos) => pos + 1,
        None => 0,
    };

    let indent_str = &before_text[line_start..];

    if options.layout.use_tabs {
        indent_str.chars().filter(|&c| c == '\t').count()
    } else {
        indent_str.chars().filter(|&c| c == ' ').count() / options.layout.indent_width
    }
}

/// Format a node (or multiple sibling nodes) with a specific base indentation.
fn format_node_with_indent<'s>(
    range_node: &RangeNode<'_, 's>,
    comments: &[Comment<'s>],
    source: Option<&'s str>,
    line_bounds: LineBounds,
    syntax: Syntax,
    options: &FormatOptions,
    base_indent: usize,
) -> String {
    let ctx = Ctx {
        source,
        syntax,
        options: &options.language,
        comments,
        indent_width: options.layout.indent_width,
        line_bounds,
    };

    let state = State {
        keep_decl_name_case: false,
        selector_override: crate::state::SelectorOverride::Unset,
    };

    // Generate doc for the range node(s)
    let doc = match range_node {
        RangeNode::Empty(_) => Doc::nil(),
        RangeNode::SingleStatement(stmt) => stmt.doc(&ctx, &state),
        RangeNode::SingleDeclaration(decl) => decl.doc(&ctx, &state),
        RangeNode::MultipleStatements(stmts) => {
            // For multiple statements, we need to join them with appropriate separators
            let mut docs = Vec::with_capacity(stmts.len() * 2);
            for (i, stmt) in stmts.iter().enumerate() {
                if i > 0 {
                    docs.push(Doc::hard_line());
                }
                docs.push(stmt.doc(&ctx, &state));
            }
            Doc::list(docs)
        }
        RangeNode::MultipleDeclarations(decls) => {
            // For multiple declarations, we need to join them with appropriate separators
            let mut docs = Vec::with_capacity(decls.len() * 2);
            for (i, decl) in decls.iter().enumerate() {
                if i > 0 {
                    docs.push(Doc::hard_line());
                }
                docs.push(decl.doc(&ctx, &state));
            }
            Doc::list(docs)
        }
    };

    // Wrap with base indent
    let doc = doc.nest(base_indent);

    // Print => doc
    tiny_pretty::print(
        &doc,
        &PrintOptions {
            indent_kind: if options.layout.use_tabs {
                IndentKind::Tab
            } else {
                IndentKind::Space
            },
            line_break: options.layout.line_break.clone().into(),
            width: options.layout.print_width,
            tab_size: options.layout.indent_width,
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_range() {
        let source = ".a { color: red; }";
        let result = format_range(source, 0..0, Syntax::Css, &FormatOptions::default());
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.code.is_empty());
        assert_eq!(result.range, 0..0);
    }

    #[test]
    fn test_range_out_of_bounds() {
        let source = ".a { color: red; }";
        let result = format_range(source, 0..1000, Syntax::Css, &FormatOptions::default());
        assert!(matches!(result, Err(Error::RangeOutOfBounds { .. })));
    }

    #[test]
    fn test_full_file_range() {
        let source = ".a { color: red; }";
        let result = format_range(
            source,
            0..source.len(),
            Syntax::Css,
            &FormatOptions::default(),
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_simple_declaration() {
        let source = ".a {\n  color: red;\n  font-size: 16px;\n}";
        let range = source.find("color").unwrap()..source.find("16px").unwrap();
        let result = format_range(source, range, Syntax::Css, &FormatOptions::default());
        assert!(result.is_ok());
        let result = result.unwrap();
        // Should format both declarations as they're in the same block
        // Note: formatted code will have newlines between declarations
        assert!(result.code.contains("color: red"));
        assert!(result.code.contains("font-size: 16px"));
    }
}
