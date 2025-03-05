use super::sch_models::PropertyWithDetails;

pub trait PropertiesRepository {
    fn fetch_top_properties(&self) -> Vec<PropertyWithDetails>;
}