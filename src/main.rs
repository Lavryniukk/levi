use std::process::Command;

use args::LeviArgs;
use clap::Parser;
use variants::{ToGithubUrl, IntoArray, TemplateVariant};
use dialoguer::{theme::ColorfulTheme, Input, Select};


mod args;
mod variants;


fn main() {
    let args = LeviArgs::parse();
    match args.operation_type {
        args::OperationType::Init => {
            get_stack_choice()
        }
    }
}

fn clone_template(repo_url: &str, destination: &str) -> Result<(), std::io::Error> {
    Command::new("git")
        .args(["clone", repo_url, destination])
        .status()?;
    
    Ok(())
}

fn get_stack_choice() {
    let chosen_template = get_template_input();
    let destination =  get_destination_input();
    match chosen_template {
        TemplateVariant::ActSeaJun => {
            println!("You selected {}", TemplateVariant::ActSeaJun.to_string());
            clone_template(TemplateVariant::to_github_url(&TemplateVariant::ActSeaJun), &destination).unwrap();
        },
        TemplateVariant::ActSea => {            
            println!("You selected {}", TemplateVariant::ActSea.to_string());
            clone_template(TemplateVariant::to_github_url(&TemplateVariant::ActSea), &destination).unwrap();

        },
        TemplateVariant::AxuSea => {
            println!("You selected {}", TemplateVariant::AxuSea.to_string());
            clone_template(TemplateVariant::to_github_url(&TemplateVariant::AxuSea), &destination).unwrap();
        },
        TemplateVariant::AxuSeaJun => {
            println!("You selected {}", TemplateVariant::AxuSeaJun.to_string());
            clone_template(TemplateVariant::to_github_url(&TemplateVariant::AxuSeaJun), &destination).unwrap();
        }
    }
}


fn get_template_input() -> TemplateVariant {
    let options = TemplateVariant::into_array();
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick your option")
        .default(0)
        .items(&options)
        .interact()
        .unwrap();

    let selected_option = options[selection];
    TemplateVariant::from(selected_option)
}

fn get_destination_input() -> String{
    Input::with_theme(&ColorfulTheme::default())
    .with_prompt("Please enter the destination folder for your new project")
    .interact_text().unwrap()
}