mod average {

    // average value
    pub fn mean(irregular: &Vec<u32>) -> u32 {

        irregular.iter().sum::<u32>() / irregular.len::<u32>()

    }

    // when sorted, the value in the middle position
    pub fn median(irregular: &Vec<i32>){
    }

    // value that occurs most often
    pub fn mode(irregular: &Vec<i32>){
    }

}

fn main() {

    let irregular = vec![32, 12, 33, 8, 23];

    println!("{}", average::mean(&irregular));

}
