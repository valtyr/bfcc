use super::ir::IRNode;

pub fn c_for_node(line_vec: &mut Vec<String>, node: IRNode, indent: u16) {
    let spaces = std::iter::repeat(" ")
        .take((indent * 2) as usize)
        .collect::<String>();

    match node {
        IRNode::Move(count) => {
            if count > 0 {
                line_vec.push(format!("{}ptr += {};", spaces, count));
            } else if count < 0 {
                line_vec.push(format!("{}ptr -= {};", spaces, -count));
            }
        }
        IRNode::Add { value: by, offset } => {
            if by > 0 {
                line_vec.push(format!("{}ptr[{}] += {};", spaces, offset, by));
            } else if by < 0 {
                line_vec.push(format!("{}ptr[{}] -= {};", spaces, offset, -by));
            }
        }
        IRNode::Zero { offset } => line_vec.push(format!("{}ptr[{}] = 0;", spaces, offset)),
        IRNode::Output { offset } => line_vec.push(format!("{}putchar(ptr[{}]);", spaces, offset)),
        IRNode::Input { offset } => {
            line_vec.push(format!("{}ptr[{}] = getchar();", spaces, offset))
        }
        IRNode::Loop { children } => {
            line_vec.push(spaces.clone() + "while (*ptr) {");
            for child in children {
                c_for_node(line_vec, child, indent + 1);
            }
            line_vec.push(spaces + "}");
        }
    };
}
