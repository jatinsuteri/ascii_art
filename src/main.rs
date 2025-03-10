use std::path::{Path, PathBuf};
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
    path: Option<PathBuf>,
    ///color image or b/w
    #[arg(short, long, action = ArgAction::SetTrue)]
    color: bool,  
    ///define patterns
    #[arg(value_enum, short = 'a', long, default_value_t = AsciiPattern::Acerola)]
    pattern: AsciiPattern,
    #[arg(short,long,action = ArgAction::SetTrue)]
    invert: bool,
    #[arg(short,long,action= ArgAction::SetTrue)]
    edgedetec:bool,
    #[arg(short,long,action= ArgAction::SetTrue)]
    brighten:bool,
    #[arg(short,long,action = ArgAction::SetTrue)]
    video:bool,
    #[arg(short('k'),long,action = ArgAction::SetTrue)]
    camera:bool,
}

fn main() {
    let args = Args::parse();
    let path = match &args.path{
        Some(p) => p ,
        None => Path::new(""),
    };
    let res = run(&path, args.color,args.pattern, args.invert, args.edgedetec, args.brighten, args.video, args.camera);
    
    let html = create_html(&res, args.color).to_html_string();
    fs::write("output.html", html).expect("Unable to write HTML file");
    print!("Image Created, check output.html");
}

fn create_html(ascii_art: &str, color: bool) -> String {
        let style = if color {
            "body { background-color: black;
                    font-family:Courier; 
                    white-space: pre;
                    line-height: 7.5px;
                    letter-spacing: 2.5px;
                    font-size: 8px; 
            }"
        } else {
            "body { background-color: black; 
                    font-family:Courier; 
                    white-space: pre;
                    line-height: 7.5px;
                    letter-spacing: 2.5px;
                    color: #b5515a;
                    font-size: 8px; 
            }"
        };
    
        HtmlPage::new()
            .with_title("ASCII Art")
            .with_style(style)
            .with_html(ascii_art)
            .to_html_string()
    }

    
    
