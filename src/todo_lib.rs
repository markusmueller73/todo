use std::env;
use std::fs::{copy, create_dir, File};
use std::io;
// the imports make me wonder sometimes why these aren't available with use std::io::*;
use std::io::{BufReader, BufWriter, IsTerminal};
use std::io::prelude::*;
use std::path::Path;
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

pub const CMD_ADD: &str = "add";
pub const CMD_DONE: &str = "done";
pub const CMD_EDIT: &str = "edit";
pub const CMD_HELP: &str = "help";
pub const CMD_LIST: &str = "list";
pub const CMD_REMOVE: &str = "remove";
pub const CMD_RESET: &str = "reset";
pub const CMD_RESTORE: &str = "restore";

const APP_SUBDIR: &str = ".r_todo";
// const APP_CONFIG: &str = "todo.conf";
const APP_DATABASE: &str = "todo.data";
const APP_BACKUP: &str = "todo.data.bak";

/// Structure for internal use, every task is stored in this simple structure.
#[derive(Clone, Debug)]
struct TodoItem {
    index: u32,
    task: String,
    start: u64,
    is_done: bool,
}

impl TodoItem {

    /// Create a new ```TodoItem```
    fn new() -> TodoItem {
        TodoItem {
            index: 0,
            task: String::default(),
            start: 0,
            is_done: false,
        }
    }

    /// Create a new ```TodoItem``` from the parameters
    fn from(ix: u32, tsk: String, time: u64, done: bool) -> TodoItem {
        TodoItem {
            index: ix,
            task: tsk,
            start: time,
            is_done: done,
        }
    }

    /// Modify an existing ```TodoItem```
    /// planned, but never used, the underscore ```_``` preserves compiler warnings
    fn _set(&mut self, ix: u32, tsk: String, time: u64, done: bool) {
        self.index = ix;
        self.task = tsk;
        self.start = time;
        self.is_done = done;
    }

}

/// The container structure for the task database.
///
/// From the ```main()``` function the arguments where collected in a
/// ```Vec<String>``` and skipping the first argument (the program executable name)
/// the rest of the Vector was passing by ref.
///
/// *This works, but is a bit complicated:* ```Var.v.Subfield```
/// *must learn more about other "Collections", but the community says,
/// that Vector is much faster than e.g. LinkedLists.*
pub struct TodoDatabase {
    v: Vec<TodoItem>,
}

impl TodoDatabase {

    /// Create a new ```TodoDatabase```
    fn new() -> TodoDatabase {
        TodoDatabase {
            v: Vec::new(),
        }
    }

    /// Load the CSV file and initialize the Structure with their childs.
    ///
    /// The path is OS depended. The HOME directory environment variables
    /// will be used with the subdir ```.r_todo```to save the CSV file.
    ///
    /// For example the Linux path: ```/home/USERNAME/.r_todo/todo.data```
    pub fn load() -> TodoDatabase {

        let mut todo_db = TodoDatabase::new();
        let app_dir = get_os_data_dir();

        let file_name = Path::new(&app_dir).join(APP_SUBDIR).join(APP_DATABASE);
        if file_name.exists() {

            let file = match File::open(&file_name) {
                Ok(file) => file,
                Err(err) => {
                    eprintln!("Couldn't open {} (error: {}).", file_name.display(), err);
                    process::exit(1)
                }
            };

            let reader = BufReader::new(file);
            for line in reader.lines() {

                let l = line.unwrap_or_default();

                if l.starts_with(";") || l.starts_with("#") || l.starts_with(" ") || l.is_empty() {
                    continue;
                }

                // that's nice and fast
                let l_v: Vec<&str> = l.split(';').collect();
                if l_v.len() == 4 {
                    let ds = TodoItem::from(
                        l_v[0].parse::<u32>().unwrap(),
                        l_v[1].to_string(),
                        l_v[2].parse::<u64>().unwrap(),
                        l_v[3].parse::<bool>().unwrap(),
                    );
                    todo_db.v.push(ds);
                }

            }

        } else {
            println!("ToDo database does not exist, creating a new one.");
        }

        todo_db

    }

