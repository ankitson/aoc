//From
//abc: def ghi
//def: asds
//to graphviz
fn to_graphviz(input: &str) -> String {
    let mut result = String::from("graph {\n");

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() != 2 {
            continue;
        }

        let source = parts[0].trim();
        let destinations: Vec<&str> = parts[1].trim().split_whitespace().collect();

        if destinations.len() == 1 {
            result.push_str(&format!("    {} -- {}\n", source, destinations[0]));
        } else {
            result.push_str(&format!("    {} -- {{{}}}\n", source, destinations.join(" ")));
        }
    }

    result.push_str("}\n");
    result
}
