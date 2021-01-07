# Sliding Features
Modular sliding window with various signal processing functions and technical indicators including Normalization. Can be used for building low latency real-time trading systems. Values in window are updated at each time step. A View defines the function which processes the incoming values and provides and output value. Views can easily be added by implementing the View Trait which requires two functions:
- update(&mut self, val: f64)
- last(&self) -> f64

The Views can be parameterized as desired when created with new() function.
Add Views to SlidingWindow by calling register_view().
SlidingWindows last() function returns a vector of all the latest observations of all views that are currently registered. This vector can serve as input to a neural network or decision tree for creating trading agents.


### Views
A View defines the function which processes value updates. They currently include:
- Echo
- Normalizer
- Center of Gravity
- Cyber Cycle
- Laguerre RSI
- Laguerre Filter
- ReFlex
- TrendFlex
- ROC
- RSI
- ALMA (Arnaux Legoux Moving Average)
- Correlation Trend Indicator (CTI)
- Entropy (acts on a bit stream, thus does not impl View trait)

### How to use
```rust
extern crate rust_timeseries_generator;

use sliding_features::*;
use rust_timeseries_generator::gaussian_process::gen;

fn main() {
    // new sliding window
    let mut sf = SlidingWindow::new();

    let norm_len = 50;
    // register some normalized indicators so output range is [-1.0, 1.0] 
    sf.register_view(Box::new(Normalizer::new(Box::new(RSI::new(14)), norm_len)));
    sf.register_view(Box::new(Normalizer::new(Box::new(ROC::new(14)), norm_len)));
    sf.register_view(Box::new(Normalizer::new(Box::new(ReFlex::new(14)), norm_len)));
    // register some variance stabilized indicators
    sf.register_view(Box::new(VST::new(Box::new(TrendFlex::new(14)))));
    sf.register_view(Box::new(VST::new(Box::new(CenterOfGravity::new(14)))));
    sf.register_view(Box::new(VST::new(Box::new(CyberCycle::new(14)))));

    // generate dummy values
    let vals = gen(1024, 100.0);
    for i in 0..vals.len() {
        sf.update(vals[i]);
        let last: Vec<f64> = sf.last();  // get the latest values from sliding window
        println!("last: {:?}", last);
    }
}
```

See examples folder.
Run the examples using
```
cargo run --example simple
cargo run --example multiple
cargo run --example multiple_normalized
```

### Images
Underlying data synthetically generated by [MathisWellmann/rust_timeseries_generator](https://www.github.com/MathisWellmann/rust_timeseries_generator)
Note that each run is differently seeded by default.

![laguerre_filter](img/laguerre_filter.png)
![center_of_gravity](img/center_of_gravity.png)
![center_of_gravity_normalized](img/center_of_gravity_normalized.png)
![cyber_cycle](img/cyber_cycle.png)
![laguerre_rsi](img/laguerre_rsi.png)
![re_flex](img/re_flex.png)
![trend_flex](img/trend_flex.png)
![roc](img/roc.png)
![rsi](img/rsi.png)
![alma](img/alma.png)
![cti](img/plot_correlation_trend_indicator_cti.png)

### TODOs:
- SMA
- EMA
- FRAMA
- MAMA
- FAMA
- Stochastic
- Super Smoother
- Zero Lag
- gaussian filter

## License
Copyright (C) 2020  <MathisWellmann wellmannmathis@gmail.com>

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.

![GNU AGPLv3](agplv3.png)