    /// Save the database as a simple CSV file.
    pub fn save(&self) {

        let app_dir = get_os_data_dir();
        let app_subdir = Path::new(&app_dir).join(APP_SUBDIR);
        if !app_subdir.exists() {
            let _dir = match create_dir(&app_subdir) {
                Err(err) => {
                    eprintln!("Can't create directory: {} (error: {})", app_subdir.display(), err);
                    process::exit(1)
                }
                Ok(dir) => dir,
            };
        }
        let file_name = app_subdir.join(APP_DATABASE);

        let file = match File::create(&file_name) {
            Ok(file)    => file,
            Err(err)    => {
                            eprintln!("Can't create database file: {} (error: {})", file_name.display(), err);
                            process::exit(1)
            }
        };

        let mut writer = BufWriter::new(file);
        writer.write_fmt(format_args!("# ToDo list database\n\n")).unwrap();
        for ds in &self.v {
            writer.write_fmt(format_args!("{};{};{};{}\n", ds.index, ds.task, ds.start, ds.is_done)).unwrap();
        }

        // save the file operation to disk
        writer.flush().unwrap();

    }

    /// Add a new task to the database.
    ///
    /// **Command:**
    ///
    /// ```todo add Add an interactive mode to the todo CLI command.```
    ///
    /// or with quotations:
    ///
    /// ```todo add "Add some documentation to the todo source code."```
    pub fn add(&mut self, argv: &[String]) {

        let mut ds =  TodoItem::new();

        // needed because the dataset (ds) is consumed here: self.v.push(ds);
        let ix = self.get_highest_id() + 1;
        ds.index = ix;

        let mut new_task = String::new();
        for i in argv {
            new_task.push_str(i);
            new_task.push(' ');
        }
        ds.task = new_task.trim_end().to_string();

        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(t) => ds.start = t.as_secs(),
            Err(_) => eprintln!("Error: SystemTime::UNIX_EPOCH."), // can this really happen?
        };

        self.v.push(ds);

