use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Input, Select};

use crate::{args, variants::{CloneRepo, IntoVec, TemplateVariant}};

pub fn get_stack_choice_and_clone_repo() {
    let chosen_template = get_template_input();
    let destination =  get_destination_input();
    TemplateVariant::clone_template_repo(&chosen_template, &destination)
}


pub fn get_template_input() -> TemplateVariant {
    let options = TemplateVariant::into_vec();
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick your option")
        .default(0)
        .items(&options)
        .interact()
        .unwrap();

    let selected_option = options[selection];
    TemplateVariant::from(selected_option)
}

pub fn get_destination_input() -> String{
    Input::with_theme(&ColorfulTheme::default())
    .with_prompt("Please enter the destination folder for your new project")
    .interact_text().unwrap()
}

pub fn handle_user_input() -> () {
    let args = args::LeviArgs::parse();
    match args.operation_type {
        args::OperationType::Init => {
            get_stack_choice_and_clone_repo()
        }
    }
}