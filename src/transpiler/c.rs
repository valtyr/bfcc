use super::ir::IRNode;

pub fn c_for_node(line_vec: &mut Vec<String>, node: IRNode, indent: u16) {
    let spaces = std::iter::repeat(" ")
        .take((indent * 2) as usize)
        .collect::<String>();

    match node {
        IRNode::ForwardBackward(count) => {
            if count > 0 {
                line_vec.push(format!("{}ptr += {};", spaces, count));
            } else if count < 0 {
                line_vec.push(format!("{}ptr -= {};", spaces, -count));
            }
        }
        IRNode::IncrementDecrement(count) => {
            if count > 0 {
                line_vec.push(format!("{}*ptr += {};", spaces, count));
            } else if count < 0 {
                line_vec.push(format!("{}*ptr -= {};", spaces, -count));
            }
        }
        IRNode::Output => line_vec.push(spaces + "putchar(*ptr);"),
        IRNode::Input => line_vec.push(spaces + "*ptr = getchar();"),
        IRNode::Loop { children } => {
            line_vec.push(spaces.clone() + "while (*ptr) {");
            for child in children {
                c_for_node(line_vec, child, indent + 1);
            }
            line_vec.push(spaces + "}");
        }
    };
}
