 # Command Line ToDo List

 small binary to manage your todo's on the command line

 (c) 2024 by markus dot mueller dot 73 at hotmail dot de

 This small project is a suggested excercise to learn the Rust programming language. It is one of
 my first attempts. If you find the code snippets to complicated, suggestions are welcome.

 The tasks saved in a CSV file, the enviroment of Linux and Windows is supported, the tasks saved
 in the users HOME directory.

 # Usage:

 **Show the help text:**

 ```todo --help```

 **Add a new task:**

 ```todo add Something importend to do!```

 Carefully, you can add tasks without quotation marks and todo can add the whole task, but if you
 include special chars like forslash ```/``` or comma ```,``` the terminal will interpret it as
 command arguments and this can result in unexpected behaviour. If you need this specials
 chars, use quotation marks ```"``` for your task.

 **Show the list of tasks:**

 ```todo list```

 The shown ids are useful for other commands like ```done``` and ```edit```.

 **Mark task number 3 as done:**

 ```todo done 3```

 **Remove task number 2 from list:**

 ```todo remove 2```

 After removing a task, all tasks get a new consecutive ID, show the ```todo list``` to view the
 new ID's


 *This cli command can be used in scripts, but the user interaction e.g. ```todo reset```
 will always be aborted.*

