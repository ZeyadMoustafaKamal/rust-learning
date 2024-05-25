use std::collections::HashMap;

fn main() {
    // vectors

    // let v = vec![1, 2, 3];
    /* 
    let mut v = Vec::new();
    v.push(1);
    v.push(3);
    v.push(5);

    println!("The values of the vec are {:?}", v)
    */
    /*
    let v = vec![1, 2, 3];
    let third = v.get(2);
    match third {
        Some(third) => println!("The value is {third}"),
        None => println!("There is not third value")
   }
    */

    // strings

    //  let mut s = String::new();
    /* 
    let data = "Some data";

    let mut s = data.to_string();

    let s2 = "+ some other data";

    s.push_str(s2);

    println!("s2 is {s2}");
    */


    // hash maps

    /*
    let mut scores = HashMap::new();
    scores.insert("Blue".to_string(), 10);
    scores.insert("Yellow".to_string(), 50);
    
    let team_name = "Blue".to_string();
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("The score is {score}");
    
    scores.entry("Red".to_string()).or_insert(50);
    println!("The scores are {:?}", scores)
    */

    /*
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    */

    fn get_median(nums: &Vec<i32>) -> i32 {
        // Given a reference to a vector stores i32 integeres and returns the median for these
        // numbers but please note that this will not work if the vec.len() is an odd number as
        // this will require the function to return a float number which means that I need to
        // return i32 or float which is something we didn't learn for now.
        let nums_len = nums.len();
        nums[nums_len / 2] 
    }

    let mut nums = vec![5, 3, 6, 1, 2]; // 3
    nums.sort();
    
    let median = get_median(&nums);
    println!("The median is {median}")
}


