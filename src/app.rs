use egui::plot::Bar;
use egui::plot::PlotUi;
use egui::plot::{BarChart, Corner, Legend, Line, Plot};
use egui::Color32;
use egui::RichText;
use ordered_float::OrderedFloat;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
#[derive(Default)]
pub struct PlotSettings {
    n: usize,
    seg_mult: usize,
    border: f64,
    filter_extreme: bool,
    distance_to_median: f64,
    bar_chart_width: f64,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
#[derive(Default)]
pub struct PlotDataAndSettings {
    name: String,
    data: String,
    settings: Option<PlotSettings>,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
#[derive(Default)]
pub struct CalcedData {
    name: String,
    data_size_before_filters: usize,
    data_parsed: Vec<f64>,
    polynom: Vec<f64>,
    minv: f64,
    maxv: f64,
    median: f64,
    avg: f64,
    values: Vec<[f64; 2]>,

    #[serde(skip)]
    bar_chart: Vec<Bar>,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct App {
    aspect_ratio: f64,

    global_settings: PlotSettings,

    values: Vec<(PlotDataAndSettings, Result<CalcedData, String>)>,
}

const DATA1: &str = "13.9 12.9 11.81 12.14 11.67 11.57 10.69 26.75 14.59 11.61 12.3 11.84 11.97 13.64 11.97 12.47 11.5 11.09 13.37 13.94 11.84 12.97 13.01 10.82 12.07 10.74 11.34 19.5 11.57 11.8 11.87 12.35 11.84 11.47 15.31 16.22 13.9 13.84 14.4 10.84 11.14 10.2 14.7 9.97 11.57 26.3 13.17 11.8 13.3 11.7 12.32 16.94 15.57 11.31 12.27 14.87 12.6 15.41 21.02 11.17 10.5 11.64 12 12.51 11 12.87 11.14 10.41 11.21 10.77 13.77 10.3 13.67 28.97 11.54 11.7 10.9 12.67 10.87 11.17 11.17 26.24 13.4 11.54 12.04 10.64 10.07 17.07 12.21 13.1 10.67 10.87 18.14 11.9 14.64 10.49 19.27 15.52 12.57 14.37 14.17 11.81 14.6 12.77 12.21 17.61 11.1 14.66 13.3 10.5 19.94 15.51 11.34 12.37 12.07 14.04 11.44 11.64 10.64 12.87 10.8 11.9 13.4 10.91 11.04 14.17 13.2 12 14.2 12.94 11.57 13.45 11.16 10.3 11.27 12.97 11.04 13.7 10.11 15.07 11.77 12.84 12.17 10.87 11.04 13.01 14.2 12.81 10.37 12.7 10.69 14.44 14.4 11.81 12.6 11.3 11.74 10.7 10.5 14.27 13.7 10.74 12.24 18.04 19.45 11.77 14.74 12.79 12.44 14.2 11.74 12.74 19.6 13.8 12.31 11.44 11.37 11.51 11.94 11.74 13.69 11.44 10.07 16.07 25.14 12.01 13.54 13.01 14.11 12.44 9.54 11.87 14.71 11.37 13.47 11.34 13.01 9.27 18.74 10.67 11.44 29.34 11.17 10.67 16.57 17.97 17.57 10.74 11.01 13.81 10.57 12.71 14.01 10.51 9.74 10.94 11.47 10.91 10.64 10.64 12.47 10.37 12.01 9.41 10.04 16.37 23.61 12.34 10.97 14.64 11.87 11.01 10.04 12.94 10.54 10.37 14.54 10.24 10.04 12.31 13.77 11.87 12.47 9.27 11.74 13.94 13.44 11.51 11.01 11.17 9.74 10.64 10.21 11.34 11.11 10.41 11.51 12.37 8.97 13.04 14.44 15.44 13.84 11.27 15.67 12.07 51.97 25.01 15.34 18.11 11.57 22.11 14.67 11.84 10.41 15.44 21.74 10.77 12.44 10.94 10.21 13.77 22.97 13.64 11.74 11.74 11.01 11.11 10.47 10.57 11.39 10.81 9.54 13.54 22.77 11.44 11.74 9.94 12.17 9.14 15.97 10.77 11.24 11.22 12.34 12.04 14.61 11.51 15.47";

const DATA2: &str = "11.77 12.81 13.01 11.34 11.81 11.24 13.34 15.64 19.24 12.21 10.87 13.91 15.24 10.51 12.94 14.34 14.57 49.77 12.81 15.17 12.21 12.77 11.34 10.51 11.61 11.57 10.74 12.31 12.77 12.01 11.41 11.97 11.44 11.24 11.17 9.64 16.44 12.87 12.91 10.54 10.37 27.34 20.17 11.21 12.41 11.87 14.44 15.04 20.54 12.24 17.11 11.31 13.14 13.04 12.31 11.61 10.24 11.01 10.91 10.34 10.74 11.07 15.61 10.54 10.77 9.14 11.97 14.04 34.84 11.41 10.47 9.97 11.34 10.24 19.11 18.04 12.61 10.77 15.21 10.11 10.77 10.17 12.11 19.74 10.64 13.14 12.81 9.97 10.97 11.31 10.27 13.24 11.27 10.04 10.44 9.47 10.07 9.74 10.27 9.71 9.87 10.47 10.31 11.64 11.14 12.94 9.74 10.37 10.97 11.87 13.01 10.57 9.94 11.81 9.41 12.47 15.21 13.21 9.57 11.87 15.04 12.81 9.99 10.84 9.74 11.97 10.51 10.94 9.91 11.71 10.47 10.87 11.31 12.67 17.67 11.41 18.71 11.67 10.44 10.74 10.34 10.81 16.01 10.27 10.01 10.27 13.57 9.77 9.4 9.57 8.47 10.27 11.37 16.04 16.57 16.94 13.41 11.14 10.71 13.57 10.61 10.34 10.84 63.41 12.51 59.47 26.14 13.27 10.67 16.04 14.84 12.54 10.74 12.44 11.91 16.51 10.67 10.07 11.84 11.01 9.61 9.81 9.44 9.71 10.01 9.11 14.07 9.74 43.74 10.27 8.81 12.54 12.24 17.57 11.07 24.24 10.84 18.91 10.44 11.91 9.64 12.71 13.84 13.41";

const GRAPH_STEP_COUNT: usize = 1000;

fn sort_f64(vec: &mut [f64]) {
    vec.sort_by_key(|x| OrderedFloat::try_from(*x).unwrap());
}

fn approximate_distribution_by_polynom(
    sorted_values: &[f64],
    n: usize,
    seg_mult: usize,
    border: f64,
) -> Vec<f64> {
    use mathru::algebra::linear::matrix::Solve;
    use mathru::algebra::linear::Matrix;
    use mathru::algebra::linear::Vector;

    let mut mat = Matrix::new(n + 1, n + 1, vec![0f64; (n + 1) * (n + 1)]);
    let mut vec = Vector::new_column(vec![0f64; n + 1]);
    let mut t = vec![0f64; n + 1];
    let minv = sorted_values[0] - border;
    let maxv = sorted_values.last().unwrap() + border;
    let seg_count = ((sorted_values.len() * seg_mult) as f64).sqrt() as i64;
    let step = (maxv - minv) / seg_count as f64;

    let mut apos = 0;
    for ai in 0..=seg_count + 2 {
        let a = minv - step + ai as f64 * step;

        while apos < sorted_values.len() && a > sorted_values[apos] {
            apos += 1;
        }

        let mut bpos = apos;
        for bi in ai + 1..=seg_count + 2 {
            let b = minv - step + bi as f64 * step;

            while bpos < sorted_values.len() && b > sorted_values[bpos] {
                bpos += 1;
            }

            let count = bpos - apos;
            let count_double = (count as f64) / sorted_values.len() as f64;

            // I checked, it holds
            // assert_eq!(count, sorted_values.iter().filter(|x| a <= **x && **x < b).count());

            (0..n + 1)
                .map(|i| (i + 1) as f64)
                .map(|i| (b.powf(i) - a.powf(i)) / i)
                .enumerate()
                .for_each(|(pos, value)| t[pos] = value);

            for i in 0..t.len() {
                vec[i] += 2. * count_double * t[i];
            }

            for i in 0..t.len() {
                for j in 0..t.len() {
                    mat[[i, j]] += t[i] * t[j];

                    // write second sum (even for i == j, it's derivative)
                    mat[[j, i]] += t[i] * t[j];
                }
            }
        }
    }

    mat.solve(&vec).unwrap().iter().copied().collect::<Vec<_>>()
}

fn print_polynom(x: &[f64], for_desmos: bool) -> String {
    use std::fmt::Write;

    let mut out = String::new();

    for (i, v) in x.iter().enumerate() {
        if i == 0 {
            write!(out, "{v}").unwrap();
        } else if i == 1 {
            write!(out, "{v}*x").unwrap();
        } else if for_desmos {
            write!(out, "{v}*x^{{{i}}}").unwrap();
        } else {
            write!(out, "{v}*x^{i}").unwrap();
        }
        if i != x.len() - 1 && x[i + 1] > 0. {
            write!(out, "+").unwrap();
        }
    }

    out
}

fn calc_polynom_value(x: f64, polynom: &[f64]) -> f64 {
    let mut sum = 0.;
    let mut mul = 1.;
    for i in polynom {
        sum += mul * i;
        mul *= x;
    }
    sum
}

fn median(numbers: &[f64]) -> f64 {
    let mid = numbers.len() / 2;
    numbers[mid]
}

fn filter_times(times: &[f64], distance_to_median: f64) -> Vec<f64> {
    let median_time = median(times);
    times
        .iter()
        .filter(|x| (**x - median_time).abs() < distance_to_median)
        .copied()
        .collect::<Vec<_>>()
}

impl Default for App {
    fn default() -> Self {
        let mut result = Self {
            aspect_ratio: 30.,

            values: vec![
                (
                    PlotDataAndSettings {
                        name: "Day 1".to_string(),
                        data: DATA1.to_owned(),
                        settings: None,
                    },
                    Ok(Default::default()),
                ),
                (
                    PlotDataAndSettings {
                        name: "Day 2".to_string(),
                        data: DATA2.to_owned(),
                        settings: None,
                    },
                    Ok(Default::default()),
                ),
            ],

            global_settings: PlotSettings {
                n: 7,
                seg_mult: 7,
                border: 1.0,
                filter_extreme: true,
                distance_to_median: 10.5,
                bar_chart_width: 0.5,
            },
        };

        result.init();

        result
    }
}

impl PlotSettings {
    fn ui(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;

        changed |= ui
            .add(egui::Slider::new(&mut self.border, 0.0..=10.0).text("Border"))
            .changed();
        changed |= ui
            .add(egui::Slider::new(&mut self.bar_chart_width, 0.01..=2.0).text("Bar chart size"))
            .changed();
        changed |= ui
            .add(egui::Slider::new(&mut self.n, 1..=30).text("N"))
            .changed();
        changed |= ui
            .add(egui::Slider::new(&mut self.seg_mult, 1..=30).text("K"))
            .changed();

        changed |= ui
            .checkbox(&mut self.filter_extreme, "Filter extreme values")
            .changed();
        if self.filter_extreme {
            changed |= ui
                .add(egui::Slider::new(&mut self.distance_to_median, 0.1..=20.).text("Distance"))
                .changed();
        }

        changed
    }
}

impl PlotDataAndSettings {
    fn calc(&self, global_settings: &PlotSettings) -> Result<CalcedData, String> {
        let settings = if let Some(settings) = self.settings.as_ref() {
            settings
        } else {
            global_settings
        };

        let mut result = CalcedData {
            name: self.name.clone(),
            data_parsed: self
                .data
                .split(' ')
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<f64>().map_err(|_| x.to_string()))
                .collect::<Result<Vec<_>, _>>()?,
            ..CalcedData::default()
        };

