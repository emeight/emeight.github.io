//! Site builder: Markdown in `src/content/<kind>/` -> `<kind>/<slug>/index.html`.
//! 
//! cargo run                               # full rebuild
//! cargo run -- notes                      # all notes + notes/index.html (+ homepage)
//! cargo run -- src/content/notes/x.md     # one page + its listing (+ homepage)

use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use parser::frontmatter::{self, Frontmatter};
use parser::{site, to_html};

struct Kind {
    dir: &'static str,
    page_tmplt: &'static str,
    card_tmplt: &'static str,
    list_tmplt: &'static str,
    list_title: &'static str,
    list_blurb: &'static str,
    required: &'static [&'static str],
}

const KINDS: &[Kind] = &[
    Kind {
        dir: "notes",
        page_tmplt: "templates/note.html",
        card_tmplt: "templates/note-card.html",
        list_tmplt: "templates/notes-index.html",
        list_title: "All Notes",
        list_blurb: "Notes and thoughts.",
        required: &["title", "date", "description", "kicker", "serial"],
    },
    Kind {
        dir: "builds",
        page_tmplt: "templates/build.html",
        card_tmplt: "templates/build-card.html",
        list_tmplt: "templates/builds-index.html",
        list_title: "All Builds",
        list_blurb: "Things I'm building.",
        required: &[
            "title", "date", "description", "kicker", "serial",
            "image", "image_alt", "version", "phase",
        ],
    },
];

struct Entry {
    path: PathBuf,
    slug: String,
    fm: Frontmatter,
    body: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2) // src/parser -> src -> repo root
        .ok_or("cannot locate repo root")?
        .to_path_buf();
    let src = root.join("src");

    let args: Vec<String> = std::env::args().skip(1).collect();
    let files: Vec<PathBuf> = args
        .iter()
        .filter(|a| a.ends_with(".md"))
        .map(|a| root.join(a))
        .collect();
    let names: Vec<&str> = args
        .iter()
        .filter(|a| !a.ends_with(".md"))
        .map(String::as_str)
        .collect();

    let layout = fs::read_to_string(src.join("layout.html"))?;

    // collect every kind once; the homepage needs all entries
    let mut all: Vec<(&Kind, Vec<Entry>)> = Vec::new();
    for kind in KINDS {
        all.push((kind, collect(&src, kind.dir)?));
    }

    let mut built = 0;

    for (kind, entries) in &all {
        let picked: Vec<&PathBuf> = files
            .iter()
            .filter(|f| kind_for(&src, f).as_deref() == Some(kind.dir))
            .collect();

        if !(args.is_empty() || names.contains(&kind.dir) || !picked.is_empty()) {
            continue;
        }

        if entries.is_empty() {
            continue;
        }

        // if only .md files were named, build just those pages
        let only: Option<Vec<String>> = (names.is_empty() && !files.is_empty()).then(|| {
            picked
                .iter()
                .filter_map(|f| f.file_stem().map(|s| s.to_string_lossy().into_owned()))
                .collect()
        });

        for e in entries {
            if let Some(slugs) = &only {
                if !slugs.contains(&e.slug) {
                    continue;
                }
            }
            let out = write_page(&layout, &root, &src, kind, e)?;
            println!("{} -> {}", rel(&root, &e.path), rel(&root, &out));
            built += 1;
        }

        let out = write_list(&layout, &root, &src, kind, entries)?;
        println!(
            "{} entr{} -> {}",
            entries.len(),
            if entries.len() == 1 { "y" } else { "ies" },
            rel(&root, &out)
        );
    }

    // build homepage last to reflect changes to featured previews
    if let Some(out) = write_home(&layout, &root, &src, &all)? {
        println!("src/content/home.html -> {}", rel(&root, &out));
    }

    println!("\n{built} page(s) built");
    Ok(())
}

fn collect(src: &Path, dir: &str) -> Result<Vec<Entry>, Box<dyn Error>> {
    let content = src.join("content").join(dir);
    if !content.exists() {
        return Ok(Vec::new());
    }

    let mut entries = Vec::new();
    for entry in fs::read_dir(&content)? {
        let path = entry?.path();
        if path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }
        let raw = fs::read_to_string(&path)?;
        let (fm, body) = frontmatter::split(&raw);
        let slug = path.file_stem().unwrap().to_string_lossy().into_owned();
        entries.push( Entry { path, slug, fm, body: body.to_string() });
    }

    // order by most recent
    entries.sort_by(|a, b| b.slug.cmp(&a.slug));
    Ok(entries)
}

