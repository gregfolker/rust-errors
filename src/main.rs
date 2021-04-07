// Project: rust-errors
// Author: Greg Folker

use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
	println!("Hello, World!");

    // The `panic!()` macro can be called
    // manually as an `assert()` as seen on
    // Line 10
    // panic!("crash and burn!");

    let _v = vec![1, 2, 3];

    // The `panic!()` call can come from a
    // library as well because of a bug
    // in our code. In this case, trying to
    // access an index that is out of bounds
    //
    // Uncomment Line 24 to and run the program
    // to observe the runtime panic
    // _v[99];

    let _f = File::open("hello.txt");

    // Example of using a `match` statement to panic
    // if there any problem is encountered while trying
    // to open the specified file
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error),
    // };

    let _f = match _f {
        Ok(file) => file,
        // This embedded `match` statement will create the file if the
        // reported error was 'No such file or directory'
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                // We still have to ensure the file was created successfully
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    // The above code can be written more concisely as follows
    // to avoid all of the nested `match` statements required for
    // additional error handling on subsequent file operations
    let _f2 = File::open("hello2.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello2.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // The `unwrap()` method can also be used as a shortcut to catch
    // errors immediately
    //
    // Line 70 reports no error, because we created the file 'hello.txt'
    // above. Uncomment line 71 to observe the error returned from `unwrap()`
    let _f3 = File::open("hello.txt").unwrap();
    // let _f4 = File::open("hello3.txt").unwrap();

    // The `expect()` method lets you choose the error message reported
    // by `panic!()`
    //
    // Again, Line 79 will report no error because the file exists at this
    // point. Uncomment Line 79 to observe the custom error message
    let _f5 = File::open("hello2.txt").expect("Failed to open hello2.txt");
    // let _f6 = File::open("hello4.txt").expect("Failed to open hello4.txt");

    // Example of handling a propegating error from a function call
    let u1 = read_username_from_file().unwrap();

    // Note: This code does not get reached if Line 84
    // resulted in an error
    println!("u1 is {}", u1);

    let u2 = read_username_from_file2().unwrap();

    println!("u2 is {}", u2);
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        // 'Propagate' the error back to the caller for handling
        // instead of handling it in the function
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// `read_username_from_file2()` has the same functionality
// as `read_username_from_file()` but implemented using
// the `?` operator
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
