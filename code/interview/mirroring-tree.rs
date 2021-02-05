# use std::convert::Into;
#
struct Node {
    val: i32,
    left: NodeLink,
    right: NodeLink,
}

type NodeLink = Option<Box<Node>>;

# fn construct_tree() -> Box<node> {
#     let l3left1 = node {
#         val: 4,
#         left: none,
#         right: none,
#     };
#
#     let l3right1 = node {
#         val: 5,
#         left: none,
#         right: none,
#     };
#
#     let l3left2 = node {
#         val: 6,
#         left: none,
#         right: none,
#     };
#
#     let l3right2 = node {
#         val: 7,
#         left: none,
#         right: none,
#     };
#
#     let l2left = node {
#         val: 2,
#         left: some(Box::new(l3left1)),
#         right: some(Box::new(l3right1)),
#     };
#
#
#     let l2right = node {
#         val: 3,
#         left: some(Box::new(l3left2)),
#         right: some(Box::new(l3right2)),
#     };
#
#     Box::new(node {
#         val: 1,
#         left: some(Box::new(l2left)),
#         right: some(Box::new(l2right)),
#     })
# }
#
# fn construct_mirror() -> Box<node> {
#     let l3left1 = node {
#         val: 7,
#         left: none,
#         right: none,
#     };
#
#     let l3right1 = node {
#         val: 6,
#         left: none,
#         right: none,
#     };
#
#     let l3left2 = node {
#         val: 5,
#         left: none,
#         right: none,
#     };
#
#     let l3right2 = node {
#         val: 4,
#         left: none,
#         right: none,
#     };
#
#     let l2left = node {
#         val: 3,
#         left: some(Box::new(l3left1)),
#         right: some(Box::new(l3right1)),
#     };
#
#
#     let l2right = node {
#         val: 2,
#         left: some(Box::new(l3left2)),
#         right: some(Box::new(l3right2)),
#     };
#
#     Box::new(node {
#         val: 1,
#         left: some(Box::new(l2left)),
#         right: some(Box::new(l2right)),
#     })
# }
#
# impl into<vec<i32>> for Box<node> {
#     fn into(mut self) -> vec<i32> {
#         let v_left: vec<i32>;
#         let v_right: vec<i32>;
#         v_left = if let some(node) = self.left.take() {
#             node.into()
#         } else {
#             vec::new()
#         };
#
#         v_right = if let some(node) = self.right.take() {
#             node.into()
#         } else {
#             vec::new()
#         };
#
#        let mut v = vec::new();
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
