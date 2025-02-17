use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use press_btn_continue::wait;

#[derive(Debug)]
pub enum Elements {
    Node(Rc<RefCell<String>>),
    Edge(Rc<RefCell<String>>),
    Weight(HashMap<String, u64>),
    RelationEdge(RelationsEdge),
    RelationWeight(RelationsWeight),
}

#[derive(Debug)]
pub struct RelationsEdge {
    edge: Rc<RefCell<String>>,
    node1: Rc<RefCell<String>>,
    node2: Rc<RefCell<String>>,
}

#[derive(Debug)]
pub struct  RelationsWeight {
    edge: String,
    weight: u64,
}

fn store_elements(input: Elements, elements: &mut Vec<Elements>) {
    elements.push(input);
}

fn input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn check_is_num() -> u64 {
    loop {
        let weight_value = input();
        let _input: u64 = match weight_value.parse() {
            Ok(num) => {
                return num;
            },
            Err(_) => {
                println!("======================");
                println!("Please enter a number!");
                println!("======================");
                continue;
            }
        };
    }
}

fn check_if_node_exists1(elements: &mut Vec<Elements>) -> bool {
    for element in elements {
        if let Elements::Node(_) = element {
            return true;
        }
    }
    println!("=======================================");
    println!("No nodes available! Cannot create edge.");
    println!("=======================================");
    false
}

fn check_if_edge_exists1(elements: &mut Vec<Elements>) -> bool {
    for element in elements {
        if let Elements::Edge(_) = element {
            return true;
        }
    }
    println!("=========================================");
    println!("No edges available! Cannot create weight.");
    println!("=========================================");
    false
}

fn check_if_node_exists2(elements: &mut Vec<Elements>) -> bool {
    for element in elements {
        if let Elements::Node(_) = element {
            return true;
        }
    }
    println!("=======================================");
    println!("No nodes available! Cannot remove node.");
    println!("=======================================");
    false
}

fn check_if_edge_exists2(elements: &mut Vec<Elements>) -> bool {
    for element in elements {
        if let Elements::Edge(_) = element {
            return true;
        }
    }
    println!("=======================================");
    println!("No edges available! Cannot remove edge.");
    println!("=======================================");
    false
}

fn check_if_weight_exists(elements: &mut Vec<Elements>) -> bool {
    for element in elements {
        if let Elements::Weight(_) = element {
            return true;
        }
    }
    println!("===========================================");
    println!("No weights available! Cannot remove weight.");
    println!("===========================================");
    false
}

fn check_if_selected_node_exists(elements: &mut Vec<Elements>, node_name: &str) -> bool {
    for element in elements {
        if let Elements::Node(node) = element {
            if *node.borrow() == node_name {
                return true;
            }
        }
    }
    println!("========================================");
    println!("Node does not exist! Cannot delete node.");
    println!("========================================");
    false
}

fn check_if_selected_edge_exists(elements: &mut Vec<Elements>, edge_name: &str) -> bool {
    for element in elements {
        if let Elements::Edge(edge) = element {
            if *edge.borrow() == edge_name {
                return true;
            }
        }
    }
    println!("========================================");
    println!("Edge does not exist! Cannot delete edge.");
    println!("========================================");
    false
}

fn check_if_selected_weight_exists(elements: &mut Vec<Elements>, weight_name: &str) -> bool {
    for element in elements {
        if let Elements::Weight(weight) = element {
            if weight.contains_key(weight_name) {
                return true;
            }
        }
    }
    println!("==========================================");
    println!("Weight does not exist! Cannot delete weight.");
    println!("==========================================");
    false
}

pub fn create_node(elements: &mut Vec<Elements>) {
    println!("===========================");
    println!("Enter the name of the node:");
    println!("===========================");
    let node_name = input();
    let node = Elements::Node(Rc::new(RefCell::new(node_name)));
    store_elements(node, elements);
    println!("============");
    println!("Created node");
}

