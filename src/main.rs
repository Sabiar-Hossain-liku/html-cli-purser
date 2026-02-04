use clap::Parser;
use maud::{html, Markup, DOCTYPE};
use pulldown_cmark::{html as md_html, Options, Parser as MarkdownParser};
use std::{fs, path::PathBuf};

#[derive(Parser, Debug)]
#[command(about = "markdown to html")]
struct Args {
    /// Input Markdown file path
    #[arg(long, short)]
    input: PathBuf,

    /// Output Html File Path
    #[arg(long, short)]
    output: Option<PathBuf>,
}

fn render_html_page(content: &str) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                meta charset= "utf-8";
                title {"Markdown to HTML Output"}
            }
            body {
                 (maud::PreEscaped(content))
            }
        }

    }
}

fn main() {
    let args = Args::parse();

    let markdown_input = fs::read_to_string(&args.input).expect("faild to read the file ");

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);

    let parser = MarkdownParser::new_ext(&markdown_input, options);

    let mut html_output = String::new();
    md_html::push_html(&mut html_output, parser);

    let full_html_output = render_html_page(&html_output).into_string();

    match &args.output {
        Some(path) => fs::write(path, full_html_output).expect("Failed to write output"),
        None => println!("Path not provided"),
    }
}
