#[macro_use]
extern crate nom;

use std::iter::FromIterator;

use nom::IResult;

named!(text_char <&str, char>, none_of!("\u{000A}\u{000D}\u{005C}\u{007B}"));

named!(text <&str, String>, do_parse!(
    chars: many1!(text) >>
    (String::from_iter(chars))
));

#[test]
fn it_stack_overflow_error() {
    let source = "foo
bar";

    let remaining = "\nbar";

    let res = text(source);
    println!("{:?}", res);
    match res {
        IResult::Done(ref i, ref o) => println!("i: {} | o: {:?}", i, o),
        _ => println!("error")
    }

    assert_eq!(res, IResult::Done(remaining, "foo".to_string()));
}
