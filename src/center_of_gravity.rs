use std::collections::VecDeque;

use super::sliding_window::View;
use crate::Echo;

/// John Ehlers Center of Gravity Indicator
/// from: https://mesasoftware.com/papers/TheCGOscillator.pdf
#[derive(Clone)]
pub struct CenterOfGravity {
    view: Box<dyn View>,
    window_len: usize,
    q_vals: VecDeque<f64>,
    out: f64,
}

impl CenterOfGravity {
    /// Create a Center of Gravity Indicator with a chained View
    /// and a given sliding window length
    pub fn new(view: Box<dyn View>, window_len: usize) -> Box<Self> {
        Box::new(CenterOfGravity {
            view,
            window_len,
            q_vals: VecDeque::new(),
            out: 0.0,
        })
    }

    /// Create a Center of Gravity Indicator with a given window length
    pub fn new_final(window_len: usize) -> Box<Self> {
        Self::new(Echo::new(), window_len)
    }
}

impl View for CenterOfGravity {
    // update receives a new value and updates its internal state
    fn update(&mut self, val: f64) {
        self.view.update(val);
        let val = self.view.last();

        if self.q_vals.len() >= self.window_len {
            self.q_vals.pop_front();
        }
        self.q_vals.push_back(val);

        let mut denom: f64 = 0.0;
        let mut num: f64 = 0.0;
        let q_len = self.q_vals.len();
        for i in 0..q_len {
            let weight = q_len - i;
            let val_i = self.q_vals.get(i).unwrap();
            num += weight as f64 * val_i;
            denom += *val_i;
        }
        if denom != 0.0 {
            self.out = -num / denom + (self.q_vals.len() as f64 + 1.0) / 2.0
        } else {
            self.out = 0.0;
        }
    }
    fn last(&self) -> f64 {
        return self.out;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plot::plot_values;
    use crate::test_data::TEST_DATA;

    #[test]
    fn center_of_gravity_plot() {
        let mut cgo = CenterOfGravity::new_final(16);
        let mut out: Vec<f64> = Vec::new();
        for v in &TEST_DATA {
            cgo.update(*v);
            out.push(cgo.last());
        }
        let filename = "img/center_of_gravity.png";
        plot_values(out, filename).unwrap();
    }
}
