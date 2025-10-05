use std::io::Read;
use std::path::Path;

use pulldown_cmark::{Event, Options, Parser, Tag};

use crate::tw::twirl::Tw; 


enum Mdi {
    Heading(u8, String),
    Paragraph(String),
    PageBreak
}


pub fn load(file: &Path) -> Result<Vec<Vec<Tw>>, Box<dyn std::error::Error>> {
   
    let mut file = std::fs::File::open(file)?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);


    // make an intermediate representation to lower complexity
    let mut mdi = vec![];
    for event in Parser::new_ext(&text, opts) {
        
        match event {
            
            Event::Start(tag) => {
                match tag {
                    Tag::Heading { level, .. } => {
                        mdi.push(Mdi::Heading(level as u8, String::new()));
                    }
                    Tag::Paragraph => {
                        mdi.push(Mdi::Paragraph(String::new()));
                    }
                    _ => {}
                }
            }

            Event::Text(text) => {
                match mdi.last_mut() {
                    Some(Mdi::Heading(_, s)) => {
                        s.push_str(&text);
                    }
                    Some(Mdi::Paragraph(s)) => {
                        s.push_str(&text);
                    }
                    _ => {}
                }
            }            

            Event::Rule => {
                mdi.push(Mdi::PageBreak);
            }

            _ => {}
        }
    }
    

    let mut tws = vec![];
    let mut current_page = vec![];
    
    for item in mdi {
        match item {
            Mdi::Heading(level, text) => {
                let size = match level {
                    1 => 48,
                    2 => 36,
                    3 => 24,
                    _ => 20,
                };
                current_page.push(Tw::Text(format!("# {}", text)));
            }
            Mdi::Paragraph(text) => {
                current_page.push(Tw::Text(text));
            }
            Mdi::PageBreak => {
                if !current_page.is_empty() {
                    tws.push(current_page);
                    current_page = vec![];
                }
            }
        }
    }
    if !current_page.is_empty() {
        tws.push(current_page);
    }       

    Ok(tws)
}
