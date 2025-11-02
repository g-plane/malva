use insta::{Settings, assert_snapshot, glob};
use malva::{config::FormatOptions, format_text};
use raffia::{ParserBuilder, ParserOptions, Syntax, ast::Stylesheet};
use std::fs;

#[test]
fn fmt_snapshot() {
    glob!("fmt/**/*.{css,scss,sass,less}", |path| {
        let input = fs::read_to_string(path).unwrap();
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

        let output = format_text(&input, syntax, &options).unwrap();
        similar_asserts::assert_eq!(
            output,
            output
                .lines()
                .map(|line| format!("{}\n", line.trim_end()))
                .collect::<String>(),
            "'{}' has trailing whitespace",
            path.display()
        );

        let regression_output = format_text(&output, syntax, &options).expect(&format!(
            "Failed to parse in stability test. Output:\n{output}"
        ));
        similar_asserts::assert_eq!(
            output,
            regression_output,
            "'{}' format is unstable",
            path.display()
        );

        let mut settings = Settings::clone_current();
        settings.set_snapshot_path(path.parent().unwrap());
        settings.remove_snapshot_suffix();
        settings.set_prepend_module_to_snapshot(false);
        settings.remove_input_file();
        settings.set_omit_expression(true);
        settings.remove_input_file();
        settings.remove_info();
        settings.bind(|| {
            let name = path.file_stem().unwrap().to_str().unwrap();
            assert_snapshot!(name, output);
        });
    });
}
