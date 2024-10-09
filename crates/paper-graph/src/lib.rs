mod printer;

use std::collections::HashSet;

use biblatex::{Bibliography, ChunksExt, Type};
use palette::{Darken, Lighten, Srgb};
use printer::{attr, attr_esc, digraph, edge, edge_attr, node, node_attr};

#[derive(Debug, Default)]
struct GraphData {
    pub seeds: HashSet<String>,
    pub exclusions: HashSet<String>,
    pub edges: Vec<(String, String)>,
}

fn parse_bib(source: &str) -> Bibliography {
    Bibliography::parse(source).unwrap()
}

fn parse_graph(source: &str) -> GraphData {
    let mut graph = GraphData::default();
    for line in source.lines() {
        let line = if let Some(cmt_idx) = line.find("#") {
            &line[0..cmt_idx]
        } else {
            line
        };
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if line.contains("->") {
            let mut sp = line.split("->");
            let (a, b) = (sp.next().unwrap(), sp.next().unwrap());
            (graph.edges).push((a.trim().to_string(), b.trim().to_string()));
        } else if line.contains("<-") {
            let mut sp = line.split("<-");
            let (a, b) = (sp.next().unwrap(), sp.next().unwrap());
            (graph.edges).push((b.trim().to_string(), a.trim().to_string()));
        } else if let Some(seed) = line.strip_prefix("seed:") {
            graph.seeds.insert(seed.trim().to_string());
        } else if let Some(exclude) = line.strip_prefix("exclude:") {
            graph.exclusions.insert(exclude.trim().to_string());
        }
    }
    (graph.edges).retain(|(u, v)| !graph.exclusions.contains(u) && !graph.exclusions.contains(v));
    graph.edges.sort_unstable();
    graph.edges.dedup();
    graph
}

fn trim_brace(s: &str) -> &str {
    &s[1..s.len() - 1]
}

fn ell(s: &str, n: usize) -> String {
    if s.len() <= n {
        return s.to_string();
    }
    let pos = s[..n].rfind(' ').unwrap_or(n);
    format!("{} ...", &s[..pos])
}

fn render_graph(bib: &Bibliography, graph: &GraphData) -> String {
    let gradient = colorous::WARM;
    let mut stmts = vec![
        attr("rankdir", "LR"),
        edge_attr("arrowsize", &0.5.to_string()),
        node_attr("width", &1.to_string()),
    ];
    for entry in bib.iter() {
        if graph.exclusions.contains(&entry.key) {
            continue;
        }
        let title = entry.title().unwrap().to_biblatex_string(false);
        let authors = entry
            .author()
            .unwrap()
            .to_chunks()
            .to_biblatex_string(false);
        let year = entry.fields.get("year").unwrap().to_biblatex_string(false);
        let year_val = trim_brace(&year).parse::<i32>().unwrap();
        let lbl = format!(
            "{} | {} | {{{} | {}}}",
            ell(trim_brace(&title), 32),
            ell(trim_brace(&authors), 32),
            trim_brace(&year),
            entry.entry_type
        );
        let color = gradient.eval_continuous(((2024 - year_val) as f64 / 10.0).min(1.0));
        let color = Srgb::new(color.r, color.g, color.b).into_format::<f32>();

        let mut attrs = vec![
            attr_esc("label", &lbl),
            attr("shape", "record"),
            attr_esc(
                "fillcolor",
                &format!("#{:x}", color.lighten(0.8).into_format::<u8>()),
            ),
            attr_esc(
                "color",
                &format!("#{:x}", color.darken(0.2).into_format::<u8>()),
            ),
        ];
        if graph.seeds.contains(&entry.key) {
            attrs.push(attr("fontcolor", "crimson"));
            attrs.push(attr_esc("style", "filled,bold"));
        } else {
            attrs.push(attr("style", "filled"));
        }
        stmts.push(node(&entry.key, &attrs));
    }
    for (u, v) in &graph.edges {
        stmts.push(edge(u, v));
    }
    digraph(&stmts)
}

pub fn generate_paper_graph(bib_source: &str, graph_source: &str) -> String {
    render_graph(&parse_bib(bib_source), &parse_graph(graph_source))
}
