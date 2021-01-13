fn collapse(head: &Box<TreeNode>) -> i32 {
    let mut x: Option<i32> = Option::None;
    let mut y: Option<i32> = Option::None;

    if let Some(left) = &head.left {
        x = Option::Some(collapse(left));
    }
    if let Some(right) = &head.right {
        y = Option::Some(collapse(right));
    }
    let x = if let Some(x) = x {
        x
    } else {
        0
    };
    let y = if let Some(y) = y {
        y
    } else {
        0
    };
    match head.op {
        OP::Sub => {
            x-y
        },
        OP::Add => {
            x+y
        },
        OP::Mul => {
            x*y
        },
        OP::Div => {
            if y ==0 {
                panic!("attempted divide-by-zero operation.");
            }
            x/y
        },
        OP::Id(x) => {
            x
        }
    }
}

struct TreeNode {
    op:OP,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>
}

impl TreeNode {
    fn new(l: TreeNode, r: TreeNode, op: OP) ->TreeNode {
        return TreeNode {
            left: Option::Some(Box::new(l)),
            right: Option::Some(Box::new(r)),
            op:op
        }
    }
}

enum OP {
    Mul,
    Div,
    Add,
    Sub,
    Id(i32)
}

fn add(l: TreeNode, r: TreeNode) -> TreeNode {
    TreeNode::new(l, r, OP::Add)
}

fn sub(l: TreeNode, r: TreeNode) -> TreeNode {
    TreeNode::new(l, r, OP::Sub)
}

fn mul(l: TreeNode, r: TreeNode) -> TreeNode {
    TreeNode::new(l, r, OP::Mul)
}

fn div(l: TreeNode, r: TreeNode) -> TreeNode {
    TreeNode::new(l, r, OP::Div)
}

fn id(v: i32) -> TreeNode {
    TreeNode {
        left: Option::None,
        right: Option::None,
        op:OP::Id(v)
    }
}

#[cfg(test)]
mod test {
    use crate::bstree::{*};
    #[test]
    pub fn testBSTree() {
        let res = collapse(&Box::new(add(
            add(id(8)
                ,
                div(
                    id(10),
                    id(2)

                )
            ),
            sub(id(10)
                ,

                mul(
                    id(2),
                    id(2)
                )
            )
        )));
        println!("exe rsult = {}",res);
    }
}