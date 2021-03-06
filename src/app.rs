use eframe::egui::plot::*;
use eframe::{egui::*, epi};
use std::collections::HashMap;

#[derive(PartialEq, Copy, Clone)]
pub enum XAxis {
    ZScore,
    Value,
}

pub struct App {
    data_string: String,
    data_enter_err: bool,
    mean: f64,
    median: f64,
    variance: f64,
    standard_deviation: f64,
    is_sample: bool,
    x_axis: XAxis,
    plot: Plot,
}

impl App {
    pub fn new() -> Self {
        App {
            data_string: String::new(),
            data_enter_err: false,
            mean: 0.0,
            median: 0.0,
            variance: 0.0,
            standard_deviation: 0.0,
            is_sample: false,
            x_axis: XAxis::ZScore,
            plot: Plot::default(),
        }
    }

    fn process_input(&mut self) {
        match data_from_csv(&self.data_string) {
            Ok(d) => {
                self.data_enter_err = false;

                let (data, mean, median, variation, standard_deviation) =
                    plot_data(&d, self.is_sample, self.x_axis);

                self.data_enter_err = false;
                self.plot = Plot::default().curve(Curve::from_values(data));
                self.mean = mean;
                self.median = median;
                self.variance = variation;
                self.standard_deviation = standard_deviation;
            }
            Err(_) => self.data_enter_err = true,
        }
    }
}

impl epi::App for App {
    fn update(&mut self, ctx: &CtxRef, frame: &mut epi::Frame<'_>) {
        // Show data entry panel
        Window::new("Data Entry").show(ctx, |ui| {
            ui.collapsing("Click here to open text entry", |ui| {
                ScrollArea::from_max_height(512.0).show(ui, |ui| {
                    ui.text_edit_multiline(&mut self.data_string);
                });
            });
            if ui.checkbox(&mut self.is_sample, "Is sample data").changed() {
                self.process_input();
            }

            ui.horizontal(|ui| {
                ui.label("X Axis:");
                if ui
                    .radio_value(&mut self.x_axis, XAxis::ZScore, "By Z Score")
                    .changed()
                    || ui
                        .radio_value(&mut self.x_axis, XAxis::Value, "By Value")
                        .changed()
                {
                    self.process_input();
                }
            });

            if self.data_enter_err {
                ui.add(Label::new("Could not parse data").text_color(Color32::RED));
            }

            if ui.button("Enter").clicked() {
                self.process_input();
            }
        });

        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Information");
            ui.label(format!("Mean: {:.2}", self.mean));
            ui.label(format!("Median: {:.2}", self.median));
            ui.label(format!("Variance: {:.2}", self.variance));
            ui.label(format!(
                "Standard Deviation: {:.2}",
                self.standard_deviation
            ));
            ui.heading("Plot");
            ui.add(self.plot.clone());
        });
    }

    fn name(&self) -> &str {
        "Statistics Helper"
    }

    fn max_size_points(&self) -> Vec2 {
        Vec2::splat(f32::INFINITY)
    }
}

/// Plots the data, returning the points, mean, median, variance and standard deviation. If provided y axis is frequency, x is alwaus z-score
fn plot_data(data: &Vec<f64>, is_sample: bool, x_axis: XAxis) -> (Vec<Value>, f64, f64, f64, f64) {
    let mut owned = data.to_owned();
    owned.sort_by(|a, b| a.partial_cmp(b).unwrap());

    // Get the mean
    let mean = owned.iter().sum::<f64>() / (owned.len() as f64);

    // Get the median
    let mut median = 0.0;
    if owned.len() > 0 {
        if owned.len() % 2 == 0 {
            let i = owned.len() / 2;
            median = (owned[i] + owned[i - 1]) / 2.0;
        } else {
            median = owned[owned.len() / 2];
        }
    }

    // Get the variation and standard deviation
    let mut variation: f64 = owned.iter().map(|x| (x - mean).powi(2)).sum();

    if is_sample {
        variation /= (owned.len() - 1) as f64;
    } else {
        variation /= owned.len() as f64;
    }
    let standard_deviation = variation.sqrt();

    // Construct the final output points
    let mut output = Vec::with_capacity(data.len());
    let frequencies = get_frequencies(owned.as_slice());

    if x_axis == XAxis::ZScore {
        for (k, count) in frequencies {
            output.push(Value::new(
                z_score(f64::from_bits(k), mean, standard_deviation),
                count as f64,
            ));
        }
    } else {
        for (k, count) in frequencies {
            output.push(Value::new(f64::from_bits(k), count as f64));
        }
    }

    output.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());

    (output, mean, median, variation, standard_deviation)
}

fn z_score(value: f64, mean: f64, standard_deviation: f64) -> f64 {
    (value - mean) / standard_deviation
}

fn data_from_csv(data: &String) -> Result<Vec<f64>, std::num::ParseFloatError> {
    // Clean up
    let data = filter(&data);

    // Parse numbers
    let data = data.split(',');

    let mut output = Vec::new();
    for i in data {
        output.push(i.parse::<f64>()?);
    }

    Ok(output)
}

fn filter(s: &String) -> String {
    let mut output = String::new();

    for c in s.chars() {
        if !c.is_whitespace() && (c.is_digit(10) || c == ',' || c == '.' || c == '-') {
            output.push(c)
        }
    }

    if output.len() > 0 {
        let last = output.pop().unwrap();
        if last != ',' {
            output.push(last)
        }
    }

    output
}

/// Counts how many times a value exists in a vec, removing them as it goes
fn get_frequencies(d: &[f64]) -> HashMap<u64, usize> {
    let mut output: HashMap<u64, usize> = HashMap::new();

    for i in d {
        let bits = (*i).to_bits();
        let count = output.entry(bits).or_insert(0);
        *count += 1;
    }

    output
}