pub fn create_edge(elements: &mut Vec<Elements>, sorted: String) {
    if !check_if_node_exists1(elements) {
        return;
    }
    println!("===========================");
    println!("Enter the name of the edge:");
    println!("===========================");
    let edge_name = input();
    println!("===================================================");
    println!("Enter the node name from which the edge originates:");
    if let Some(node_line) = sorted.lines().next() {
        println!("Available nodes: {}", node_line);
    }
    println!("===================================================");
    let node1_name = input();
    println!("==========================================");
    println!("Enter the node name where the edge points:");
    if let Some(node_line) = sorted.lines().next() {
        println!("Available nodes: {}", node_line);
    }
    println!("==========================================");
    let node2_name = input();

    let edge = Rc::new(RefCell::new(edge_name.clone()));
    let node1 = Rc::new(RefCell::new(node1_name.clone()));
    let node2 = Rc::new(RefCell::new(node2_name.clone()));

    store_elements(Elements::Edge(edge.clone()), elements);

    let relations = RelationsEdge {
        edge,
        node1,
        node2,
    };
    store_elements(Elements::RelationEdge(relations), elements);

    println!("================");
    println!("Created edge: {}", edge_name);
}

pub fn create_weight(elements: &mut Vec<Elements>, sorted: String) {
    loop {
        if check_if_edge_exists1(elements) == false {
            break;
        }
        println!("==============================");
        println!("Enter the edge to be weighted:");
        if let Some(edge_line) = sorted.lines().nth(1) {
            println!("Available edges: {}", edge_line);
        }
        println!("==============================");
        let weight_name = input();
        println!("==============================");
        println!("Enter the value of the weight:");
        println!("==============================");
        let weight_value = check_is_num();
        let mut map = HashMap::new();
        map.insert(weight_name, weight_value);
        let weight = Elements::Weight(map);
        store_elements(weight, elements);
        println!("==============");
        println!("Created weight");
        break;
    }
}

pub fn remove_node(elements: &mut Vec<Elements>, sorted: String) {
    if check_if_node_exists2(elements) == false {
        return;
    }
    if let Some(first_line) = &sorted.lines().next() {
        println!("===================");
        println!("Available nodes: {}", first_line);
        println!("===================");
    }
    let node_name = input();
    if check_if_selected_node_exists(elements, &node_name) == false {
        return;
    }
    elements.retain(|element| {
        if let Elements::Node(node) = element {
            *node.borrow() != node_name
        } else {
            true
        }
    });
    println!("===================");
    println!("Removed node: {}", node_name);
}

pub fn remove_edge(elements: &mut Vec<Elements>, sorted: String)    {
    if check_if_edge_exists2(elements) == false {
        return;
    }
    if let Some(second_line) = &sorted.lines().nth(1) {
        println!("===================");
        println!("Available edges: {}", second_line);
        println!("===================");
    }
    let edge_name = input();
    if check_if_selected_edge_exists(elements, &edge_name) == false {
        return;
    }
    elements.retain(|element| {
        if let Elements::Edge(edge) = element {
            *edge.borrow() != edge_name
        } else if let Elements::RelationEdge(relation_edge) = element {
            *relation_edge.edge.borrow() != edge_name
        } else {
            true
        }
    });
    println!("===================");
    println!("Removed edge: {}", edge_name);
}

pub fn remove_weight(elements: &mut Vec<Elements>, sorted: String) {
    if check_if_weight_exists(elements) == false {
        return;
    }
    if let Some(_) = &sorted.lines().nth(2) {
        println!("=====================");
        for element in &mut *elements {
            if let Elements::Weight(weight) = element {
                for key in weight {
                    println!("Available weights: {:?}", key);
                }
            }
        }
        println!("=====================");
    }
    let weight_name = input();
    if check_if_selected_weight_exists(elements, &weight_name) == false {
        return;
    }
    elements.retain(|element| {
        if let Elements::Weight(weight) = element {
            !weight.contains_key(&weight_name)
        } else if let Elements::RelationWeight(relation_weight) = element {
            relation_weight.edge != weight_name
        } else {
            true
        }
    });
    println!("===================");
    println!("Removed weight: {}", weight_name);
}

pub fn display_graph(sorted_elements: String, elements: &Vec<Elements>) {
    let lines: Vec<&str> = sorted_elements.lines().collect();
    if !lines.is_empty() {
        println!("Nodes:");
        for node in lines[0].split(',').filter(|s| !s.trim().is_empty()) {
            println!("  - {}", node.trim());
        }
    }
    println!("Edges:");
    for element in elements {
        if let Elements::RelationEdge(relation_edge) = element {
            println!("  - {}: {} -> {}", relation_edge.edge.borrow(), relation_edge.node1.borrow(), relation_edge.node2.borrow());
        }
    }
    println!("Weights:");
    for element in elements {
        if let Elements::Weight(weight_map) = element {
            for (edge, weight) in weight_map {
                println!("  - {}: {}", edge, weight);
            }
        }
    }
    wait("Press Enter to continue...").unwrap();
    println!(" ");
}