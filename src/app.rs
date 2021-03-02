use eframe::{egui::*, epi};
use eframe::egui::plot::*;

pub struct App{
    data_string: String,
    data_enter_err: bool,
    data: Vec<f64>,
    plot: Plot
}

impl App{
    pub fn new() -> Self{
        App{
            data_string: String::new(),
            data_enter_err: false,
            data: Vec::new(),
            plot: Plot::default()
        }
    }
}

impl epi::App for App{
    fn name(&self) -> &str{
        "Statistics Helper"
    }

    fn update(&mut self, ctx: &CtxRef, frame: &mut epi::Frame<'_>){
        // Show data entry panel
        Window::new("Data Entry").show(ctx, |ui| {
            ui.text_edit_multiline(&mut self.data_string);

            if self.data_enter_err{
                ui.add(Label::new("Could not parse data").text_color(Color32::RED));
            }

            if ui.button("Enter").clicked(){
                match data_from_csv(&self.data_string){
                    Ok(d) => {
                        self.data = d;
                        self.data_enter_err = false;
                        self.plot = Plot::default()
                        .curve(Curve::from_values(plot_data(&self.data)));
                    },
                    Err(_) => self.data_enter_err = true
                }
            }
        });

        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Plot");
            ui.add(self.plot.clone());
            ui.heading("Information")
        });
    }
}

fn plot_data(data: &Vec<f64>) -> Vec<Value>{
    let mut output = Vec::with_capacity(data.len());

    for (p, v) in data.iter().enumerate(){
        output.push(Value::new(p as f64, *v))
    }

    output
}

fn data_from_csv(data: &String) -> Result<Vec<f64>, std::num::ParseFloatError>{
    // Clean up
    let data  = filter(&data);
    
    // Parse numbers
    let data = data.split(',');

    let mut output = Vec::new();
    for i in data{
        output.push(i.parse::<f64>()?);
    }

    Ok(output)
}

fn filter(s: &String) -> String{
    let mut output = String::new();
    
    for c in s.chars(){
        if !c.is_whitespace() && (c.is_digit(10) || c == ','){
            output.push(c)
        }
    }

    if output.len() > 0{
        let last = output.pop().unwrap();
        if last != ','{
            output.push(last)
        }
    }

    output
}