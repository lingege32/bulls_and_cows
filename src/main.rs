use rand::Rng;
use std::io;
use std::io::Write;


fn main() {
    let mut answer_db = generate_the_answer_database();
    while answer_db.len()>1 {
        let guess_ans = guess_a_number(&answer_db);
        println!("is answer {:?} ? ",guess_ans);
        let answer_tag = user_give_me_point();
        answer_db.retain(|&ans| diff_two_array(ans,guess_ans)==answer_tag);
    }
    if answer_db.len()==0 {
        println!("wtf");
    }
    println!("Answer is {:?}",answer_db[0]);
}

fn diff_two_array(lhs:[u8;4], rhs:[u8;4]) -> (u8,u8) {
    let mut a = 0u8;
    let mut b = 0u8;
    for (lindex,lval) in lhs.iter().enumerate() {
        for (rindex,rval) in rhs.iter().enumerate(){
            if lval==rval {
                if lindex==rindex {
                    a+=1;
                }
                else{
                    b+=1;
                }
            }
        }
    }
    if a+b>4 {
        panic!()
    }
    (a,b)
}


fn user_give_me_point() -> (u8,u8) {
    loop{
        print!("give me the correct point, like 1A2B:  ");
        io::stdout().flush().unwrap();

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        // check the guess is correct
        guess = guess.trim().to_string();
        if guess.len()==(4 as usize) &&
            (guess.as_bytes()[1]==('A' as u8) || guess.as_bytes()[1] == 'a' as u8) &&
            (guess.as_bytes()[3]==('B' as u8) || guess.as_bytes()[3] == 'b' as u8) {
            let a = guess.as_bytes()[0] - '0' as u8;
            let b = guess.as_bytes()[2] - '0' as u8;
            if a+b<5 {
                return (a,b);
            }
        }
    }
}

fn generate_the_answer_database() -> Vec<[u8;4]> {
    let mut tmp = Vec::new();
    for num in 0..10000 {
        match check_is_legal_answer_and_return(num) {
            Some(x) => tmp.push(x),
            None    => continue
        }
    }
    tmp
}

fn check_is_legal_answer_and_return(input : i32) -> Option<[u8;4]> {
    if input>10000 || input<0 {
        None
    }
    else{
        let thousand = (input/1000    ) as u8 ;
        let hundred  = (input%1000/100) as u8 ;
        let ten      = (input%100/10  ) as u8 ;
        let one      = (input%10      ) as u8 ;
        if thousand==hundred ||
           thousand==ten     ||
           thousand==one     ||
           hundred==ten      ||
           hundred==one      ||
           ten==one {
            None
        }
        else{
            Some([thousand,hundred,ten,one])
        }
    }
}

fn guess_a_number(ans_db : &Vec<[u8;4]>) -> [u8;4]{
    let index = rand::thread_rng().gen_range(0,ans_db.len()+1);
    ans_db[index]
}
