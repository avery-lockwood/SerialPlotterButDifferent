fn main(){
    let mut arry: [i32; 10] = [0; 10];

    for i in 0..10{
        arry[i] = i as i32;
    }

    for x in arry {
        println!("{x} ");
    }

    println!("{}",'\n');

    let slicey = &arry[0..5];

    for i in slicey.iter(){
        println!("{}", i);
    }
}