        if result.data_parsed.is_empty() {
            return Err(String::new());
        }

        sort_f64(&mut result.data_parsed);

        result.data_size_before_filters = result.data_parsed.len();

        if settings.filter_extreme {
            result.data_parsed = filter_times(&result.data_parsed, settings.distance_to_median);
        }

        result.polynom = approximate_distribution_by_polynom(
            &result.data_parsed,
            settings.n,
            settings.seg_mult,
            settings.border,
        );

        result.minv = *result
            .data_parsed
            .iter()
            .min_by_key(|x| OrderedFloat::from(**x))
            .unwrap();
        result.maxv = *result
            .data_parsed
            .iter()
            .max_by_key(|x| OrderedFloat::from(**x))
            .unwrap();
        result.median = median(&result.data_parsed);
        result.avg = result.data_parsed.iter().sum::<f64>() / result.data_parsed.len() as f64;

        let count = GRAPH_STEP_COUNT;
        let step = (result.maxv - result.minv) / count as f64;

        result.values.clear();
        for i in 0..count {
            let x = result.minv + i as f64 * step;
            let y = calc_polynom_value(x, &result.polynom);
            result.values.push([x, y]);
        }

        result.bar_chart.clear();
        let mut count = 0;
        let mut pos = result.data_parsed[0];
        for i in &result.data_parsed {
            while *i > pos + settings.bar_chart_width {
                result.bar_chart.push(
                    Bar::new(
                        pos + settings.bar_chart_width / 2.0,
                        count as f64 / result.data_parsed.len() as f64 / settings.bar_chart_width,
                    )
                    .width(settings.bar_chart_width),
                );
                count = 0;
                pos += settings.bar_chart_width;
            }
            count += 1;
        }
        result.bar_chart.push(
            Bar::new(
                pos + settings.bar_chart_width / 2.0,
                count as f64 / result.data_parsed.len() as f64 / settings.bar_chart_width,
            )
            .width(settings.bar_chart_width),
        );

