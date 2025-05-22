#[cfg(test)]
mod tests;

use crate::result_analyser::charts::chart_data::{ChartData, ChartDataEntry};
use crate::result_analyser::charts::{AspectRatio, ChartConfig, Resolution, SVGString};
use std::cmp::max;
use std::ops::Mul;

const VERTICAL_LABEL_WIDTH: usize = 200;
const HORIZONTAL_LABEL_HEIGHT: usize = 100;
const SPEED_SCALE: usize = 10;

pub fn generate(data: &ChartData, config: &ChartConfig) -> SVGString {
    let graph_resolution = graph_resolution(data);
    let scaled_graph_resolution = scaled_graph_resolution(&graph_resolution, &config);
    let actual_resolution = actual_chart_resolution(&scaled_graph_resolution);
    let actual_chart_width = actual_resolution.x;
    let actual_chart_height = actual_resolution.y;
    let graph_width = graph_resolution.x;
    let graph_height = graph_resolution.y;
    let scaled_graph_width = scaled_graph_resolution.x;

    let speed_graph = generate_speed_curve(&data, graph_height);
    let svg = format!(r#"
        <svg viewBox="0 0 {actual_chart_width} {actual_chart_height}" xmlns="http://www.w3.org/2000/svg">
            <style type="text/css">
                <![CDATA[
                    .speed-curve {{
                        stroke-width: 1;
                        stroke: black;
                        fill: none;
                    }}
                ]]>
            </style>
            <svg x="{VERTICAL_LABEL_WIDTH}" y="0" width="{scaled_graph_width}" height="{graph_height}" viewBox="0 0 {graph_width} {graph_height}" xmlns="http://www.w3.org/2000/svg">
                <rect x="0" y="0" width="{graph_width}" height="{graph_height}" stroke="black" fill="white"/>
                {speed_graph}
            </svg>
        </svg>
    "#).into();
    svg
}

fn graph_resolution(data: &ChartData) -> Resolution {
    let height = data.entries().iter().fold(0, |height, entry| {
        let entry_height = entry.max_speed_value().kilometers_per_hour().mul(SPEED_SCALE as f32).ceil() as usize + 5 * SPEED_SCALE;
        max(height, entry_height)
    });
    Resolution {
        x: data.entries().iter()
            .map(|entry| entry.distance)
            .max_by(|a, b|
                a.partial_cmp(&b).unwrap()
            ).unwrap_or_default().ceil() as usize,
        y: height,
    }
}

fn scaled_graph_resolution(graph_resolution: &Resolution, config: &ChartConfig) -> Resolution {
    match config.aspect_ratio {
        AspectRatio::Auto => (*graph_resolution).clone(),
        AspectRatio::Fixed(value) => Resolution {
            x: (graph_resolution.x as f64 * value).round() as usize,
            y: 0,
        },
    }
}

fn actual_chart_resolution(scaled_graph_resolution: &Resolution) -> Resolution {
    Resolution {
        x: scaled_graph_resolution.x + VERTICAL_LABEL_WIDTH,
        y: scaled_graph_resolution.y + HORIZONTAL_LABEL_HEIGHT,
    }
}

fn generate_speed_curve(data: &ChartData, y_zero: usize) -> SVGString {
    let points = data.entries().iter()
        .map(
            |entry|
            format!(
                "{},{}",
                entry.distance,
                y_zero - entry.actual_speed.kilometers_per_hour().round() as usize * SPEED_SCALE,
            )
        )
        .collect::<Vec<String>>().join(" ");
    format!(r#"<polyline points="{points}" class="speed-curve"/>"#).into()
}