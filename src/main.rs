// ToDo
// (c) 2024 by markus dot mueller dot 73 at hotmail dot de
// Small binary to manage your todo's on the command line
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and
// associated documentation files (the “Software”), to deal in the Software without restriction,
// including without limitation the rights to use, copy, modify, merge, publish, distribute,
// sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial
// portions of the Software.
//
// THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT
// NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
// IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
// WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
// SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//

//! # Command Line ToDo List
//!
//! small binary to manage your todo's on the command line
//!
//! (c) 2024 by markus dot mueller dot 73 at hotmail dot de
//!
//! This small project is a suggested excercise to learn the Rust programming language. It is one of
//! my first attempts. If you find the code snippets to complicated, suggestions are welcome.
//!
//! The tasks saved in a CSV file, the enviroment of Linux and Windows is supported, the tasks saved
//! in the users HOME directory.
//!
//! # Usage:
//!
//! **Show the help text:**
//!
//! ```todo --help```
//!
//! **Add a new task:**
//!
//! ```todo add Something importend to do!```
//!
//! Carefully, you can add tasks without quotation marks and todo can add the whole task, but if you
//! include special chars like forslash ```/``` or comma ```,``` the terminal will interpret it as
//! command arguments and this can result in unexpected behaviour. If you need this specials
//! chars, use quotation marks ```"``` for your task.
//!
//! **Show the list of tasks:**
//!
//! ```todo list```
//!
//! The shown ids are useful for other commands like ```done``` and ```edit```.
//!
//! **Mark task number 3 as done:**
//!
//! ```todo done 3```
//!
//! **Remove task number 2 from list:**
//!
//! ```todo remove 2```
//!
//! After removing a task, all tasks get a new consecutive ID, show the ```todo list``` to view the
//! new ID's
//!
//!
//! *This cli command can be used in scripts, but the user interaction e.g. ```todo reset```
//! will always be aborted.*
//!

mod todo_lib;

use std::env;
use crate::todo_lib::*;

/// like every Rust binary, this is the entry function.
///
/// The parsing of the command line arguments are done here with ```Vec<String>```.
fn main() -> std::result::Result<(), usize>{

    // loading the CSV file and initialize it as Vector in a Structure
    let mut todo_db = TodoDatabase::load();

    let prg_name = env::args().nth(0).unwrap();
    let version = option_env!("CARGO_PKG_VERSION").unwrap();
    let argv: Vec<String> = env::args().skip(1).collect();

    if argv.len() >= 1 {

        match argv[0].to_ascii_lowercase().as_str() {

            CMD_ADD => {
                todo_db.add(&argv[1..]);
            }

            CMD_DONE => {
                todo_db.done(&argv[1..]);
            }

            CMD_EDIT => {
                todo_db.edit(&argv[1..]);
            }

            CMD_LIST => {
                todo_db.list();
            }

            CMD_REMOVE => {
                todo_db.remove(&argv[1..]);
            }

            CMD_RESET => {
                todo_db.reset();
            }

            CMD_RESTORE => {
                todo_db.restore();
            }

            // maybe in the future
            // "-i" | "--interactive" => {
            //     // TODO: start interactive mode
            // }

            "-h" | "--help" | CMD_HELP => {
                help(&prg_name);
                return Ok(());
            }

            "-v" | "--version" => {
                    println!("{} v{}\n", prg_name, version);
                    return Ok(());
            }

            _ => {
                eprintln!("Unknown argument, try {} --help", prg_name);
                return Ok(());
            }

        }

    } else {

        println!("{} needs at least one argument, try --help", prg_name);

    }

    todo_db.save();

    Ok(())
}

