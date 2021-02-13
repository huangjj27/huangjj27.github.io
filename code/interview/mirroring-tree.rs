# use std::convert::Into;
#
struct Node {
    val: i32,
    left: NodeLink,
    right: NodeLink,
}

type NodeLink = Option<Box<Node>>;

# fn construct_tree() -> Box<Node> {
#     let l3left1 = Node {
#         val: 4,
#         left: None,
#         right: None,
#     };
#
#     let l3right1 = Node {
#         val: 5,
#         left: None,
#         right: None,
#     };
#
#     let l3left2 = Node {
#         val: 6,
#         left: None,
#         right: None,
#     };
#
#     let l3right2 = Node {
#         val: 7,
#         left: None,
#         right: None,
#     };
#
#     let l2left = Node {
#         val: 2,
#         left: Some(Box::new(l3left1)),
#         right: Some(Box::new(l3right1)),
#     };
#
#
#     let l2right = Node {
#         val: 3,
#         left: Some(Box::new(l3left2)),
#         right: Some(Box::new(l3right2)),
#     };
#
#     Box::new(Node {
#         val: 1,
#         left: Some(Box::new(l2left)),
#         right: Some(Box::new(l2right)),
#     })
# }
#
# fn construct_mirror() -> Box<Node> {
#     let l3left1 = Node {
#         val: 7,
#         left: None,
#         right: None,
#     };
#
#     let l3right1 = Node {
#         val: 6,
#         left: None,
#         right: None,
#     };
#
#     let l3left2 = Node {
#         val: 5,
#         left: None,
#         right: None,
#     };
#
#     let l3right2 = Node {
#         val: 4,
#         left: None,
#         right: None,
#     };
#
#     let l2left = Node {
#         val: 3,
#         left: Some(Box::new(l3left1)),
#         right: Some(Box::new(l3right1)),
#     };
#
#
#     let l2right = Node {
#         val: 2,
#         left: Some(Box::new(l3left2)),
#         right: Some(Box::new(l3right2)),
#     };
#
#     Box::new(Node {
#         val: 1,
#         left: Some(Box::new(l2left)),
#         right: Some(Box::new(l2right)),
#     })
# }
#
# impl Into<Vec<i32>> for Box<Node> {
#     fn into(mut self) -> Vec<i32> {
#         let v_left: Vec<i32>;
#         let v_right: Vec<i32>;
#         v_left = if let Some(node) = self.left.take() {
#             node.into()
#         } else {
#             Vec::new()
#         };
#
#         v_right = if let Some(node) = self.right.take() {
#             node.into()
#         } else {
#             Vec::new()
#         };
#
#        let mut v = Vec::new();
#        v.push(self.val);
#        v.extend(v_left.into_iter());
#        v.extend(v_right.into_iter());
#
#        v
#     }
# }
#
fn mirror(root: &mut Node) {
    let (mut tmp_left, mut tmp_right) = (NodeLink::None, NodeLink::None);

    if let Some(mut node) = root.left.take() {
        mirror(&mut node);
        tmp_left = Some(node);
    }

    if let Some(mut node) = root.right.take() {
        mirror(&mut node);
        tmp_right = Some(node);
    }

    root.left = tmp_right;
    root.right = tmp_left;
}

# #[cfg(test)]
# mod test {
#     use super::*;
#
#     #[test]
#     fn should_mirrored() {
#         let mut tree = construct_tree();
#         let expect = construct_mirror();
#
#         let mirror = mirror(&mut tree);
#         assert_eq!(<Box<Node> as Into<Vec<i32>>>::into(tree), <Box<Node> as Into<Vec<i32>>>::into(expect));
#     }
# }