        Ok(result)
    }

    fn ui(&mut self, ui: &mut egui::Ui, pos: usize) -> bool {
        let mut changed = false;

        ui.heading("Data settings");

        ui.horizontal(|ui| {
            ui.label("Name: ");
            changed |= ui.text_edit_singleline(&mut self.name).changed();
        });

        ui.label("Write data separated by spaces: ");
        ui.push_id(pos, |ui| {
            egui::ScrollArea::vertical()
                .max_height(100.)
                .show(ui, |ui| {
                    changed |= ui.text_edit_multiline(&mut self.data).changed();
                });
        });

        ui.separator();

        let mut has_settings = self.settings.is_some();
        if ui
            .checkbox(&mut has_settings, "Use custom settings")
            .changed()
        {
            changed = true;
            if self.settings.is_some() {
                self.settings = None;
            } else {
                self.settings = Some(PlotSettings {
                    n: 7,
                    seg_mult: 7,
                    border: 1.0,
                    filter_extreme: true,
                    distance_to_median: 1.5,
                    bar_chart_width: 0.5,
                });
            }
        }
        if let Some(settings) = self.settings.as_mut() {
            ui.heading("Distribution Settings");
            changed |= settings.ui(ui);
        }

        changed
    }
}

impl CalcedData {
    fn ui(&self, ui: &mut egui::Ui) {
        ui.heading("Info about data");

        ui.label(format!(
            "Count of points before filters: {}",
            self.data_size_before_filters
        ));
        ui.label(format!(
            "Count of points after filters: {}",
            self.data_parsed.len()
        ));
        ui.label(format!("Min: {}", self.minv));
        ui.label(format!("Max: {}", self.maxv));
        ui.label(format!("Avg: {}", self.avg));
        ui.label(format!("Median: {}", self.median));

        ui.horizontal(|ui| {
            ui.label("Polynom: ");
            let mut polynom_str = print_polynom(&self.polynom, false);
            ui.text_edit_singleline(&mut polynom_str);
        });

        ui.horizontal(|ui| {
            ui.label("Polynom for desmos: ");
            let mut polynom_str = print_polynom(&self.polynom, true);
            ui.text_edit_singleline(&mut polynom_str);
        });
    }

