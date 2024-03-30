use crate::internals::model::res_data::ResData;

use super::repository::ResDataRepository;

// Implement the MyTrait trait for a struct or type
pub struct ResDataRepoImpl;

impl ResDataRepoImpl {
    pub fn new() -> Self {
        ResDataRepoImpl {}
    }
}

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