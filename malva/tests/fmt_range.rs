use malva::{config::FormatOptions, format_range};
use raffia::{ParserBuilder, ParserOptions, Syntax, ast::Stylesheet};

#[test]
fn format_range_empty() {
    let input = "";
    let result = format_range(input, 0..0, Syntax::Css, &FormatOptions::default());
    assert!(result.is_ok());
    let result = result.unwrap();
    assert!(result.code.is_empty());
    assert_eq!(result.range, 0..0);
}

#[test]
fn format_range_full_file() {
    let input = ".a {\n  color: red;\n}";
    let result = format_range(
        input,
        0..input.len(),
        Syntax::Css,
        &FormatOptions::default(),
    );
    assert!(result.is_ok());
    let result = result.unwrap();
    assert!(!result.code.is_empty());
}

#[test]
fn format_range_single_declaration() {
    let input = ".a {\n  color: red;\n}";
    let start = input.find("color").unwrap();
    let end = input.find("red;").unwrap() + 4; // +4 for "red;"
    let result = format_range(input, start..end, Syntax::Css, &FormatOptions::default());
    assert!(result.is_ok());
}

#[test]
fn format_range_multiple_declarations() {
    let input = ".a {\n  color: red;\n  font-size: 16px;\n}";
    let start = input.find("color").unwrap();
    let end = input.find("16px").unwrap() + 4;
    let result = format_range(input, start..end, Syntax::Css, &FormatOptions::default());
    assert!(result.is_ok());
}

#[test]
fn format_range_multiple_rules() {
    let input = ".a {\n  color: red;\n}\n.b {\n  color: blue;\n}";
    let start = input.find(".a").unwrap();
    let end = input.find(".b").unwrap() + 10; // +10 for ".b {\n  color:"
    let result = format_range(input, start..end, Syntax::Css, &FormatOptions::default());
    assert!(result.is_ok());
}

#[test]
fn format_range_out_of_bounds() {
    let input = ".a { color: red; }";
    let result = format_range(input, 0..1000, Syntax::Css, &FormatOptions::default());
    assert!(result.is_err());
}

#[test]
fn format_range_snapshot() {
    use insta::{assert_snapshot, glob};

    glob!("fmt_range/**/*.{css,scss,sass,less}", |path| {
        let input = std::fs::read_to_string(path).unwrap();
        let syntax = match path.extension().unwrap().to_str().unwrap() {
            "css" => Syntax::Css,
            "scss" => Syntax::Scss,
            "sass" => Syntax::Sass,
            "less" => Syntax::Less,
            _ => unreachable!("unknown file extension"),
        };

        let mut comments = vec![];
        let mut parser = ParserBuilder::new(&input)
            .syntax(syntax)
            .comments(&mut comments)
            .options(ParserOptions {
                try_parsing_value_in_custom_property: true,
                tolerate_semicolon_in_sass: true,
            })
            .build();
        parser
            .parse::<Stylesheet>()
            .map_err(|error| {
                format!(
                    "failed to parse '{}': {} from {} to {}",
                    path.display(),
                    error.kind,
                    error.span.start,
                    error.span.end
                )
            })
            .unwrap();
        let options = if let Some(config) = comments
            .first()
            .and_then(|comment| comment.content.trim_start().strip_prefix("cfg"))
        {
            toml::from_str::<FormatOptions>(config).unwrap()
        } else {
            Default::default()
        };
        // Read range from corresponding .range file

        let range_file = path.with_extension("range");
        let range_content = std::fs::read_to_string(&range_file).unwrap();

        let mut parts = range_content
            .split("..")
            .map(str::trim)
            .map(|s| s.parse().unwrap());
        let start: usize = parts.next().unwrap();
        let end: usize = parts.next().unwrap();

        let output = format_range(&input, start..end, syntax, &options).unwrap();
        let output = {
            let mut result = input.clone();
            result.replace_range(output.range, &output.code);
            result
        };

        similar_asserts::assert_eq!(
            output,
            output
                .lines()
                .map(|line| format!("{}\n", line.trim_end()))
                .collect::<String>(),
            "'{}' format result is unstable",
            path.display()
        );

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path(path.parent().unwrap());
        settings.remove_snapshot_suffix();
        settings.set_prepend_module_to_snapshot(false);
        settings.remove_input_file();
        settings.set_omit_expression(true);
        settings.remove_info();
        settings.bind(|| {
            let name = path.file_stem().unwrap().to_str().unwrap();
            assert_snapshot!(name, output);
        });
    });
}
