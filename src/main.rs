use parser::parse;

mod tokenizer;
mod parser;

fn main() {
    let input = "
        <!DOCTYPE html>
        <html>
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
        </html>
    " ;

    parse(input);
}
