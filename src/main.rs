use std::path::PathBuf;
use build_html::*;
use ascii_game_project_j1::{run, AsciiPattern};
use clap::{ArgAction, Parser};
use std::fs;
use build_html::{HtmlPage, Html};


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    ///provide path
    #[arg(short, long)]
    path: PathBuf,
    ///color image or b/w
    #[arg(short, long, action = ArgAction::SetTrue)]
    color: bool,  
    ///define patterns
    #[arg(value_enum, short = 'a', long, default_value_t = AsciiPattern::Acerola)]
    pattern: AsciiPattern,
}

fn main() {
    let args = Args::parse();
    let res = run(&args.path, args.color,args.pattern);

    let html = create_html(&res, args.color).to_html_string();

fn create_html(ascii_art: &str, color: bool) -> String {
        let style = if color {
            "body { background-color: black; font-family:Courier; white-space: pre;line-height: 7.5px; letter-spacing: 2.5px; font-size: 8px; }"
        } else {
            "body { background-color: black; font-family:Courier; white-space: pre;line-height: 7.5px; letter-spacing: 2.5px; color: white; font-size: 8px; }"
        };
    
        HtmlPage::new()
            .with_title("ASCII Art")
            .with_style(style)
            .with_html(ascii_art)
            .to_html_string()
    }

    fs::write("output.html", html).expect("Unable to write HTML file");
}