use std::collections::VecDeque;

use super::sliding_window::View;
use crate::Echo;

/// John Ehlers LaguerreRSI
/// from: http://mesasoftware.com/papers/TimeWarp.pdf
#[derive(Clone)]
pub struct LaguerreRSI {
    view: Box<dyn View>,
    value: f64,
    gamma: f64,
    l0s: VecDeque<f64>,
    l1s: VecDeque<f64>,
    l2s: VecDeque<f64>,
    l3s: VecDeque<f64>,
}

impl LaguerreRSI {
    /// Create a new LaguerreRSI with a chained View
    /// and a given sliding window length
    pub fn new(view: Box<dyn View>, window_len: usize) -> Box<Self> {
        Box::new(LaguerreRSI {
            view,
            value: 0.0,
            gamma: 2.0 / (window_len as f64 + 1.0),
            l0s: VecDeque::new(),
            l1s: VecDeque::new(),
            l2s: VecDeque::new(),
            l3s: VecDeque::new(),
        })
    }

    /// Create a new LaguerreRSI with a given window length
    pub fn new_final(window_len: usize) -> Box<Self> {
        Self::new(Echo::new(), window_len)
    }
}

impl View for LaguerreRSI {
    fn update(&mut self, val: f64) {
        self.view.update(val);
        let val = self.view.last();

        if self.l0s.len() >= 3 {
            self.l0s.pop_front();
            self.l1s.pop_front();
            self.l2s.pop_front();
            self.l3s.pop_front();
        }

        if self.l0s.len() < 2 {
            self.l0s.push_back(0.0);
            self.l1s.push_back(0.0);
            self.l2s.push_back(0.0);
            self.l3s.push_back(0.0);
            return;
        } else {
            let last = self.l0s.len() - 1;
            self.l0s
                .push_back((1.0 - self.gamma) * val + self.gamma * self.l0s.get(last - 1).unwrap());
            self.l1s.push_back(
                -self.gamma * self.l0s.get(last).unwrap()
                    + self.l0s.get(last - 1).unwrap()
                    + self.gamma * self.l1s.get(last - 1).unwrap(),
            );
            self.l2s.push_back(
                -self.gamma * self.l1s.get(last).unwrap()
                    + self.l1s.get(last - 1).unwrap()
                    + self.gamma * self.l2s.get(last - 1).unwrap(),
            );
            self.l3s.push_back(
                -self.gamma * self.l2s.get(last).unwrap()
                    + self.l2s.get(last - 1).unwrap()
                    + self.gamma * self.l3s.get(last - 1).unwrap(),
            );
        }
        let last = self.l0s.len() - 1;

        let mut cu: f64 = 0.0;
        let mut cd: f64 = 0.0;
        if self.l0s.get(last) >= self.l1s.get(last) {
            cu = self.l0s.get(last).unwrap() - self.l1s.get(last).unwrap();
        } else {
            cd = self.l1s.get(last).unwrap() - self.l0s.get(last).unwrap();
        }
        if self.l1s.get(last) >= self.l2s.get(last) {
            cu += self.l1s.get(last).unwrap() - self.l2s.get(last).unwrap();
        } else {
            cd += self.l2s.get(last).unwrap() - self.l1s.get(last).unwrap();
        }
        if self.l2s.get(last) >= self.l3s.get(last) {
            cu += self.l2s.get(last).unwrap() - self.l3s.get(last).unwrap();
        } else {
            cd += self.l3s.get(last).unwrap() - self.l2s.get(last).unwrap();
        }

        if cu + cd != 0.0 {
            self.value = cu / (cu + cd);
        }
    }
    fn last(&self) -> f64 {
        return self.value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plot::plot_values;
    use crate::test_data::TEST_DATA;

    #[test]
    fn laguerre_rsi() {
        let mut lrsi = LaguerreRSI::new_final(16);
        for v in &TEST_DATA {
            lrsi.update(*v);
            let last = lrsi.last();
            assert!(last <= 1.0);
            assert!(last >= -1.0);
        }
    }

    #[test]
    fn laguerre_rsi_plot() {
        let mut lrsi = LaguerreRSI::new_final(16);
        let mut out: Vec<f64> = Vec::new();
        for v in &TEST_DATA {
            lrsi.update(*v);
            out.push(lrsi.last());
        }
        // graph the results
        let filename = "img/laguerre_rsi.png";
        plot_values(out, filename).unwrap();
    }
}
