mod average {

    use std::collections::BTreeMap;

    // average value
    pub fn mean(irregular: &Vec<u32>) -> u32 {

        irregular.iter().sum::<u32>() / irregular.len() as u32

    }

    // when sorted, the value in the middle position
    pub fn median(irregular: &Vec<u32>) -> u32 {
        
        let mut sorted = irregular.to_vec();
        sorted.sort();

        let len = sorted.len() as u32;

        if len <= 1 { return 0 }
        let middle = (len / 2) as usize;

        if let 0 = len % 2 {

            (sorted[middle] + sorted[middle-1]) / 2

        } else {

            sorted[middle]

        }

    }

    // value that occurs most often
    pub fn mode(irregular: &Vec<u32>) -> u32 {

        let mut keys = irregular.to_vec();
        keys.dedup();

        let mut sum: BTreeMap<_, _> = keys.iter().zip(vec![0; keys.len()]).collect();
        for key in &keys {
            sum.insert(key, irregular.iter().filter(|&n| *n == *key).count());
        };

        **sum.iter().max_by(|x, y| x.1.cmp(y.1)).unwrap().0

    }

}

fn main() {

    let irregular = vec![15, 15, 22, 22, 22, 32, 32, 32, 15, 32, 15, 15];

    println!("{}", average::mean(&irregular));

    println!("{}", average::median(&irregular));

    println!("{}", average::mode(&irregular)); 

}
