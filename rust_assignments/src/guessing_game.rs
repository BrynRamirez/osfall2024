fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess < secret {
        return -1;
    }
    else if guess > secret {
        return 1;
    }
    else {
        return 0;
    }
}

pub fn main() {
    let secret = 47;
    let mut guess = 45;
    let mut ttl_guesses = 0;

    loop {
        // if guess > secret +1 to guess and +1 ttl_guess and go to the top of the loop
        if check_guess(guess, secret) == 1 {
            guess -= 1;
            ttl_guesses += 1;
            continue;
        }
        // if guess < secret -1 to guess and +1 ttl_guess and go to the top of the loop
        else if check_guess(guess, secret) == -1 {
            guess += 1;
            ttl_guesses += 1;
            continue;
        }
        // guess = secret the print ttl_guess and end loop
        else {
            println!("total guesses: {}", ttl_guesses);
            break;
        }
    }
}