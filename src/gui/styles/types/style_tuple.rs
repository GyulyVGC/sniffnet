use std::sync::Arc;

use crate::gui::styles::types::element_type::ElementType;
use crate::StyleType;

/// This tuple permits to specify the correct style depending on the style type and on the element type
pub struct StyleTuple(pub Arc<StyleType>, pub ElementType);

impl Clone for StyleTuple {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0), self.1)
    }
}
