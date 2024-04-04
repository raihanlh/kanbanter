use async_trait::async_trait;

use crate::internals::model::res_data::ResData;

use super::repository::ResDataRepository;

pub struct ResDataRepoImpl;

impl ResDataRepoImpl {
    pub fn new() -> Self {
        ResDataRepoImpl {}
    }
}

#[async_trait]
impl ResDataRepository for ResDataRepoImpl {
    fn get_all_data(&self) -> Vec<ResData> {
        let res = vec![
            ResData {
                id: 1,
                content: "First content".to_string(),
            },
            ResData {
                id: 2,
                content: "Second content".to_string(),
            },
        ];

        res
    }
}
