
fn add_dyn<'a>(scr:&'a mut dyn Iterator<Item=&i32>) -> Box<dyn Iterator<Item=i32>+'a> {

    Box::new(scr.map(|x|x+1))
}

fn add_impl<'a>(scr:impl Iterator<Item=&'a i32>+'a)->impl Iterator<Item=i32>+'a{
    scr.map(|x|x+1)
}

fn main() {

    let ite;


        let v=vec![1,2,3];
        ite=v.iter();


    let ret=add_impl(ite);

    for i in ret {
        println!("{}",i);
    }


}

