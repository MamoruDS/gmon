use crate::nv_gpu;

use chrono::{DateTime, Local};
use owo_colors::colors as Colors;
use owo_colors::{OwoColorize, Stream::Stdout, Style, Styled};
use terminal_size::{terminal_size, Width};

pub struct DF {
    is_support_color: bool,
    pad_fill: char,
}

impl DF {
    pub fn new() -> Self {
        let text = String::from("A");
        let text_colored = format!(
            "{}",
            text.if_supports_color(Stdout, |t| t.fg::<Colors::Yellow>())
        );
        DF {
            is_support_color: text.len() != text_colored.len(),
            pad_fill: ' ',
        }
    }
    pub fn try_style<'a, T: OwoColorize>(&self, input: &'a T, style: Style) -> Styled<&'a T> {
        if self.is_support_color {
            input.style(style)
        } else {
            let style = Style::new();
            input.style(style)
        }
    }
    pub fn padding(
        &self,
        val: (&nv_gpu::Value, Option<Style>),
        val_fmt: Option<fn(&nv_gpu::Value) -> String>,
        prefix: (Option<&str>, Option<Style>),
        suffix: (Option<&str>, Option<Style>),
        width: usize,
        pad_position: Pad,
        pad_fill: Option<char>,
    ) -> String {
        fn default_val_fmt(val: &nv_gpu::Value) -> String {
            val.val_as_isize().to_string()
        }
        let prefix_str = prefix.0.unwrap_or("");
        let val_string = val_fmt.unwrap_or(default_val_fmt)(val.0);
        let unit_str = match val.0.unit.as_ref() {
            Some(unit) => unit.as_str(),
            None => "",
        };
        let suffix_str = suffix.0.unwrap_or(unit_str);
        let val_str = format!("{}{}{}", prefix_str, val_string, suffix_str);
        let padding_size = if width > val_str.len() {
            width - val_str.len()
        } else {
            0
        };
        let formatted_val_str = format!(
            "{}{}{}",
            match prefix.1 {
                Some(style) => format!("{}", self.try_style(&prefix_str, style)),
                None => format!("{}", prefix_str),
            },
            match val.1 {
                Some(style) => format!("{}", self.try_style(&val_string, style)),
                None => format!("{}", val_string),
            },
            match suffix.1 {
                Some(style) => format!("{}", self.try_style(&suffix_str, style)),
                None => format!("{}", suffix_str),
            },
        );
        if padding_size == 0 {
            formatted_val_str
        } else {
            let padding =
                String::from_iter((0..padding_size).map(|_| pad_fill.unwrap_or(self.pad_fill)));
            match pad_position {
                Pad::Left => format!("{}{}", padding, formatted_val_str),
                Pad::Right => format!("{}{}", formatted_val_str, padding),
            }
        }
    }
}

fn get_width() -> usize {
    let w = match terminal_size() {
        Some((Width(w), _)) => w,
        None => 60,
    };
    w as usize
}

enum Pad {
    Left,
    Right,
}

fn padding(
    text: String,
    pad: Pad,
    pad_size: usize,
    fill: Option<char>,
    post_fmt: Option<fn(String) -> String>,
) -> String {
    let ori_len = text.chars().count();
    let text = match post_fmt {
        Some(fmt) => fmt(text),
        None => text,
    };
    let size = (pad_size - ori_len).max(0);
    if size == 0 {
        text
    } else {
        let p = String::from_iter((0..size).map(|_| fill.unwrap_or(' ')));
        match pad {
            Pad::Left => format!("{}{}", p, text),
            Pad::Right => format!("{}{}", text, p),
        }
    }
}

pub fn print_header(info: &nv_gpu::GPUInfo) {
    println!("width: ({})", get_width());
    let dt_text = format!("{} {}", &info.timestamp, Local::now().format("%z"));
    let dt = DateTime::parse_from_str(&dt_text, "%a %b %e %H:%M:%S %Y %z").unwrap();
    let l = dt.format("%Y/%m/%d %H:%M:%S");
    let r = format!(
        "{} CUDA:{}",
        info.driver_version,
        info.cuda_version.fg::<Colors::Yellow>(),
    );
    println!(
        "{}{}{}",
        l,
        String::from_iter((0..get_width() - 19 - 19).map(|_| ' ')),
        r,
    );
}

pub fn print_gpu_info_basic(info: &nv_gpu::GPUInfo) {
    let df = DF::new();
    let mut max_mem_str_len: Option<usize> = None;
    for (i, gpu) in info.gpus.iter().enumerate() {
        if max_mem_str_len.is_none() {
            max_mem_str_len = Some(
                format!(
                    "{}",
                    match gpu.fb_memory_usage.total.val {
                        nv_gpu::ValidValue::INT(v) => {
                            v
                        }
                        _ => 1000,
                    }
                )
                .len(),
            );
        }
        let gpu_id = df.try_style(&i, Style::new().fg::<Colors::Yellow>());
        // let product_name
        let temperature = {
            let read = gpu.temperature.value.val_as_isize();
            let style = Style::new();
            let style = if read > 75 {
                style.fg::<Colors::Red>()
            } else if read > 50 {
                style.fg::<Colors::Yellow>()
            } else if read > 30 {
                style.fg::<Colors::Green>()
            } else {
                style.fg::<Colors::Blue>()
            };
            df.padding(
                (&gpu.temperature.value, Some(style)),
                None,
                (None, None),
                (Some("°C"), Some(style)),
                3,
                Pad::Left,
                None,
            )
        };
        let power_draw = df.padding(
            (&gpu.power_readings.draw, None),
            None,
            (None, None),
            (Some(""), None),
            3,
            Pad::Left,
            None,
        );
        let power_limit = {
            if gpu
                .power_readings
                .limit
                .val
                .eq(&gpu.power_readings.default_limit.val)
            {
                String::from("")
            } else {
                df.padding(
                    (
                        &gpu.power_readings.limit,
                        Some(Style::new().fg::<Colors::Blue>()),
                    ),
                    None,
                    (None, None),
                    (Some(""), None),
                    3,
                    Pad::Left,
                    None,
                )
            }
        };
        let usage = df.padding(
            (&gpu.utilization.gpu, None),
            None,
            (None, None),
            (Some(""), None),
            3,
            Pad::Left,
            None,
        );
        let mem_usage = df.padding(
            (&gpu.fb_memory_usage.used, None),
            None,
            (None, None),
            (Some(""), None),
            max_mem_str_len.unwrap_or(4),
            Pad::Left,
            None,
        );
        let mem_total = df.padding(
            (&gpu.fb_memory_usage.total, None),
            None,
            (None, None),
            (Some(""), None),
            max_mem_str_len.unwrap_or(4),
            Pad::Left,
            None,
        );

        println!(
            "{} {} {} {}{}W {}% {}/{}MB",
            gpu_id,
            gpu.product_name,
            temperature,
            power_draw,
            power_limit,
            usage,
            mem_usage,
            mem_total,
        );
    }
}