        // escape sequences for colourful messages must be in Hex format, not Octal like in
        // linux terminals, seems to be an issue in Rust
        if io::stdout().is_terminal() {
            println!("Added a task with id \x1B[92m{}\x1B[39m.", ix);
        } else {
            println!("Added a task with id {}.", ix);
        }

    }

    /// Mark a task as done, you need the ID for the task, get the ID's from ```todo list```
    ///
    /// **Command:**
    ///
    /// ```todo done 2```
    pub fn done(&mut self, argv: &[String]) {

        let item: u32 = argv[0].parse::<u32>().unwrap_or_default();
        if item > 0 {

            for ds in &mut self.v {

                if ds.index == item {

                    ds.is_done = true;
                    if io::stdout().is_terminal() {
                        println!("Task with id \x1B[92m{}\x1B[39m marked as done.", item);
                    } else {
                        println!("Task with id {} marked as done.", item);
                    }
                    break;

                }

            }

        }

    }

    /// Edit an existing task, you need the ID for the task, get the ID's from ```todo list```
    ///
    /// **Command:**
    ///
    /// ```todo edit "Add an interactive mode to the todo source."```
    pub fn edit(&mut self, argv: &[String]) {

        let item: u32 = argv[0].parse::<u32>().unwrap_or_default();

        if item > 0 {

            let mut new_task = String::from("");
            for i in &argv[1..] {
                new_task.push_str(i);
                new_task.push(' ');
            }

            // here I search for my internal task ID, it's the same as "Vector[i] + 1", but I want to try
            // another way to get the right dataset
            for ds in &mut self.v {

                if ds.index == item {
                    ds.task = new_task;
                    if io::stdout().is_terminal() {
                        println!("Task with id \x1B[92m{}\x1B[39m edited.", item);
                    } else {
                        println!("Task with id {} edited.", item);
                    }
                    break;
                }

            }

        }

    }

    /// Show the task list, with all ID's
    ///
    /// **Command:**
    ///
    /// ```todo list```
    pub fn list(&mut self) {

        if self.v.len() > 0 {

            if io::stdout().is_terminal() {
                println!("\n\x1B[1mTask List:\x1B[0m\n\x1B[1m----------\x1B[0m\n");
            } else {
                println!("\nTask List:\n----------\n");
            }

            let mut ds_done: u32 = 0;
            let mut ds_open: u32 = 0;

            for ds in &mut self.v {

                let mut line = String::default();

                if io::stdout().is_terminal() {
                    if ds.is_done {
                        line.push_str("[\x1B[93mX\x1B[39m] ");
                        ds_done += 1;
                    } else {
                        line.push_str("[ ] ");
                        ds_open += 1;
                    }
                    line.push_str(format!("\x1B[92m{:2}.\x1B[39m ", ds.index).as_str());
                    if ds.is_done {
                        line.push_str(format!("\x1B[9m{}\x1B[0m", ds.task).as_str());
                    } else {
                        line.push_str(ds.task.as_str());
                    }
                } else {
                    if ds.is_done {
                        line.push_str("[X] ");
                        ds_done += 1;
                    } else {
                        line.push_str("[ ] ");
                        ds_open += 1;
                    }
                    line.push_str(format!("{:2}. ", ds.index).as_str());
                    if ds.is_done {
                        line.push_str(format!("{}", ds.task).as_str());
                    } else {
                        line.push_str(ds.task.as_str());
                    }
                }
                if ds.is_done == false {
                    line.push_str(since_string(ds.start).as_str());
                }

                println!("{}", line);

            }
            if ds_done == 1 {
                if io::stdout().is_terminal() {
                    println!("\nFound \x1B[92m{}\x1B[39m open task(s) and \x1B[93m{}\x1B[39m is finished.", ds_open, ds_done);
                } else {
                    println!("\nFound {} open task(s) and {} is finished.", ds_open, ds_done);
                }
            } else {
                if io::stdout().is_terminal() {
                    println!("\nFound \x1B[92m{}\x1B[39m open task(s) and \x1B[93m{}\x1B[39m are finished.", ds_open, ds_done);
                } else {
                    println!("\nFound {} open task(s) and {} are finished.", ds_open, ds_done);
                }
            }

        } else {
            println!("There are no tasks in the list.");
        }

        println!("");

    }

    /// Remove one task, you need the ID for the task, get the ID's from ```todo list```.
    /// The user will be prompted for confirmation.
    ///
    /// **Command:**
    ///
    /// ```todo remove 2```
    pub fn remove(&mut self, argv: &[String]) {

        let item: u32 = argv[0].parse::<u32>().unwrap_or_default();
        if item > 0 {

            // this search is like the example of the official documentation
            let pos = &self.v.iter().position(|i| i.index == item).unwrap();
            let task = &self.v[pos.to_owned()].task;

            println!("Are you sure to delete this task (y|n)?\n-> {}", task);
            let mut input = [0];
            let mut stdin = io::stdin();
            let mut delete: bool = false;

            if stdin.is_terminal() {

                let _ = stdin.read(&mut input);

                loop {

                    match input[0] as char {
                        'y' | 'Y' => {
                            delete = true;
                            break;
                        }
                        _ => break,
                    }

                }

            }

            if delete {

                self.v.remove(pos.to_owned());

                if io::stdout().is_terminal() {
                    println!("Task with id \x1B[92m{}\x1B[39m was removed.", item);
                } else {
                    println!("Task with id {} was removed.", item);
                }

                let mut i: u32 = 0;
                for ds in &mut self.v {
                    i += 1;
                    ds.index = i;
                }

            } else {
                println!("Aborted.");
            }

        }

    }

    /// Reset the whole database and make a backup before resetting.
    /// The user will be prompted for confirmation.
    ///
    /// **Command:**
    ///
    /// ```todo reset```
    pub fn reset(&mut self) {

        println!("Are you sure to reset the database, all entries will be lost (y|n)?");
        let mut input = [0];
        let mut stdin = io::stdin();//.read(&mut input);
        let mut delete: bool = false;

        // prevent here, that during scripting the script waits for input
        if stdin.is_terminal() {

            let _ = stdin.read(&mut input);

            loop {

                match input[0] as char {
                    'y' | 'Y' => {
                        delete = true;
                        break;
                    }
                    _ => break,
                }

            }

        }

        if delete {

            let app_dir = get_os_data_dir();
            let old_db = Path::new(&app_dir).join(APP_SUBDIR).join(APP_DATABASE);
            let bak_db = Path::new(&app_dir).join(APP_SUBDIR).join(APP_BACKUP);

            let _cp = match copy(&old_db, &bak_db) {
                Err(err) => {
                    eprintln!("Can't copy database: {} (error: {})", old_db.display(), err);
                    process::exit(1)
                }
                Ok(cp) => cp,
            };

            self.v.clear();

            if io::stdout().is_terminal() {
                println!("\x1B[91mThe database was resetted, and is empty.\x1B[39m");
            } else {
                println!("The database was resetted, and is empty.");
            }

        } else {

            println!("Aborted.");

        }

    }

    /// Restore the backup from the last resetted database
    ///
    /// **Command:**
    ///
    /// ```todo restore```
    pub fn restore(&mut self) {

        print!("Restoring last database backup...");

        let app_dir = get_os_data_dir();
        let old_db = Path::new(&app_dir).join(APP_SUBDIR).join(APP_DATABASE);
        let bak_db = Path::new(&app_dir).join(APP_SUBDIR).join(APP_BACKUP);

        let _cp = match copy(&bak_db, &old_db) {
            Err(err) => {
                eprintln!("Can't copy database: {} (error: {})", old_db.display(), err);
                process::exit(1)
            }
            Ok(cp) => cp,
        };

        println!("done.");

    }

    /// For internal use.
    ///
    /// The function do what the name says, looks for the highest ID in the database.
    fn get_highest_id(&self) -> u32 {
        let mut id: u32 = 0;
        for ds in &self.v {
            if ds.index > id { id = ds.index; }
        }
        id
    }

}

