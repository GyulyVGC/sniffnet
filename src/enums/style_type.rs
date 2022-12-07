/// Used to specify the kind of style of the application
#[derive(Copy, Eq, PartialEq)]
pub enum StyleType {
    Night,
    Day,
}

impl Clone for StyleType {
    fn clone(&self) -> Self {
        *self
    }
}
