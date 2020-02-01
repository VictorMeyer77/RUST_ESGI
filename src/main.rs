use std::io::prelude::*;
use std::process::{Command, Stdio};
use std::env;
use colored::Colorize;
use std::process;


fn main() {

    // parse arguments

    let mut to_sort = false;
    let mut to_count = false;
    let mut to_color = "no_color";

    let mut i = 1;
    let args_vec: Vec<String> = env::args().collect();
    while i < args_vec.len() {
        match args_vec[i].as_str() {
            "--al" => to_sort = true,
            "--ct" => to_count = true,
            "--cl" => {
                if i + 1 < args_vec.len() && (args_vec[i + 1].as_str() == "blue"|| args_vec[i + 1].as_str() == "red" || args_vec[i + 1].as_str() == "green") {
                    to_color = args_vec[i + 1].as_str()
                }
                else{
                    exit_with_error()
                }
            },
            "green" => {i += 1; continue;},
            "red" => {i += 1; continue;},
            "blue" => {i += 1; continue;},
            _ => println!("Command not found: {}", args_vec[i])
        }
        i += 1;
    }

    execute(to_sort, to_count, to_color)


}

// execute and return ls command
fn get_ls_result() -> String{

    let error = "error".to_string();

    // ls command execution
    let process = match Command::new("ls")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn() {
        Err(_err) => panic!("Failed to run ls command"),
        Ok(process) => process,
    };

    // store and resturn ls result
    let mut ls_result = String::new();
    match process.stdout.unwrap().read_to_string(&mut ls_result) {
        Err(_err) => error,
        Ok(_) => return ls_result,
    }

}

// sort files and folder by ascending alphabetic order
fn sort_alphabetic(ls_result: String) -> String{

    let mut arguments_vector:Vec<&str> = ls_result.split("\n").collect();
    let _sort_args= arguments_vector.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
    return  arguments_vector.join("\n");

}

// add a line with total of folders and files
fn count_files(ls_result: String) -> String{

    // get nb rows
    let mut arguments_vector:Vec<&str> = ls_result.split("\n").collect();
    let count = arguments_vector.len();
    // add total line
    let string_count: String = format!("Total files and floders: {}", count);
    arguments_vector.push(string_count.as_str());
    return arguments_vector.join("\n");

}

// print ls result with specified color
fn print_color(ls_result: String, color: &str){

    match color{

        "blue" => println!("{}", ls_result.as_str().blue()),
        "red" => println!("{}", ls_result.as_str().red()),
        "green" => println!("{}", ls_result.as_str().green()),
        _ => println!("Unknow color")
    }
}

// stop program and print documentation
fn exit_with_error(){

    println!("ls_rust --> ls command made with Rust");
    println!("--al --> sort by alphabetic order");
    println!("--red --> color rows");
    println!("--count --> count rows");
    process::exit(0x0100);

}

// launch all modification
fn execute(to_sort: bool, to_count: bool, to_color: &str){

    let mut ls_result = get_ls_result();

    // exit program if ls command failed
    if ls_result == "error".to_string() {
        exit_with_error();
    }

    if !to_count && !to_sort && to_color == "no_color" {

        // return ls result without modification if n parameters
        if env::args().count() == 1{
            println!("{}", ls_result)
        }
        else{
            exit_with_error()
        }

    }
    else{

        if to_sort{
            ls_result = sort_alphabetic(ls_result)
        }
        if to_count{
            ls_result = count_files(ls_result)
        }

        if to_color != "no_color"{
            print_color(ls_result, to_color)
        }
        else{
            println!("{}", ls_result)
        }

    }

}