fn write_page(layout: &str, root: &Path, src: &Path, kind: &Kind, e: &Entry) -> Result<PathBuf, Box<dyn Error>> {
    for field in kind.required {
        e.fm.require(field).map_err(|m| format!("{}: {m}", e.path.display()))?;
    }
    let inner = fs::read_to_string(src.join(kind.page_tmplt))?;
    let html = site::render_page(layout, &inner, &e.fm, &to_html(&e.body));

    let out = root.join(kind.dir).join(&e.slug).join("index.html");
    fs::create_dir_all(out.parent().unwrap())?;
    fs::write(&out, html)?;
    Ok(out)
}

fn write_list(layout: &str, root: &Path, src: &Path, kind: &Kind, entries: &[Entry]) -> Result<PathBuf, Box<dyn Error>> {
    let inner = fs::read_to_string(src.join(kind.list_tmplt))?;
    let card = fs::read_to_string(src.join(kind.card_tmplt))?;

    let data: Vec<(&str, &Frontmatter)> = entries.iter().map(|e| (e.slug.as_str(), &e.fm)).collect();
    let html = site::render_list(layout, &inner, &card, kind.list_title, kind.list_blurb, &data);

    let out = root.join(kind.dir).join("index.html");
    fs::write(&out, html)?;
    Ok(out)
}

fn write_home(
    layout: &str,
    root: &Path,
    src: &Path,
    all: &[(&Kind, Vec<Entry>)],
) -> Result<Option<PathBuf>, Box<dyn Error>> {
    let home = match fs::read_to_string(src.join("content/home.html")) {
        Ok(s) => s,
        Err(_) => {
            eprintln!("warning: src/content/home.html missing, skipping homepage");
            return Ok(None);
        }
    };

    let (fm, inner) = frontmatter::split(&home);

    let note_sel = featured(&fm, "featured_notes", entries_for(all, "notes"))?;
    let build_sel = featured(&fm, "featured_builds", entries_for(all, "builds"))?;

    let note_card = fs::read_to_string(src.join("templates/note-card.html"))?;
    let build_card = fs::read_to_string(src.join("templates/build-card.html"))?;

    let note_data: Vec<(&str, &Frontmatter)> = note_sel.iter().map(|e| (e.slug.as_str(), &e.fm)).collect();
    let build_data: Vec<(&str, &Frontmatter)> = build_sel.iter().map(|e| (e.slug.as_str(), &e.fm)).collect();

    let html = site::render_home(
        layout, inner, &fm, &note_card, &note_data, &build_card, &build_data,
    );

    let out = root.join("index.html");
    fs::write(&out, html)?;
    Ok(Some(out))
}

fn entries_for<'a>(all: &'a [(&Kind, Vec<Entry>)], dir: &str) -> &'a [Entry] {
    all.iter()
        .find(|(k, _)| k.dir == dir)
        .map(|(_, e)| e.as_slice())
        .unwrap_or(&[])
}

/// `featured_*` frontmatter: absent -> latest 3; present -> exactly as listed
/// slugs, in order. An unknown slug will raise an error.
fn featured<'a>(
    fm: &Frontmatter,
    key: &str,
    entries: &'a [Entry],
) -> Result<Vec<&'a Entry>, String> {
    let list = match fm.get(key) {
        None => return Ok(entries.iter().take(3).collect()),
        Some(l) => l,
    };
    let mut out = Vec::new();
    for slug in list.split(',').map(str::trim).filter(|s| !s.is_empty()) {
        let e = entries
            .iter()
            .find(|e| e.slug == slug)
            .ok_or_else(|| format!("src/content/home.html: {key} references unknown slug `{slug}`"))?;
        out.push(e);
    }
    Ok(out)
}

fn kind_for(src: &Path, md: &Path) -> Option<String> {
    let rel = md.strip_prefix(src.join("content")).ok()?;
    Some(rel.components().next()?.as_os_str().to_string_lossy().into_owned())
}

fn rel(root: &Path, p: &Path) -> String {
    p.strip_prefix(root).unwrap_or(p).display().to_string()
}