/// For internal use.
///
/// Very easy function to get the user dir depended on the operating system
/// you work with this program on the command line, so the environment
/// variables will be set.
fn get_os_data_dir() -> String {
    #[cfg(target_os="windows")]
    let d = env::var("LOCALAPPDATA").unwrap_or_else(|err| {
        eprintln!("could not find %LOCALAPPDATA%: {}", err);
        String::from(".")
    });
    #[cfg(target_os="linux")]
    let d = env::var("HOME").unwrap_or_else(|err| {
        eprintln!("could not find %HOME: {}", err);
        String::from(".")
    });
    d
}

/// For internal use.
///
/// Very easy time function, to be independed from the OS time functions I only use the system time
/// for this duration calculation it is accurate enough.
fn time_diff(secs: u64) -> u64 {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let diff: u64 = now.as_secs() - secs;
    diff
}

/// For internal use.
///
/// Get the ```String``` from the ```time_diff()``` function.
fn since_string(secs: u64) -> String {
    let diff: u64 = time_diff(secs);
    let s: String;
    if diff < 60 {
        s = format!("(since {} second(s))", diff);
    } else if diff >= 60 && diff < 3_600 {
        s = format!("(since {} minute(s))", diff / 60);
    } else if diff >= 3_600 && diff < 86_400 {
        s = format!("(since {} hour(s))", diff / 3_600);
    } else {
        s = format!("(since {} day(s))", diff / 86_400);
    }
    s
}

/// Print out some help Text.
pub fn help(name: &str) {
    println!("\nUsage:");
    println!("------");
    println!("{} [<OPTION>] or [COMMAND] [<ARGUMENTS>]\n", name);
    println!("Options:");
    println!("--------");
    println!("-h, --help         show this help");
    println!("-v, --version      show the program version and exit");
    println!("");
    println!("Commands:");
    println!("---------");
    println!("{}, {}, {}, {}, {}, {}, {}", CMD_ADD, CMD_DONE, CMD_EDIT, CMD_LIST, CMD_REMOVE, CMD_RESET, CMD_RESTORE);
    println!("");
    println!("Command usage:");
    println!("--------------");
    println!("{}\t[TASK]        \tadd the TASK to the todo list", CMD_ADD);
    println!("{}\t[INDEX]       \tmark the task with INDEX as done", CMD_DONE);
    println!("{}\t[INDEX] [TASK]\treplace the task with INDEX with TASK", CMD_EDIT);
    println!("{}                \tprint out all tasks", CMD_LIST);
    println!("{}\t[TASK]        \tremove task with INDEX fro list", CMD_REMOVE);
    println!("{}\t              \treset (delete) the whole database", CMD_RESET);
    println!("{}\t              \trestore a backup from the last deleted database", CMD_RESTORE);
    println!("");
}
