use crate::chart::types::chart_type::ChartType;
use crate::gui::styles::donut::Catalog;
use crate::networking::types::byte_multiple::ByteMultiple;
use iced::{Color, Radians, Theme};
use std::f32::consts;

pub enum DonutKind {
    Total(ChartType, u128, u128, u128, u128),
    Ip,
    Proto,
}

impl DonutKind {
    pub fn get_title(&self) -> String {
        match self {
            Self::Total(chart_type, inc, out, filtered_out, dropped) => {
                if chart_type.eq(&ChartType::Bytes) {
                    ByteMultiple::formatted_string(inc + out + filtered_out + dropped)
                } else {
                    format!("{}", inc + out + filtered_out + dropped)
                }
            }
            Self::Ip => "IP".to_string(),
            Self::Proto => "Proto".to_string(),
        }
    }

    pub fn get_labels(&self) -> Vec<String> {
        match self {
            Self::Total(..) => vec!["Incoming".to_string(), "Outgoing".to_string()],
            Self::Ip => vec!["IP".to_string(), "Other".to_string()],
            Self::Proto => vec!["Proto".to_string(), "Other".to_string()],
        }
    }

    pub fn get_values(&self) -> Vec<u128> {
        match self {
            Self::Total(_, inc, out, filtered_out, dropped) => {
                println!("{}", dropped);
                vec![*inc, *out, *filtered_out, *dropped]
            }
            Self::Ip => vec![50, 250],
            Self::Proto => vec![75, 225],
        }
    }

    pub fn get_angles(&self) -> Vec<(Radians, Radians)> {
        let mut values = self.get_values();
        let total: u128 = values.iter().sum();
        let min_val = 5 * total / 100;
        let mut diff = 0;

        for value in values.iter_mut() {
            if *value != 0 && *value < min_val {
                diff += min_val - *value;
                *value = min_val;
            }
        }
        // remove the diff from the max value
        if diff > 0 {
            let _ = values.iter_mut().max().map(|max| *max -= diff);
        }

        let mut start_angle = Radians(-consts::FRAC_PI_2);
        let mut angles = Vec::new();
        for value in values {
            let end_angle = start_angle + Radians(consts::TAU) * (value as f32) / (total as f32);
            angles.push((start_angle, end_angle));
            start_angle = end_angle;
        }
        angles
    }
}
