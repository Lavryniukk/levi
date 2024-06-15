use std::process::Command;


pub enum TemplateVariant {
    ActSeaJun,
    ActSea,
    // AxuSea,
    // AxuSeaJun
}


pub trait ToGithubUrl {
    fn to_github_url(&self) -> &'static str;
}

pub trait CloneRepo {
    fn clone_template_repo(variant: &TemplateVariant, destination: &str) -> Result<(), std::io::Error>;

}

impl ToGithubUrl for TemplateVariant {
    fn to_github_url(&self) -> &'static str{
        match self {
            TemplateVariant::ActSeaJun => "https://github.com/lavryniukk/levi-actix-seaorm-juniper",
            TemplateVariant::ActSea => "https://github.com/lavryniukk/levi-actix-seaorm",
            // TemplateVariant::AxuSea => "https://github.com/lavryniukk/levi-axum-seaorm",
            // TemplateVariant::AxuSeaJun => "https://github.com/lavryniukk/levi-axum-seaorm-juniper"
        }
    }
}


impl From<&str> for TemplateVariant {
    fn from(s: &str) -> Self {
        match s {
            "Actix-Web + SeaORM + Juniper" => TemplateVariant::ActSeaJun,
            "Actix-Web + SeaORM" => TemplateVariant::ActSea,
            // "Axum + SeaORM" => TemplateVariant::AxuSea,
            // "Axum + SeaORM + Juniper" => TemplateVariant::AxuSeaJun,
            _ => panic!("Invalid variant")
        }
    }
}


impl CloneRepo for TemplateVariant {
    fn clone_template_repo(variant: &TemplateVariant, destination: &str) -> Result<(), std::io::Error> {
        Command::new("git")
        .args(["clone", TemplateVariant::to_github_url(variant), destination])
        .status()?;
    
    Ok(())
    }
}


impl ToString for TemplateVariant {
    fn to_string(&self) -> String {
        match self {
            TemplateVariant::ActSeaJun => "Actix-Web + SeaORM + Juniper".to_string(),
            TemplateVariant::ActSea => "Actix-Web + SeaORM".to_string(),
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