fn main(){
    let nums = vec![2,7,11,15];
    let target = 9;

    let result = two_sum(nums,target);

    assert_eq!(result, vec![0,1]);
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (i, num) in nums.iter().enumerate() {
        if nums.contains(&(target - num)) {
            return vec![i as i32, nums.iter().position(|x| x == (target - num)).unwrap() as i32];
        }
    }
    vec![]
}