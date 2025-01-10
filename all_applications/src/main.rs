mod to_do_list;
mod time_tracker;
mod notbook;

use std::io;
fn main() {

    println!("Chose one of CLI applications");
    println!("1.TO-DO LIST MANAGER");
    println!("2.TIME TRACKER");
    println!("3.CLI NOTEBOOK");


    let mut input=String::new();
    io::stdin().read_line(&mut input).expect("Enter vaild input");

    match input.trim() 
    {                                                       
        "1" => {to_do_list::main();}
        "2" => {time_tracker::main();}
        "3" => {notbook::main();}
        _=>{println!("Enter valid input");}
        
    }
}