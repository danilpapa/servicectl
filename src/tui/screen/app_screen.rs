#[derive(Clone)]
pub enum Screen {
    SelectServices,
    Actions(Vec<String>),
}
