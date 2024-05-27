
pub enum TemplateVariant {
    ActSeaJun,
    ActSea,
    AxuSea,
    AxuSeaJun
}

pub trait IntoArray {
    fn into_array() -> Vec<&'static str>;
}

pub trait ToGithubUrl {
    fn to_github_url(&self) -> &'static str;
}

impl ToGithubUrl for TemplateVariant {
    fn to_github_url(&self) -> &'static str{
        match self {
            TemplateVariant::ActSeaJun => "https://github.com/lavryniukk/levi-actix-seaorm-juniper",
            TemplateVariant::ActSea => "https://github.com/lavryniukk/levi-actix-seaorm",
            TemplateVariant::AxuSea => "https://github.com/lavryniukk/levi-axum-seaorm",
            TemplateVariant::AxuSeaJun => "https://github.com/lavryniukk/levi-axum-seaorm-juniper"
        }
    }
}

impl From<&str> for TemplateVariant {
    fn from(s: &str) -> Self {
        match s {
            "Actix-Web + SeaORM + Juniper" => TemplateVariant::ActSeaJun,
            "Actix-Web + SeaORM" => TemplateVariant::ActSea,
            "Axum + SeaORM" => TemplateVariant::AxuSea,
            "Axum + SeaORM + Juniper" => TemplateVariant::AxuSeaJun,
            _ => panic!("Invalid variant")
        }
    }
}


impl ToString for TemplateVariant {
    fn to_string(&self) -> String {
        match self {
            TemplateVariant::ActSeaJun => "Actix-Web + SeaORM + Juniper".to_string(),
            TemplateVariant::ActSea => "Actix-Web + SeaORM".to_string(),
            TemplateVariant::AxuSea => "Axum + SeaORM".to_string(),
            TemplateVariant::AxuSeaJun => "Axum + SeaORM + Juniper".to_string()
        }
    }
}

impl IntoArray for TemplateVariant {
    fn into_array() -> Vec<&'static str> {
        Vec::from([
            "Actix-Web + SeaORM + Juniper",
            "Actix-Web + SeaORM",
            "Axum + SeaORM",
            "Axum + SeaORM + Juniper" 
        ])      
    }
}