    fn plot_ui(&self, plot_ui: &mut PlotUi) {
        plot_ui.line(
            Line::new(self.values.clone())
                .fill(0.)
                .name(format!("Polynom of `{}`", self.name)),
        );
        plot_ui.bar_chart(
            BarChart::new(self.bar_chart.clone()).name(format!("Histogram of `{}`", self.name)),
        );
    }
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            let mut result: App = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
            result.init();
            return result;
        }

        Default::default()
    }

    fn init(&mut self) {
        for (plot_data_and_settings, calced_data) in &mut self.values {
            *calced_data = plot_data_and_settings.calc(&self.global_settings);
        }
    }
}

impl eframe::App for App {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            values,
            aspect_ratio,
            global_settings,
        } = self;

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.heading("Interface settings");
                ui.add(egui::Slider::new(aspect_ratio, 0.1..=50.0).text("Aspect ratio"));

                ui.separator();

                ui.heading("Global settings");
                let changed_global = global_settings.ui(ui);

                ui.separator();

                for (pos, (plot_data_and_settings, calced_data)) in values.iter_mut().enumerate() {
                    let mut changed = changed_global && plot_data_and_settings.settings.is_none();

                    changed |= plot_data_and_settings.ui(ui, pos);
                    if let Err(error) = calced_data {
                        ui.label(
                            RichText::new(format!("Error parsing data - not a number: `{error}`"))
                                .color(Color32::RED),
                        );
                    }

                    ui.separator();

                    if let Ok(calced_data) = calced_data {
                        calced_data.ui(ui);
                    }

                    if changed {
                        *calced_data = plot_data_and_settings.calc(global_settings);
                    }

                    ui.separator();
                    ui.separator();
                    ui.separator();
                }

                if ui.button("Add values").clicked() {
                    let plot = PlotDataAndSettings {
                        name: "Day 2".to_string(),
                        data: DATA2.to_owned(),
                        settings: None,
                    };
                    let calced_data = plot.calc(global_settings);
                    values.push((plot, calced_data));
                }

                if ui
                    .add_enabled(!values.is_empty(), egui::Button::new("Remove bottom value"))
                    .clicked()
                {
                    values.pop();
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let plot = Plot::new("items_demo")
                .legend(Legend::default().position(Corner::RightBottom))
                .show_x(false)
                .show_y(false)
                .data_aspect(*aspect_ratio as f32);
            plot.show(ui, |plot_ui| {
                for (_, calced_data) in values.iter_mut() {
                    if let Ok(calced_data) = calced_data {
                        calced_data.plot_ui(plot_ui);
                    }
                }
            });
        });
    }
}
