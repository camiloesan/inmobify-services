use super::sch_models::Property;

pub trait PropertiesRepository {
    fn fetch_top_properties(&self) -> Vec<Property>;
}