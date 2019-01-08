mod average {

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
    pub fn mode(_irregular: &Vec<i32>){
    }

}

fn main() {

    let irregular = vec![15, 22, 32];

    println!("{}", average::mean(&irregular));

    println!("{}", average::median(&irregular)); 

}
