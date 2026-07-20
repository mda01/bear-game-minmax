pub trait NodeType: Sized {
    fn is_over(&self) -> bool;
    fn heuristic(&self) -> i32;
    fn get_children(&self) -> Vec<Self>;
    fn hash(&self) -> i128;
}

pub fn minmax<T: NodeType>(node: T, depth: usize, player: bool) -> i32 {
    if depth == 0 || node.is_over() {
        return node.heuristic();
    }
    if player == true {
        let mut val = i32::MIN;
        for child in node.get_children() {
            val = std::cmp::max(val, minmax(child, depth - 1, false));
        }
        return val;
    } else {
        let mut val = i32::MAX;
        for child in node.get_children() {
            val = std::cmp::min(val, minmax(child, depth - 1, false));
        }
        return val;
    }
}
