use anyhow::{anyhow, Result};
use clap::Parser;
use comrak::adapters::SyntaxHighlighterAdapter;
use comrak::arena_tree::Node;
use comrak::nodes::{Ast, NodeValue};
use comrak::plugins::syntect::SyntectAdapter;
use include_dir::{include_dir, Dir, DirEntry};
use std::cell::RefCell;
use std::fs::{create_dir_all, File};
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use zk_pub_models::{Zettel, ZettelMap};

static DIST_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../zk-pub-app/dist");

#[derive(Parser, Debug)]
struct Opts {
    /// Input directory. If not set, try $ZK_NOTEBOOK_DIR.
    #[clap(long)]
    input: Option<PathBuf>,

    /// Output directory.
    #[clap(long)]
    output: PathBuf,
}

/// Syntax highlight adapter based on syntect that does not style the <pre> and <code> blocks.
struct Adapter<'a> {
    inner: SyntectAdapter<'a>,
}

impl Adapter<'_> {
    fn new() -> Self {
        Self {
            inner: SyntectAdapter::new("base16-ocean.light"),
        }
    }
}

impl SyntaxHighlighterAdapter for Adapter<'_> {
    fn highlight(&self, lang: Option<&str>, code: &str) -> String {
        self.inner.highlight(lang, code)
    }

    fn build_pre_tag(&self, _attributes: &std::collections::HashMap<String, String>) -> String {
        String::from("<pre>")
    }

    fn build_code_tag(&self, _attributes: &std::collections::HashMap<String, String>) -> String {
        String::from("<code>")
    }
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

fn get_text(node: &Node<RefCell<Ast>>) -> Option<String> {
    node.data
        .borrow()
        .value
        .text()
        .map(|d| match String::from_utf8(d.to_vec()) {
            Ok(s) => Some(s),
            Err(_) => None,
        })
        .flatten()
}

fn get_node_heading_text(node: &Node<RefCell<Ast>>) -> Option<String> {
    match node.data.borrow().value {
        NodeValue::Heading(_) => node.first_child().map(get_text).flatten(),
        _ => None,
    }
}

/// Try to read a Zettel from `path`.
fn zettel_from(path: PathBuf) -> Result<(String, Zettel)> {
    let anchor = path.file_stem().unwrap().to_string_lossy().to_string();
    let data = std::fs::read_to_string(&path)?;
    let arena = comrak::Arena::new();
    let options = comrak_options();

    let root = comrak::parse_document(&arena, &data, &options);

    let title = root
        .first_child()
        .map(get_node_heading_text)
        .flatten()
        .unwrap_or_else(|| anchor.clone());

    let adapter = Adapter::new();
    let mut plugins = comrak::ComrakPlugins::default();
    plugins.render.codefence_syntax_highlighter = Some(&adapter);

    let mut html = vec![];
    comrak::format_html_with_plugins(root, &options, &mut html, &plugins)?;
    let inner_html = String::from_utf8(html)?;

    Ok((anchor, Zettel { title, inner_html }))
}

/// Write front end app.
fn write_app(path: &Path) -> Result<()> {
    for entry in DIST_DIR.entries() {
        if let DirEntry::File(entry) = entry {
            let mut file = File::create(path.join(entry.path()))?;
            file.write_all(entry.contents())?;
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let opts = Opts::parse();

    if !opts.output.is_dir() {
        return Err(anyhow!("{:?} is not a directory", opts.output));
    }

    let input = match opts.input {
        Some(input) => input,
        None => PathBuf::from(std::env::var("ZK_NOTEBOOK_DIR")?),
    };

    let zettels = std::fs::read_dir(input)?
        .filter_map(Result::ok)
        .filter_map(path_if_entry_is_md)
        .map(zettel_from)
        .collect::<Result<ZettelMap, _>>()?;

    let path = opts.output.join("static");
    create_dir_all(&path)?;

    let file = std::fs::File::create(path.join("zettel.json"))?;
    serde_json::to_writer(file, &zettels)?;

    write_app(&opts.output)?;

    Ok(())
}
