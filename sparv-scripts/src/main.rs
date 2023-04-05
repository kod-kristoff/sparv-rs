use std::{fs, io, path::{Path, PathBuf}};

use quick_xml::{Reader, Writer, events::Event};

type Error = Box<dyn std::error::Error>;
fn main() -> Result<(), Error>  {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("trace")).init();
    let text_path: PathBuf = std::env::args()
        .nth(1)
        .map(|p| p.into() )
        .unwrap_or_else(|| "./data/8sidor-mini.xml".into());

    let num_text_elems: u32 = std::env::args().nth(2).map(|x| x.parse().expect("expected integer")).unwrap_or(1);
    let dst_path =  std::env::args()
        .nth(3).map(|f| f.into());

    log::info!("Reading {} 'text'-elements from '{}'", num_text_elems,text_path.display());
    let dst_path = dst_path.unwrap_or_else(|| {
        let parent = text_path.parent().unwrap_or_else(|| Path::new("./data"));
        let dst_stem = text_path.file_stem().map(|f| f.to_str()).flatten().unwrap_or("output");
        let dst_ext = text_path.extension().map(|f| f.to_str()).flatten().unwrap_or("out");
        let dst_file = format!("{}-{}.{}", dst_stem, num_text_elems, dst_ext);
        parent.join(&dst_file)
    });
    log::info!("Writing to '{}'", dst_path.display());
    let mut reader = Reader::from_file(text_path)?;
    let dst_file = fs::File::create(dst_path)?;
    let dst_writer = io::BufWriter::new(dst_file);
    let mut writer = Writer::new(dst_writer);
    let mut buf = Vec::new();
    let mut num_elems = 0;
    let mut write_elem = true;
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) if e.name().as_ref() == b"text" => {
                if num_elems < num_text_elems {
                    assert!(writer.write_event(Event::Start(e)).is_ok());
                    write_elem = true;
                } else {
                    write_elem = false;
                }
            }
            Ok(Event::End(e)) if e.name().as_ref() == b"text" => {
                if num_elems < num_text_elems {
                    assert!(writer.write_event(Event::End(e)).is_ok());
                }
                write_elem = true;
                num_elems += 1;
            }
            Ok(Event::Eof) => break,
            Err(e) => {
                eprintln!("Error at position {}: {:?}", reader.buffer_position(), e);
                return Err(e.into());
            }
            Ok(e) => {
                if write_elem {
                    assert!(writer.write_event(e).is_ok());
                }
            },
        }
    }
    log::info!("Found {} elements", num_elems);
    Ok(())
}
