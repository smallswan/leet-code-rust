/// 力扣（LCP 06. 拿硬币） https://leetcode-cn.com/problems/na-ying-bi/
pub fn min_count(coins: Vec<i32>) -> i32 {
    let mut times = 0;

    for num in coins {
        if num % 2 == 0 {
            times += num / 2;
        } else {
            times += num / 2 + 1;
        }
    }

    times
}
use std::collections::HashMap;
use std::collections::HashSet;
/// 力扣（LCP 07. 传递信息） https://leetcode-cn.com/problems/chuan-di-xin-xi/submissions/
pub fn num_ways(n: i32, relation: Vec<Vec<i32>>, k: i32) -> i32 {
    let mut map = HashMap::<i32, HashSet<i32>>::new();
    for re in relation {
        let key = re[0];

        match map.get_mut(&key) {
            Some(set) => {
                set.insert(re[1]);
            }
            None => {
                let mut set = HashSet::<i32>::new();
                set.insert(re[1]);
                map.insert(re[0], set);
            }
        }
    }
    println!("map:{:?}", map);
    let mut account = 0;
    search_recurse(&map, 0, n - 1, 1, k, &mut account);
    account
}

fn search_recurse(
    map: &HashMap<i32, HashSet<i32>>,
    search: i32,
    end: i32,
    level: i32,
    k: i32,
    count: &mut i32,
) -> bool {
    if let Some(set) = map.get(&search) {
        for val in set.iter() {
            print!("{}->{}->", search, *val);
            if level >= k {
                if set.contains(&end) {
                    *count += 1;
                    println!();
                    return true;
                } else {
                    println!();
                    return false;
                }
            }
            search_recurse(map, *val, end, level + 1, k, count);
        }
    };

    false
}

#[test]
fn min_count_test() {
    let coins: Vec<i32> = vec![4, 2, 1];
    let times = min_count(coins);

    println!("times:{}", times);

    let mut relation = vec![];
    relation.push(vec![0, 2]);
    relation.push(vec![4, 1]);
    relation.push(vec![2, 1]);
    relation.push(vec![3, 4]);
    relation.push(vec![2, 3]);
    relation.push(vec![1, 4]);
    relation.push(vec![2, 0]);
    relation.push(vec![0, 4]);

    let n = 5;
    let k = 3;

    let r = num_ways(n, relation, k);

    println!("r:{}", r);
}
