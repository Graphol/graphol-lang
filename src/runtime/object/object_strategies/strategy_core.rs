#[path = "node_primitives.rs"]
mod node_primitives;
#[path = "numeric_ops.rs"]
mod numeric_ops;

pub use node_primitives::{new_node, new_string};
pub use numeric_ops::{new_number, new_operator};
