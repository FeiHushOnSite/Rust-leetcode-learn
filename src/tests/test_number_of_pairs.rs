#[test]
pub fn test_number_of_pairs() {
    let nums1 = vec![1, 3, 4];
    let nums2 = vec![1, 3, 4];
    let res = number_of_pairs(nums1, nums2, 1);
    assert_eq!(5, res)
}

fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i32 {
    let mut count = 0;
    for num1 in &nums1 {
        for num2 in &nums2 {
            if num1 % (num2 * k) == 0 {
                count += 1;
            }
        }
    }
    count
}
