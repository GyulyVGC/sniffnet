use iced::widget::Tooltip;
use iced::Renderer;

use crate::gui::types::message::Message;
use crate::networking::types::address_port_pair::AddressPortPair;
use crate::networking::types::info_address_port_pair::InfoAddressPortPair;
use crate::StyleType;

/// Struct embedding all the info needed to build a row of the connections report
pub struct ReportEntry {
    pub key: AddressPortPair,
    pub val: InfoAddressPortPair,
    pub tooltip: Tooltip<'static, Message, Renderer<StyleType>>,
}
