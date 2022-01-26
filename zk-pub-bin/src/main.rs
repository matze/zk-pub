use anyhow::{anyhow, Result};
use clap::Parser;
use std::path::PathBuf;
use zk_pub_models::Zettel;
use comrak::nodes::{NodeHeading, NodeValue};

#[derive(Parser, Debug)]
struct Opts {
    /// Input directory. If not set, try $ZK_NOTEBOOK_DIR.
    #[clap(long)]
    input: Option<PathBuf>,

    /// Output directory.
    #[clap(long)]
    output: PathBuf,
}

/// Return a new `PathBuf` if `entry` is a file and ends with .md.
fn path_if_entry_is_md(entry: std::fs::DirEntry) -> Option<PathBuf> {
    let path = entry.path();

    if path.is_file() {
        match path.extension() {
            Some(ext) => {
                if ext == "md" {
                    Some(path)
                } else {
                    None
                }
            }
            None => None,
        }
    } else {
        None
    }
}

/// Create parse and render options.
fn comrak_options() -> comrak::ComrakOptions {
    let extension = comrak::ComrakExtensionOptions {
        table: true,
        ..Default::default()
    };

    let parse = comrak::ComrakParseOptions {
        smart: true,
        ..Default::default()
    };

    comrak::ComrakOptions {
        extension,
        parse,
        ..Default::default()
    }
}

/// Try to read a Zettel from `path`.
fn zettel_from(path: PathBuf) -> Result<Zettel> {
    let anchor = path.file_stem().unwrap().to_string_lossy().to_string();
    let data = std::fs::read_to_string(&path)?;
    let arena = comrak::Arena::new();
    let options = comrak_options();

    let root = comrak::parse_document(&arena, &data, &options);

    // Ugh ...
    let title = if let Some(node) = root.first_child() {
        if let NodeValue::Heading(NodeHeading {
            level: _,
            setext: _,
        }) = node.data.borrow().value
        {
            if let Some(node) = node.first_child() {
                if let Some(data) = node.data.borrow().value.text() {
                    String::from_utf8(data.to_vec())?
                } else {
                    anchor.clone()
                }
            } else {
                anchor.clone()
            }
        } else {
            anchor.clone()
        }
    } else {
        anchor.clone()
    };

    let mut html = vec![];
    comrak::format_html(&root, &options, &mut html)?;
    let inner_html = String::from_utf8(html)?;

    Ok(Zettel {
        anchor,
        title,
        inner_html,
    })
}

fn main() -> Result<()> {
    let opts = Opts::parse();

    if !opts.output.is_dir() {
        return Err(anyhow!("{:?} is not a directory", opts.output));
    }

    let zettels = std::fs::read_dir(opts.input.unwrap())?
        .filter_map(Result::ok)
        .filter_map(path_if_entry_is_md)
        .map(zettel_from)
        .collect::<Result<Vec<Zettel>, _>>()?;

    let path = opts.output.join("zettel.json");
    let file = std::fs::File::create(path)?;
    serde_json::to_writer(file, &zettels)?;

    Ok(())
}
