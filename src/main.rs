use args::LeviArgs;
use clap::Parser;
use variants::{TemplateVariants, IntoArray};
use dialoguer::{theme::ColorfulTheme, Select};


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

fn get_stack_choice() {
    let options = TemplateVariants::into_array();
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick your option")
        .default(0)
        .items(&options)
        .interact()
        .unwrap();

    let selected_option = options[selection];
    let option = TemplateVariants::from(selected_option);
    match option {
        TemplateVariants::ActSeaJun => {
            println!("You selected {}", TemplateVariants::ActSeaJun.to_string())
        },
        TemplateVariants::ActSea => {
            println!("You selected {}", TemplateVariants::ActSea.to_string())
        },
        TemplateVariants::AxuSea => {
            println!("You selected {}", TemplateVariants::AxuSea.to_string())
        },
        TemplateVariants::AxuSeaJun => {
            println!("You selected {}", TemplateVariants::AxuSeaJun.to_string())
        }
    }
}