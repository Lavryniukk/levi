
pub enum TemplateVariants {
    ActSeaJun,
    ActSea,
    AxuSea,
    AxuSeaJun
}

pub trait IntoArray {
    fn into_array() -> Vec<&'static str>;
}


impl From<&str> for TemplateVariants {
    fn from(s: &str) -> Self {
        match s {
            "Actix-Web + SeaORM + Juniper" => TemplateVariants::ActSeaJun,
            "Actix-Web + SeaORM" => TemplateVariants::ActSea,
            "Axum + SeaORM" => TemplateVariants::AxuSea,
            "Axum + SeaORM + Juniper" => TemplateVariants::AxuSeaJun,
            _ => panic!("Invalid variant")
        }
    }
}

impl ToString for TemplateVariants {
    fn to_string(&self) -> String {
        match self {
            TemplateVariants::ActSeaJun => "Actix-Web + SeaORM + Juniper".to_string(),
            TemplateVariants::ActSea => "Actix-Web + SeaORM".to_string(),
            TemplateVariants::AxuSea => "Axum + SeaORM".to_string(),
            TemplateVariants::AxuSeaJun => "Axum + SeaORM + Juniper".to_string()
        }
    }
}

impl IntoArray for TemplateVariants {
    fn into_array() -> Vec<&'static str> {
        Vec::from([
            "Actix-Web + SeaORM + Juniper",
            "Actix-Web + SeaORM",
            "Axum + SeaORM",
            "Axum + SeaORM + Juniper" 
        ])      
    }
}