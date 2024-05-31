use std::process::Command;


pub enum TemplateVariant {
    ActSeaJun,
    ActSea,
    // AxuSea,
    // AxuSeaJun
    Yew,
}


pub trait ToGithubUrl {
    fn to_github_url(&self) -> &'static str;
}

pub trait CloneRepo {
    fn clone_template_repo(variant: &TemplateVariant, destination: &str) -> ();

}

impl ToGithubUrl for TemplateVariant {
    fn to_github_url(&self) -> &'static str{
        match self {
            TemplateVariant::ActSeaJun => "https://github.com/lavryniukk/levi-actix-seaorm-juniper",
            TemplateVariant::ActSea => "https://github.com/lavryniukk/levi-actix-seaorm",
            TemplateVariant::Yew => "https://github.com/lavryniukk/levi-yew"
            // TemplateVariant::AxuSea => "https://github.com/lavryniukk/levi-axum-seaorm",
            // TemplateVariant::AxuSeaJun => "https://github.com/lavryniukk/levi-axum-seaorm-juniper"
        }
    }
}

fn clone_template(repo_url: &str, destination: &str) -> Result<(), std::io::Error> {
    Command::new("git")
        .args(["clone", repo_url, destination])
        .status()?;
    
    Ok(())
}

impl From<&str> for TemplateVariant {
    fn from(s: &str) -> Self {
        match s {
            "Actix-Web + SeaORM + Juniper" => TemplateVariant::ActSeaJun,
            "Actix-Web + SeaORM" => TemplateVariant::ActSea,
            "Yew" => TemplateVariant::Yew,
            // "Axum + SeaORM" => TemplateVariant::AxuSea,
            // "Axum + SeaORM + Juniper" => TemplateVariant::AxuSeaJun,
            _ => panic!("Invalid variant")
        }
    }
}


impl CloneRepo for TemplateVariant {
    fn clone_template_repo(variant: &TemplateVariant, destination: &str) -> () {
        match variant {
            TemplateVariant::ActSeaJun => {
                println!("You selected {}", TemplateVariant::ActSeaJun.to_string());
                clone_template(TemplateVariant::to_github_url(&TemplateVariant::ActSeaJun), &destination).unwrap();
            },
            TemplateVariant::ActSea => {            
                println!("You selected {}", TemplateVariant::ActSea.to_string());
                clone_template(TemplateVariant::to_github_url(&TemplateVariant::ActSea), &destination).unwrap();
    
            },
            TemplateVariant::Yew => {
                println!("You selected {}", TemplateVariant::Yew.to_string());
                clone_template(TemplateVariant::to_github_url(&TemplateVariant::Yew), &destination).unwrap();
            }
            // TemplateVariant::AxuSea => {
            //     println!("You selected {}", TemplateVariant::AxuSea.to_string());
            //     clone_template(TemplateVariant::to_github_url(&TemplateVariant::AxuSea), &destination).unwrap();
            // },
            // TemplateVariant::AxuSeaJun => {
            //     println!("You selected {}", TemplateVariant::AxuSeaJun.to_string());
            //     clone_template(TemplateVariant::to_github_url(&TemplateVariant::AxuSeaJun), &destination).unwrap();
            // }
        }
    }
}


impl ToString for TemplateVariant {
    fn to_string(&self) -> String {
        match self {
            TemplateVariant::ActSeaJun => "Actix-Web + SeaORM + Juniper".to_string(),
            TemplateVariant::ActSea => "Actix-Web + SeaORM".to_string(),
            TemplateVariant::Yew => "Yew".to_string(),
            // TemplateVariant::AxuSea => "Axum + SeaORM".to_string(),
            // TemplateVariant::AxuSeaJun => "Axum + SeaORM + Juniper".to_string()
        }
    }
}

pub trait IntoVec {
    fn into_vec() -> Vec<&'static str>;
}

impl IntoVec for TemplateVariant {
    fn into_vec() -> Vec<&'static str> {
        Vec::from([
            "Actix-Web + SeaORM + Juniper",
            "Actix-Web + SeaORM",
            "Axum + SeaORM",
            "Axum + SeaORM + Juniper" 
        ])      
    }
}