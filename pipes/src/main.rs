use std::io;

fn main() {
    println!("Pipes Que List! Choose a type of que: \n1) Light Cue\n2) Sound Cue\n3) Projection Cue");
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer);

    if buffer == "1"
    {
        println!("You added a lighting cue!")
    }
    else if buffer == "2"
    {
        println!("You added a sound cue!")
    }
    else if buffer == "3"
    {
        println!("You added a projection cue!")
    }


}
