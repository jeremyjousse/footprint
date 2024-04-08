use serde::Deserialize;
#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListingSort {
    pub by: String,
    pub order: String, // TODO SortOrder
}

impl ListingSort {
    pub fn default(by: String) -> ListingSort {
        ListingSort {
            by,
            order: "Asc".to_string(), // TODO SortOrder
        }
    }
}

#[derive(Deserialize)]
pub enum SortOrder {
    Asc,
    Desc,
}
