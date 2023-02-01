pub mod error;
pub mod source;
mod source_map;
pub mod span;

#[cfg(test)]
mod tests {
    use crate::{
        error::{EnvyError, EnvyErrorAnnotation, EnvyErrorLevel},
        source_map::SourceMap,
        span::Span,
    };
    use codespan_reporting::term::{
        self,
        termcolor::{ColorChoice, StandardStream},
        Config,
    };

    use indoc::indoc;

    #[test]
    fn it_works() {
        let mut source_map = SourceMap::default();
        let source_id = source_map.push(
            "a.envy",
            indoc! {
                r#"
                // Adds two numbers, but as a function, it is
                // quite useless.
                //
                // a :: Int - First number to add.
                // b :: Int - Second number to add.
                define add(a: Int, b: Int) =
                    a + c
                "#
            },
        );

        let error = EnvyError {
            level: EnvyErrorLevel::Error,
            code: 1,
            title: "undefined reference".to_string(),
            annotations: vec![EnvyErrorAnnotation {
                message: Some("`c` has not been defined by this point".to_string()),
                span: Span {
                    source_id,
                    start_pos: 175,
                    end_pos: 176,
                },
            }],
            footer_notes: vec!["perhaps you meant to use `b`?".to_string()],
        };

        let writer = StandardStream::stderr(ColorChoice::Auto);
        let config = Config::default();
        term::emit(&mut writer.lock(), &config, &source_map, &error.into()).unwrap();
    }
}
