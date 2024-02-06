use miette::{Diagnostic, NamedSource, SourceSpan, Report};

#[derive(thiserror::Error, Debug, Diagnostic)]
#[error("MyBad")]
struct MyBad {
    #[source_code]
    src: NamedSource<String>,
    #[label("This bit here")]
    bad_bit: SourceSpan,
}

fn main() {
    let src = "fn main() { println!(\"this is a string\"); }".to_string();

    let x: Report = MyBad {
        src: NamedSource::new("bad_file.rs", src).with_language("Rust"),
        bad_bit: (9, 4).into(),
    }.into();
    println!("{:?}", x)

}
