mod to_do_list;
mod time_tracker;
mod notbook;
mod weather;
use std::io;
fn main() {

    println!("==========>Chose one of CLI applications<==========");
    println!("1.TO-DO LIST MANAGER");
    println!("2.TIME TRACKER");
    println!("3.CLI NOTEBOOK");
    println!("4.WEATHER APP");


    let mut input=String::new();
    io::stdin().read_line(&mut input).expect("Enter vaild input");

    match input.trim() 
    {                                                       
        "1" => 
        {
            println!("=====>Commands to Run: ");
            println!("1. ADD TASK:          cargo run -- add 'Project Name' 10-01-2025");
            println!("2. LIST TASK:         cargo run -- list");
            println!("3. REMOVE TASK:       cargo run -- remove 1");
            println!("4. COMPLETE TASK:     cargo run -- complete 1");
            println!("");
            to_do_list::main();
        }
        "2" => 
        {

            println!("=====>Commands to Run: ");
            println!("1. TASK START:        cargo run -- start 'Project Name'");
            println!("2. TASK STOP:         cargo run -- stop");
            println!("3. TASK REPORT:       cargo run -- report");
            println!("");
            time_tracker::main();
        }
        "3" => 
        {
            println!("=====>Commands to Run: ");
            println!("1. ADD NOTE:        cargo run -- add --title 'title of notebook' --content 'content of notebook'");
            println!("2. VIEW NOTE:       cargo run -- view --title 'title name'");
            println!("3. DELETE NOTE:     cargo run -- delete --title 'title name'");
            println!("4. LIST OF NOTES:   cargo run -- list");
            println!("");
            notbook::main();
        }
        "4" => 
        {
            println!("=====>Commands to Run: ");
            println!("2. WEATHER OF CITY:       cargo run -- --city 'city name'");
            weather::main();
        }
        _=>{println!("Enter valid input");}
        
    }
}
