use dom::Arena;
use parser::parse;

mod tokenizer;
mod parser;
mod dom;

fn main() {
    let mut arena = Arena::new();
    let mut html = arena.node("html");
    let mut head = arena.node("head");
    arena.append(&mut html, &mut head);

    /*
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
    */
}
