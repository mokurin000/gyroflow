use super::*;

use crate::gyro_source::TimeQuat;

#[derive(Clone)]
pub struct Plain { pub time_constant: f64 }

impl Default for Plain {
    fn default() -> Self { Self { time_constant: 0.2 } }
}

impl SmoothingAlgorithm for Plain {
    fn get_name(&self) -> String { "Plain 3D smoothing".to_owned() }

    fn set_parameter(&mut self, name: &str, val: f64) {
        match name {
            "time_constant" => self.time_constant = val,
            _ => log::error!("Invalid parameter name: {}", name)
        }
    }
    fn get_parameters_json(&self) -> simd_json::owned::Value {
        simd_json::json!([
            {
                "name": "time_constant",
                "description": "Smoothness",
                "type": "SliderWithField",
                "from": 0.01,
                "to": 1.0,
                "value": 0.25,
                "unit": "s"
            }
        ])
    }

    fn get_checksum(&self) -> u64 {
        self.time_constant.to_bits()
    }

    fn smooth(&self, quats: &TimeQuat, duration: f64) -> TimeQuat { // TODO Result<>?
        if quats.is_empty() || duration <= 0.0 { return quats.clone(); }

        let sample_rate: f64 = quats.len() as f64 / (duration / 1000.0);

        let mut alpha = 1.0;
        if self.time_constant > 0.0 {
            alpha = 1.0 - (-(1.0 / sample_rate) / self.time_constant).exp();
        }
        
        let mut q = *quats.iter().next().unwrap().1;
        let smoothed1: TimeQuat = quats.iter().map(|x| {
            q = q.slerp(x.1, alpha);
            (*x.0, q)
        }).collect();

        // Reverse pass
        let mut q = *smoothed1.iter().next_back().unwrap().1;
        smoothed1.iter().rev().map(|x| {
            q = q.slerp(x.1, alpha);
            (*x.0, q)
        }).collect()
        // No need to reverse the BTreeMap, because it's sorted by definition
    }
}
