//esteban donaire assignment 2

fn is_even(n: i32) -> bool{
    if n%2==1 {
        return false;
    }
    else{
        return true;
    }
}


fn main(){

    let num_array: [i32;10] = [1,2,3,4,5,6,7,8,9,10];

    for idx in 0..num_array.len(){
        if num_array[idx] % 3 == 0 && num_array[idx] % 5 == 0 {
            println!("FizzBuzz");
        } else if num_array[idx] % 3 == 0 {
            println!("Buzz");
        } else if num_array[idx] % 5 == 0 {
            println!("Fizz");
        } else if num_array[idx] % 2 == 0{
            println!("True");
        }
        else{
            println!("False");
        }
    }

    let mut i = 9;
    let mut sum=0;

    while i != 0 {
        sum+=num_array[i];
        i-=1;
    } 
    sum+=num_array[0];



        
    println!("{} is the sum of all numbers in the array",sum);

    let mut largest = num_array[0];
    for ix in 0..num_array.len(){
        if largest<num_array[ix]{
            largest=num_array[ix];
        }

    }
    println!("{} is the largest number in the array",largest);
}