use std::env;
use std::fs;
use std::path::Path;

use dialoguer::{
    Select,
    theme::ColorfulTheme
};

use console::Term;


fn read_and_write(path: String)  {
    println!("(￣^￣ )ゞ Yosh! I am on it!");

    fs::copy(path, ".env").expect("Expected to copy file");

    println!("Done!");
}

fn switch() {

    let is_directory: bool = Path::new("./env").is_dir();

    if !is_directory {
       println!("(ノಠ益ಠ)ノ彡┻━┻ I could not find env directory! Did you even read documentation ಠ_ಠ ?!")
    } else {

        let paths = fs::read_dir("./env").unwrap();
        let mut options = Vec::new();
        let mut path_names = Vec::new();

        for path in paths {
            let path_name = path.unwrap().path().display().to_string();
            let file = path_name.replace("./env/", "");
            let file_name = file.replace(".env", "").replace(".", "");

            options.push(file_name);
            path_names.push(path_name);
        }
        
        println!("Which one would you prefer today?");

        let selection = Select::with_theme(&ColorfulTheme::default())
            .items(&options)
            .default(0)
            .interact_on_opt(&Term::stderr());

        match selection {
            Ok(Some(index)) => read_and_write(path_names[index].to_string()),
            Ok(None) => println!("(╬≖_≖) You did not select anything :C "),
            Err(_) => println!("Oops, I could not find env files!")
        }
    }
}

fn test(){
    let file_path: &str = "./test-env.json";
    let is_config: bool = Path::new(file_path).is_file();
    if !is_config {
        println!("(ノಠ益ಠ)ノ彡┻━┻ I could not find test-env.json file! Did you even read documentation ಠ_ಠ ?!")
     }  else {
        println!("Aha! You want to test a feature! Which variable you want to switch?");
     }
}

fn main() -> std::io::Result<()> {

    let args: Vec<String> = env::args().collect();
   
    if args.len() > 1 {
        let query = &args[1];
        if query == "switch" {
           return  Ok(switch())
        } else if query == "test" {
           return  Ok(test())
        }
    }

    println!("(•◡•) Hello developer, what do you want to do with your ENV today?");

    let items = vec!["Switch env", "Test a feature"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    match selection {
        Some(index) => if index == 0 {switch()} else {test()},
        None => println!("(╬≖_≖) You did not select anything :C ")
    }

    Ok(())
}
