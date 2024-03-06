use actix_web::{http,web, App, HttpServer, HttpResponse, Error};
use actix_cors::Cors;
use plotters::prelude::*;
use serde::{Deserialize, Serialize};
// use serde_json::Result;

#[derive(Debug,Deserialize)]
struct RequestData {
    initial_mach: f64,
    initial_pressure: f64,
    initial_temperature: f64,
    nozzle_angle: f64,
}

#[derive(Serialize, Deserialize)]
struct OutputData {
    distances: Vec<f64>,
    mach_numbers: Vec<f64>,
    temperatures: Vec<f64>,
    pressures: Vec<f64>,
   
}

async fn calculate_characteristics( 
    req_data: web::Json<RequestData>,
) -> Result<HttpResponse,Error> {
    println!("{:?}", req_data);
    let initial_mach = req_data.initial_mach;
    let specific_heat_ratio = 1.4;
    let nozzle_angle_rad = req_data.nozzle_angle.to_radians();
    let initial_pressure = req_data.initial_pressure;
    let initial_temperature = req_data.initial_temperature;
    let num_points =1000;

    let (distances, mach_numbers, temperatures, pressures) = calculate_characteristics_impl(
        initial_mach,
        specific_heat_ratio,
        nozzle_angle_rad,
        initial_pressure,
        initial_temperature,
        num_points,
        
    );
    let root_mach = BitMapBackend::new("../frontend/src/mach_plot.png", (800, 600)).into_drawing_area();
    root_mach.fill(&WHITE).unwrap();
    let mut chart_mach = ChartBuilder::on(&root_mach)
        .caption("Mach Number Variation in Converging Nozzle", ("Arial", 20).into_font())
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0.0..0.2, 0.0..1.5)
        .unwrap();

    chart_mach
        .configure_mesh()
        .x_desc("Distance (x)")
        .y_desc("Mach Number")
        .axis_desc_style(("Arial", 15).into_font())
        .draw()
        .unwrap();

    chart_mach
        .draw_series(LineSeries::new(
            distances.iter().zip(mach_numbers.iter()).map(|(x, m)| (*x, *m)),
            &RED,
        ))
        .unwrap()
        .label("Mach Number")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart_mach
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .position(SeriesLabelPosition::UpperRight)
        .draw()
        .unwrap();
    let root_temp = BitMapBackend::new("../frontend/src/temp_plot.png", (800, 600)).into_drawing_area();
    root_temp.fill(&WHITE).unwrap();
    let mut chart_temp = ChartBuilder::on(&root_temp)
        .caption("Temperature Variation in Converging Nozzle", ("Arial", 20).into_font())
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0.0..0.2, 0.0..400.0)
        .unwrap();

    chart_temp
        .configure_mesh()
        .x_desc("Distance (x)")
        .y_desc("Temperature")
        .axis_desc_style(("Arial", 15).into_font())
        .draw()
        .unwrap();

    chart_temp
        .draw_series(LineSeries::new(
            distances.iter().zip(temperatures.iter()).map(|(x, m)| (*x, *m)),
            &BLUE,
        ))
        .unwrap()
        .label("Tempereature")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    chart_temp
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .position(SeriesLabelPosition::UpperRight)
        .draw()
        .unwrap();
    let root_pressure = BitMapBackend::new("../frontend/src/pressure_plot.png", (800, 600)).into_drawing_area();
    root_pressure.fill(&WHITE).unwrap();
    let mut chart_press = ChartBuilder::on(&root_pressure)
        .caption("Temperature Variation in Converging Nozzle", ("Arial", 20).into_font())
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0.0..0.2, 0.0..400.0)
        .unwrap();

    chart_press
        .configure_mesh()
        .x_desc("Distance (x)")
        .y_desc("Pressure")
        .axis_desc_style(("Arial", 15).into_font())
        .draw()
        .unwrap();

    chart_press
        .draw_series(LineSeries::new(
            distances.iter().zip(pressures.iter()).map(|(x, m)| (*x, *m)),
            &GREEN,
        ))
        .unwrap()
        .label("Pressure")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));

    chart_press
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .position(SeriesLabelPosition::UpperRight)
        .draw()
        .unwrap();

    

    Ok(HttpResponse::Ok().json(OutputData {
        distances,
        mach_numbers,
        temperatures,
        pressures,
    }))
}

fn calculate_characteristics_impl(
    initial_mach: f64,
    specific_heat_ratio: f64,
    nozzle_angle_rad: f64,
    initial_pressure: f64,
    initial_temperature: f64,
    num_points: usize,
) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
    let mut distances = Vec::with_capacity(num_points);
    let mut mach_numbers = Vec::with_capacity(num_points);
    let mut temperatures = Vec::with_capacity(num_points);
    let mut pressures = Vec::with_capacity(num_points);

    distances.push(0.0);
    mach_numbers.push(initial_mach);
    temperatures.push(initial_temperature / (1.0
        + ((specific_heat_ratio - 1.0) / 2.0) * mach_numbers[0].powi(2)),);
    pressures.push(
        initial_pressure / (1.0 + (specific_heat_ratio - 1.0) / 2.0 * initial_mach.powi(2))
            .powf(specific_heat_ratio / (specific_heat_ratio - 1.0)),
    );
    // let astar = 0.6289;
    // let h_initial = 10;
    // let h_final =
    for i in 1..num_points {
        let dx = distances[i - 1] + 1.0 / (num_points as f64 - 1.0);

        let d_mach =
            nozzle_angle_rad / (1.0 + mach_numbers[i - 1].powi(2) * (specific_heat_ratio
                + (2.0 * nozzle_angle_rad).cos()));
        mach_numbers.push(mach_numbers[i - 1] + d_mach * dx);

        if mach_numbers[i] >= 1.0 {
            break;
        }

        temperatures.push(
            initial_temperature / (1.0
                + ((specific_heat_ratio - 1.0) / 2.0) * mach_numbers[i - 1].powi(2)),
        );
        pressures.push(
            initial_pressure / (1.0
                + (specific_heat_ratio - 1.0) / 2.0 * mach_numbers[i - 1].powi(2))
                .powf(specific_heat_ratio / (specific_heat_ratio - 1.0)),
        );
        distances.push(dx);
    }

    (distances, mach_numbers, temperatures, pressures)
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000") 
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::ACCEPT,
                http::header::CONTENT_TYPE,
            ])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(web::resource("/calculate_properties").route(web::post().to(calculate_characteristics)))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await?;

    Ok(())
}