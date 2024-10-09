pub fn attr(key: &str, value: &str) -> String {
    format!("{key} = {value}")
}

pub fn attr_esc(key: &str, value: &str) -> String {
    format!("{key} = \"{value}\"")
}

pub fn node_attr(key: &str, value: &str) -> String {
    format!("node [{key} = {value}]")
}

pub fn edge_attr(key: &str, value: &str) -> String {
    format!("edge [{key} = {value}]")
}

pub fn node(id: &str, attrs: &[String]) -> String {
    format!("\"{id}\" [{}]", attrs.join(", "))
}

pub fn edge(from: &str, to: &str) -> String {
    format!("\"{from}\" -> \"{to}\"")
}

pub fn digraph(stmts: &[String]) -> String {
    format!(
        "digraph {{
{}
}}",
        stmts.join("\n")
    )
}
