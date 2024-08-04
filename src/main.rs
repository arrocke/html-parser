use crate::tokenizer::Tokenizer;

mod tokenizer;

fn main() {
    let input = "
        <!DOCTYPE html>
        <head>
            <title>Register</title>
        </head>
        <body>
            <form method='POST' action='/form' >
                <label for=email >Email</label>
                <input id=email name=email type=email />
                <button>Submit</button>
            </form>
        </body>
    " ;
    let tokenizer = Tokenizer::new(input);

    for token in tokenizer {
        println!("{:?}", token);
    }
}
