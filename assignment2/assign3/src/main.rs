//Esteban Donaire Assignment 3

fn check_guess(guess: i32, secret: i32) -> i32{
    if guess == secret {
        return 0;
    }
    else if guess > secret {
        return 1;
    }
    else{
        return -1;
    }
}

fn main() {
    let mut secret_num: i32 = 7;

    let mut guess_num: i32 = 0;

    let mut num_guesses: i32 = 1;

    while check_guess(guess_num, secret_num) != 0{

        if check_guess(guess_num, secret_num) < 0{
            println!("Too low!");
        }
        else{
            println!("Too high!");
        }
        guess_num+=1;
        num_guesses+=1;

    }

    println!("Correct! It took {} attempts!", num_guesses);
}
