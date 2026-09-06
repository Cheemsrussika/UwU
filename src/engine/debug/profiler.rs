use std::collections::HashMap;
use std::time::Instant;
use super::result_field::ResultField;

#[derive(Default, Clone)]
pub struct ProfilerNode {
    pub total_nanos: u64,
    pub count: u64,
    pub children: HashMap<String, ProfilerNode>,
}

pub struct Profiler {
    pub stack: Vec<(String, Instant)>,
    pub current_path: Vec<String>,
    pub root: ProfilerNode,
}

impl Profiler {
    pub fn new() -> Self {
        Self { stack: Vec::new(), current_path: Vec::new(), root: ProfilerNode::default() }
    }

    pub fn start_tick(&mut self) {
        self.stack.clear();
        self.current_path.clear();
        self.push("root");
    }

    pub fn end_tick(&mut self) {
        self.pop();
    }

    pub fn push(&mut self, name: &str) {
        self.current_path.push(name.to_string());
        self.stack.push((name.to_string(), Instant::now()));
    }

    pub fn pop(&mut self) {
        if let Some((_, start)) = self.stack.pop() {
            let elapsed = start.elapsed().as_nanos() as u64;
            let mut node = &mut self.root;
            for seg in &self.current_path {
                node = node.children.entry(seg.clone()).or_default();
            }
            node.total_nanos += elapsed;
            node.count += 1;
            self.current_path.pop();
        }
    }

    pub fn get_times(&self, path: &str) -> Vec<ResultField> {
        let segs: Vec<&str> = if path.is_empty() || path == "root" {
            vec!["root"]
        } else {
            path.split('.').collect()
        };

        let mut curr = &self.root;
        for &s in &segs {
            if let Some(c) = curr.children.get(s) { curr = c; } else { return Vec::new(); }
        }

        let node_total = curr.total_nanos.max(1) as f64;
        let root_total = self.root.children.get("root").map(|r| r.total_nanos.max(1) as f64).unwrap_or(node_total);
        let mut list = Vec::new();
        let mut children_sum = 0.0;

        for (c_name, c_node) in &curr.children {
            let p = (c_node.total_nanos as f64 / node_total) * 100.0;
            let gp = (c_node.total_nanos as f64 / root_total) * 100.0;
            children_sum += p;
            list.push(ResultField::new(c_name.clone(), p, gp, c_node.count));
        }

        list.sort_by(|a, b| b.percentage.partial_cmp(&a.percentage).unwrap());
        if children_sum < 99.9 {
            let rem = 100.0 - children_sum;
            let rem_gp = (rem / 100.0) * (node_total / root_total) * 100.0;
            list.push(ResultField::new("unspecified".to_string(), rem, rem_gp, 0));
        }

        let node_name = segs.last().copied().unwrap_or("root").to_string();
        let node_gp = (node_total / root_total) * 100.0;
        list.insert(0, ResultField::new(node_name, 100.0, node_gp, curr.count));
        list
    }
}