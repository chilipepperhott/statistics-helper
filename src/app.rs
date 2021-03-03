use eframe::egui::plot::*;
use eframe::{egui::*, epi};

pub struct App {
    data_string: String,
    data_enter_err: bool,
    mean: f64,
    median: f64,
    plot: Plot,
}

impl App {
    pub fn new() -> Self {
        App {
            data_string: String::new(),
            data_enter_err: false,
            mean: 0.0,
            median: 0.0,
            plot: Plot::default(),
        }
    }
}

impl epi::App for App {
    fn update(&mut self, ctx: &CtxRef, frame: &mut epi::Frame<'_>) {
        // Show data entry panel
        Window::new("Data Entry").show(ctx, |ui| {
            ui.text_edit_multiline(&mut self.data_string);
            ui.label(&self.data_string);

            if self.data_enter_err {
                ui.add(Label::new("Could not parse data").text_color(Color32::RED));
            }

            if ui.button("Enter").clicked() {
                match data_from_csv(&self.data_string) {
                    Ok(d) => {
                        self.data_enter_err = false;

                        let (data, mean, median) = plot_data(&d);
                        println!(
                            "Plotting {}. Mean: {}. Median: {}.",
                            self.data_string, mean, median
                        );

                        self.data_enter_err = false;
                        self.plot = Plot::default().curve(Curve::from_values(data));
                        self.mean = mean;
                        self.median = median;
                    }
                    Err(_) => self.data_enter_err = true,
                }
            }
        });

        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Information");
            ui.label(format!("Mean: {}", self.mean));
            ui.label(format!("Median: {}", self.median));
            ui.heading("Plot");
            ui.add(self.plot.clone());
        });
    }

    fn name(&self) -> &str {
        "Statistics Helper"
    }
}

/// Plots the data, returning the points, mean, median, and standard deviation
fn plot_data(data: &Vec<f64>) -> (Vec<Value>, f64, f64) {
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

    // Construct the final output
    let mut output = Vec::with_capacity(data.len());

    for (p, v) in data.iter().enumerate() {
        output.push(Value::new(p as f64, *v))
    }

    (output, mean, median)
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
        if !c.is_whitespace() && (c.is_digit(10) || c == ',' || c == '.') {
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
