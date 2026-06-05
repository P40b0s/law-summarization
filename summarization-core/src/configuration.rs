use std::{path::{Path, PathBuf}, sync::Arc};
use anyhow::Context;
use tracing::info;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CoreConfiguration
{
    #[serde(default = "ai_service_url")]
    pub ai_service_url: String,
    #[serde(default = "document_types")]
    pub document_types: Vec<String>,
    #[serde(default = "document_signatories")]
    pub document_signatories: Vec<String>,
    #[serde(default = "check_period_min")]
    pub check_period_min: usize,
}

pub fn document_types() -> Vec<String>
{
    vec![
            "e0c56da5-17f1-40ac-a10f-8a9a1cee0c4e".to_string(),
            "93273da3-3133-4acf-96c2-4adc1ae70e19".to_string(),
            "82a8bf1c-3bc7-47ed-827f-7affd43a7f27".to_string(),
            "0790e34b-784b-4372-884e-3282622a24bd".to_string(),
            "fd5a8766-f6fd-4ac2-8fd9-66f414d314ac".to_string(),
            "7ff5b3b5-3757-44f1-bb76-3766cabe3593".to_string(),


        ]
}
pub fn document_signatories() -> Vec<String>
{
    vec![
            //"Президент Российской Федерации"
            "225698f1-cfbc-4e42-9caa-32f9f7403211".to_string(),
            //"Верховный Главнокомандующий Вооруженными силами Российской Федерации"
            "1049e10d-0133-4ef6-95ae-a487c0e7f653".to_string(),
            //"Совет Федерации Федерального Собрания Российской Федерации"
            "730e580c-c6ad-4aca-be1a-b49f7e4694fe".to_string(),
            //"Государственная Дума Федерального Собрания Российской Федерации"
            "1e57a3e5-9122-41e5-b3cf-0ab68ed3601a".to_string(),
            //"Правительство Российской Федерации"
            "8005d8c9-4b6d-48d3-861a-2a37e69fccb3".to_string(),
            //"Конституционный Суд Российской Федерации"
            "72b9c96e-9091-4b4e-b5eb-113a8432d2cd".to_string(),
            //"Российская Федерация"
            "8fc90ffa-4caa-45ac-a2b2-41fc5df98dc5".to_string(),
        ]
}
pub fn check_period_min() -> usize
{
    5
}
pub fn ai_service_url() -> String
{
    "http://10.0.0.2:8080".to_string()
}


impl Default for CoreConfiguration
{
    fn default() -> Self 
    {
        Self 
        {
            ai_service_url: ai_service_url(),
            document_types: document_types(),
            document_signatories: document_signatories(),
            check_period_min: check_period_min(),
        }
    }
}