use std::collections::HashMap;
#[derive(Debug)]
pub struct AdjList {
    pub nbrs: HashMap<String, Vec<String>>,
}

pub fn parse(input: &str) -> AdjList {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for line in input.lines() {
        if let Some((a, b)) = line.split_once('-') {
            let mut newVec: Vec<String> = Vec::new();
            match map.get(a) {
                None => {
                    newVec.push(b.to_string());
                    map.insert(a.to_string(), newVec);
                }
                Some(_) => {
                    let v = map.get_mut(a).unwrap();
                    v.push(b.to_string())
                }
            };
            let mut newVec: Vec<String> = Vec::new();
            match map.get(b) {
                None => {
                    newVec.push(a.to_string());
                    map.insert(b.to_string(), newVec);
                }
                Some(_) => {
                    let v = map.get_mut(b).unwrap();
                    v.push(a.to_string())
                }
            };
        } else {
            panic!("unable to parse line {}", line)
        }
    }
    AdjList { nbrs: map }
}
