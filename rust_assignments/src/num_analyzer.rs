// 2
fn is_even(n: i32) -> bool {
    if n % 2 == 0 {
        return true;
    }
    else{
        return false;
    }
}

pub fn main() {
    // 1
    let arr:Vec<i32> = vec![12, 4, 7, 55, 6, 91, 94, 24, 47, 1];

    // 2 and 3
    for x in &arr {
        print!("{} is even {}", x, is_even(*x));
        if x % 3 == 0 && x % 5 == 0 {
            println!(" FizzBuzz");
        }
        else if x % 3 == 0 {
            println!(" Fizz");
        }
        else if x % 5 == 0 {
            println!(" Buzz");
        }
        else{
            println!(" {}", x);
        };
    }

    // 4
    let mut ittr = 0;
    let mut sum = 0;
    while ittr < arr.len(){
        sum += arr[ittr];
        ittr +=1;
    }
    println!("Sum of array: {}", sum);
    
    // 5
    let max:i32 = *arr.iter().max().unwrap();
    println!("largest num: {}", max);
}
