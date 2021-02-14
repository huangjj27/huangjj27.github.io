# use std::cmp::Ordering;
#
/// 给出一个从小到大排列的数组，请实现一个函数，用二分法把指定数组 x 的位置找出来。若 x
/// 不存在，则返回 -1. 若 x 存在多个，请返回 x 在数组中第一次出现的位置
fn find(arr: Vec<i32>, x: i32) -> i32 {
    let (mut left, mut right) = (0, arr.len() - 1);
    loop {
        if left > right {
            return -1;
        }

        let mut mid = (left + right) / 2;
        match arr[mid].cmp(&x) {
            // 记得要排除已经命中的元素！
            Ordering::Less => left = mid + 1,
            Ordering::Greater => right = mid - 1,
            Ordering::Equal => {
                while mid >= 1 && arr[mid - 1] == x {
                    mid -= 1;
                }

                return mid as i32;
            },
        }
    }
}

# #[cfg(test)]
# mod test {
#     use super::*;
#
#     #[test]
#     fn should_return_minus1() {
#         let arr = vec![1, 3, 5];
#         let x = 2;
#
#         assert_eq!(find(arr, x), -1);
#     }
#
#     #[test]
#     fn should_return_mid() {
#         let arr = vec![1, 3, 5, 7, 9, 10, 10];
#         let x = 7;
#
#         assert_eq!(find(arr, x), 3);
#     }
#
#     #[test]
#     fn should_return_first() {
#         let arr = vec![1, 3, 5, 7, 9, 10, 10];
#         let x = 10;
#
#         assert_eq!(find(arr, x), 5);
#     }
# }
