mod visual;
mod operations;
use operations::Elements;

fn input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

pub fn run() {
    let mut elements = Vec::new();
    loop {
        let b = format_elements(&elements);
        visual::choices();
        let choice = input();
        let filtered_input: u32 = match choice.parse() {
            Ok(num) => num,
            Err(_) => {
                visual::enter_number_err();
                continue;
            }
        };
        match filtered_input {
            1 => sort_add(&mut elements),
            2 => sort_remove(&mut elements),
            3 => operations::display_graph(b, &mut elements),
            4 => break,
            _ => visual::enter_value_err(),
        }
    }
}

fn sort_add(elements: &mut Vec<Elements>) {
    visual::add_elements();
    loop {
        let choice = input();
        let filtered_input: u32 = match choice.parse() {
            Ok(num) => num,
            Err(_) => {
                visual::enter_number_err();
                continue;
            }
        };
        match filtered_input {
            1 => operations::create_node(elements),
            2 => operations::create_edge(elements, format_elements(elements)),
            3 => operations::create_weight(elements, format_elements(elements)),
            _ => visual::enter_value_err(),
        }
        break;
    }
}

fn sort_remove(elements: &mut Vec<Elements>) {
    visual::remove_elements();
    loop {
        let choice = input();
        let filtered_input: u32 = match choice.parse() {
            Ok(num) => num,
            Err(_) => {
                visual::enter_number_err();
                continue;
            }
        };
        match filtered_input {
            1 => operations::remove_node(elements, format_elements(elements)),
            2 => operations::remove_edge(elements, format_elements(elements)),
            3 => operations::remove_weight(elements, format_elements(elements)),
            _ => visual::enter_value_err(),
        }
        break;
    }
}

fn format_elements(elements: &Vec<Elements>) -> String {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    let mut weights = Vec::new();
    let mut relations = Vec::new();

    for element in elements {
        match element {
            Elements::Node(node) => {
                nodes.push(format!("\"{}\"", node.borrow()));
            }
            Elements::Edge(edge) => {
                edges.push(format!("\"{}\"", edge.borrow()));
            }
            Elements::Weight(weight) => {
                weights.push(format!("\"{:?}\"", weight));
            }
            Elements::RelationEdge(relation_edge) => {
                relations.push(format!("\"{:#?}\"", relation_edge));
            }
            Elements::RelationWeight(relation_weight) => {
                relations.push(format!("\"{:#?}\"", relation_weight));
            }
        }
    }

    let mut formatted_elements = String::new();

    if !nodes.is_empty() {
        formatted_elements.push_str(&format!("{}\n", nodes.join(" ")));
    }
    if !edges.is_empty() {
        formatted_elements.push_str(&format!("{}\n", edges.join(" ")));
    }
    if !weights.is_empty() {
        formatted_elements.push_str(&format!("{}\n", weights.join(" ")));
    }
    if !relations.is_empty() {
        formatted_elements.push_str(&format!("{}\n", relations.join(" ")));
    }

    formatted_elements
}
