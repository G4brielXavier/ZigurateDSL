use std::path::PathBuf;
use std::println;

use std::str::from_utf8;

use colored::Colorize;

use crate::cli::commands::Commands;
use crate::tools::sita::Sita;
use crate::core::error::ZigurateError;
use crate::enki_vm::enki::EnkiVM;

use crate::cli::{
    get_enki_data
};

use crate::core::{zigurage_compiler_simple, zigurate_compiler_enki};




pub fn match_command(cmd: &Commands) -> Result<(), ZigurateError> {

    let sita = Sita::gibil();

    let zigurate_name = format!("{}", "/|\\ ZIGURATE");
    let status_stable = format!("{}", "(beta 4)".yellow().italic());
    let version_zigurate = format!("{} — UR1 {}", zigurate_name.bold(), status_stable);

    match cmd {

        Commands::Run { path } => {

            let curr_path = sita.current_dir()
                .map_err(|e| ZigurateError::IOError(e))?;

            let path_cont = match path {
                Some(e) => Ok(e.clone()),
                None => {

                    let is_enki_project = sita.path_exists(&curr_path.join("Enki.yaml"))
                        .map_err(|e| ZigurateError::IOError(e))?;

                    if is_enki_project {
                        
                        let enki_data = get_enki_data(&sita, &curr_path)?;
                        let path_world = PathBuf::from(enki_data.main);

                        Ok(path_world)

                    } else {
                        Err(ZigurateError::ProjectNotFoundOrNotExist)
                    }

                }
            };

            match path_cont {

                Ok(main_path) => {

                    let content = sita.read_file(main_path)
                        .map_err(|e| ZigurateError::IOError(e))?;

                    let byte_content = content.as_bytes();


                    let enki_data = get_enki_data(&sita, &curr_path)?;

                    let cmd = format!("enki run");

                    let msg_2 = format!("");

                    println!();
                    println!("{}", version_zigurate);
                    println!("--- {} {} st.{}", "info:".bold().yellow(), enki_data.name.italic(), enki_data.stage.italic());
                    println!("--- {} {} {}", "cmd:".bold().yellow(), cmd.yellow().italic(), "!executed".bold().green());
                    println!("--- {} {}", "in:".bold().yellow(), curr_path.display().to_string().underline());
                    println!();
                    println!("{} {}", "Compiling EnkiVM...".bright_blue().bold(), msg_2);

                    // MAIN

                    let enki_instructions = zigurage_compiler_simple(byte_content)?;
                    let mut enki_vm = EnkiVM::on(enki_instructions);

                    zigurate_compiler_enki(&mut enki_vm)?;

                },
                Err(err) => {
                    return Err(err)
                }

            }

            
        }
    
        Commands::Create { project_name } => {

            let curr_path = sita.current_dir()
                .map_err(|e| ZigurateError::IOError(e))?; 

            let project_already_exists = sita.path_exists(&curr_path.join(project_name))
                .map_err(|e| ZigurateError::IOError(e))?;

            if !project_already_exists {

                let fld_project = sita.create_dir(project_name, &curr_path)
                    .map_err(|e| ZigurateError::IOError(e))?;

                let src_project = sita.create_dir("world", &fld_project)
                    .map_err(|e| ZigurateError::IOError(e))?;

                let fil_main = sita.create_file("main.zi", src_project)
                    .map_err(|e| ZigurateError::IOError(e))?;

                let _ = sita.edit_file(fil_main, "silim(\"Hello World!\")".to_string());

                let fil_enki = sita.create_file("Enki.yaml", &fld_project)
                    .map_err(|e| ZigurateError::IOError(e))?;

                let enki_content = format!(
                    "zigurate_ur: 1\nname: \"{name}\"\ndesc: \"\"\nstage: \"1\"\nmain: \"world/main.zi\"\n\nrules: []",
                    name = project_name
                );

                let _ = sita.edit_file(fil_enki, enki_content);

                let cmd = format!("enki create {}", project_name.yellow().italic());

                let cmd2 = format!("cd .\\{}\\", project_name);

                let msg_2 = format!("Project {} was created.", project_name.yellow().italic());
                let msg_3 = format!("Now, use {}", cmd2.yellow().italic());
                let msg_4 = format!("After, run {}", "enki run".yellow().italic());

                println!();
                println!("{}", version_zigurate);
                println!("--- {} {} {}", "cmd:".bold().yellow(), cmd.yellow().italic(), "!executed".bold().green());
                println!("--- {} {}", "in:".bold().yellow(), curr_path.display().to_string().underline());
                println!();
                println!("{} {}", "success:".bright_green().bold(), msg_2);
                
                println!();
                println!("{} {}", "tip:".bright_yellow().bold(), msg_3);
                println!("{} {}", "tip:".bright_yellow().bold(), msg_4);
                println!();

            } else {

                let cmd = format!("enki create {}", project_name);

                let msg_2 = format!("Already exists a project called {}", project_name.yellow().italic());
                
                println!();
                println!("{}", version_zigurate);
                println!("--- {} {} {}", "cmd:".bold().yellow(), cmd.yellow().italic(), "!not executed".bold().red());
                println!("--- {} {}", "in:".bold().yellow(), curr_path.display().to_string().underline());
                println!();
                println!("{} {}", "warning:".bright_yellow().bold(), msg_2);
                println!();

            }

            return Ok(())
        }

        Commands::Export { json } => {
            return Ok(())
        }

    }

    Ok(())
}