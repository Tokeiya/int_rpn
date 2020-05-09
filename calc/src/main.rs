use calc_core::splitter::split;

fn main() {

    let out=split("   ");

    println!("{}",out.len());

    for elem in out.iter().enumerate() {
        println!("{}:{}",elem.0,elem.1);
    }

}
