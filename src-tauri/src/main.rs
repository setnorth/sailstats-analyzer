#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::cmp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Deref;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

use tauri::State;

//Fields in the file. Derived values are awd, tws, twd, twa
const FILE_FIELD_NAMES: [&str; 13] = [
    "awa",
    "aws",
    "latitude",
    "longitude",
    "hdg",
    "cog",
    "sog",
    "stw",
    "rot",
    "pitch",
    "yaw",
    "roll",
    "rudder_angle",
];

//Fields that get treated for outliers by removing the top 1%
const OUTLIER_FIELDS: [&str; 2] = ["aws", "stw"];

///Number of datapoints per vector returned to frontend.
const SLICE_CHUNK_SIZE: usize = 250;
const PATH_CHUNK_SIZE: usize = 100;

///X,Y Point type
#[derive(serde::Serialize, Default, Debug, Clone)]
struct Point {
    x: String, //Time as string, processing to time value is done in frontend
    y: f32,    //Value
}

/// Keeps track of all data points
/// The csv values time;wind_angle;wind_speed;latitude;longitude;heading;cog;sog;stw;rot;pitch;yaw;roll;rudder_angle
/// are processed and stored in the form of Vec[Point{x: timestring, y: val}] in a hashmap according to their key,
/// i.e., wind_angle, wind_speed, etc
#[derive(Default)]
struct Data(Arc<Mutex<HashMap<String, Vec<Point>>>>);

/// Gets rid of the somewhat hard to read "data.0" notation and returns the content
impl Deref for Data {
    type Target = Arc<Mutex<HashMap<String, Vec<Point>>>>;
    fn deref(&self) -> &Arc<Mutex<HashMap<String, Vec<Point>>>> {
        &self.0
    }
}

///Returns vectors for the selected fields between start and stop.
///At most PORTION_SIZE datapoints per field.
#[tauri::command]
async fn get_slices(
    fields: Vec<String>,
    start: usize,
    stop: usize,
    data: State<'_, Data>,
) -> Result<Vec<Vec<Point>>, String> {
    let mut rval: Vec<Vec<Point>> = Vec::new();
    let db = data.lock().or(Err("Could not lock db."))?;

    let step_size: usize = ((stop - start) / SLICE_CHUNK_SIZE) + 1; //+1 to avoid stepsize 0

    for field in fields {
        let src_vec: &Vec<Point> = db
            .get(&field)
            .ok_or_else(|| format!("Could not get field {}.", field))?;
        //Vector might be shorter due to filtered top 1% values
        let stop = cmp::min(stop, src_vec.len());
        let field_vec: Vec<Point> = src_vec[start..stop]
            .iter()
            .step_by(step_size)
            .cloned()
            .collect();
        rval.push(field_vec);
    }
    Ok(rval)
}

/// Returns polar vectors of averages or maximums for fields between start and stop.
/// (polar -> vector has exactly 360 f32 elements).
/// The field names decide the mode of calculation.
/// Field name syntax (<mode>,<table to process>,<direction table>).
/// Example: The string "avg,aws,awa" calculates average AWS for all AWA in the interval.
/// Possible modes: avg, dist
#[tauri::command]
async fn get_radars(
    fields: Vec<String>,
    start: usize,
    stop: usize,
    data: State<'_, Data>,
) -> Result<Vec<Vec<f32>>, String> {
    let db = data.lock().or(Err("Could not lock db"))?;
    let mut res: Vec<Vec<f32>> = Vec::new();

    for field in fields {
        let mode: Vec<&str> = field.split(',').collect();
        let mut fres: [f32; 360] = [0.0; 360]; //Result for this field

        //Calculate the average winds for the different wind directions
        if mode[0] == "avg" {
            let quantities: &Vec<Point> = db
                .get(mode[1])
                .ok_or_else(|| format!("Table {} not found.", mode[1]))?;
            let directions: &Vec<Point> = db
                .get(mode[2])
                .ok_or_else(|| format!("Table {} not found.", mode[2]))?;

            let mut fcount: [f32; 360] = [0.0; 360]; //Counter to calculate average

            //Vector might be shorter due to filtered top 1% values
            let stop = cmp::min(stop, quantities.len());
            let stop = cmp::min(stop, directions.len());

            for (value, direction) in quantities[start..stop]
                .iter()
                .zip(directions[start..stop].iter())
            {
                let direction = direction.y.floor() as usize % 360;
                let value = value.y;
                fres[direction] =
                    (fres[direction] * fcount[direction] + value) / (fcount[direction] + 1.0);
                fcount[direction] += 1.0;
            }
        //Calculate the percentage a specific angle is measured
        } else if mode[0] == "dist" {
            let mut occurences: &[Point] = &db
                .get(mode[1])
                .ok_or_else(|| format!("Table {} not found.", mode[1]))?;
            //Vector might be shorter due to filtered top 1% values
            let stop = cmp::min(stop, occurences.len());
            occurences = &occurences[start..stop];

            for occurence in occurences {
                fres[occurence.y.floor() as usize % 360] += 1.0;
            }
            //Get the percentage
            fres = fres.map(|x| x / (stop - start) as f32 * 100.0);
        }
        res.push(fres.to_vec());
    }
    Ok(res)
}

///Returns the path as vector of [lat,long] between start and stop
#[tauri::command]
fn get_path(start: usize, stop: usize, data: State<Data>) -> Result<Vec<[f32; 2]>, &str> {
    let mut path: Vec<[f32; 2]> = Vec::new();
    let db = data.lock().or(Err("Could not lock data."))?;

    let step_size: usize = ((stop - start) / PATH_CHUNK_SIZE) + 1;

    let lats = db
        .get("latitude")
        .ok_or("Could not find field 'latitude'.")?;
    let longs = db
        .get("longitude")
        .ok_or("Could not find field 'longitude'.")?;

    for (lat, long) in lats[start..stop]
        .iter()
        .step_by(step_size)
        .zip(longs[start..stop].iter().step_by(step_size))
    {
        path.push([lat.y, long.y]);
    }
    Ok(path)
}

/// Reads the file and write the data to the State, derive values after reading
/// Cuts out the top 1% of data to correct for outliers.
#[tauri::command]
async fn read_file(file_name: String, data: State<'_, Data>) -> Result<(), String> {
    let stream = File::open(file_name).or(Err("Cannot read file!"))?;
    let reader = BufReader::new(stream);
    let mut db = data.lock().or(Err("Cannot lock database."))?;

    for field in FILE_FIELD_NAMES {
        db.insert(field.to_string(), Vec::new());
    }

    //Skip the first two lines. First line contains the header, the second line contains
    //mostly empty fields and gives especially in the position data nonsensical information.
    for line in reader.lines().skip(2) {
        let line = line.or(Err("Cannot read line!"))?;
        let fields: Vec<&str> = line.split(';').collect();

        let time = fields[0].to_string();

        for (field_id, field_name) in (1..FILE_FIELD_NAMES.len() + 1).zip(FILE_FIELD_NAMES) {
            db.get_mut(field_name)
                .ok_or_else(|| format!("Could not get field {}.", field_name))?
                .push(Point {
                    x: time.clone(),
                    y: f32::from_str(fields[field_id]).or(Err("Could not parse value."))?,
                });
        }
    }

    // Treat outliers in fields by finding the minimum and the maximum values and then
    // removing the points which represent the top 1% of values.
    for field in OUTLIER_FIELDS {
        let vec: &mut Vec<Point> = db
            .get_mut(field)
            .ok_or_else(|| format!("Could not get field {}.", field))?;
        let max: f32 = vec
            .iter()
            .map(|p| p.y)
            .reduce(f32::max)
            .ok_or_else(|| format!("Could could not reduce field {}.", field))?;
        //Assuming that values start at 0 ...
        let threshold = max * 0.99;
        vec.retain(|p| p.y <= threshold);
    }

    derive_values(&mut *db)?;

    //Create low pass filtered sensor data
    let low_pass_fields = [
        ("aws", 300),
        ("awa", 100),
        ("awd", 100),
        ("tws", 300),
        ("twa", 100),
        ("twd", 100),
        ("stw", 100),
        ("roll", 200),
        ("rudder_angle", 100),
    ];
    for (field, magnitude) in low_pass_fields {
        let v = low_pass(
            &db.get(field)
                .ok_or_else(|| format!("Could not get {}", field))?
                .to_vec(),
            magnitude,
        );
        let mut f = field.to_string();
        f.push_str("_lp");
        db.insert(f, v);
    }

    Ok(())
}

///Derives awd, tws, twd and twa
fn derive_values(db: &mut HashMap<String, Vec<Point>>) -> Result<(), String> {
    let awa_vec = db.get("awa").ok_or("Could not get field 'awa'.")?;
    let aws_vec = db.get("aws").ok_or("Could not get field 'aws'.")?;
    let hdg_vec = db.get("hdg").ok_or("Could not get field 'hdg'.")?;
    let cog_vec = db.get("cog").ok_or("Could not get field 'cog'.")?;
    let sog_vec = db.get("sog").ok_or("Could not get field 'sog'.")?;

    let mut awd_vec: Vec<Point> = Vec::new();
    let mut tws_vec: Vec<Point> = Vec::new();
    let mut twd_vec: Vec<Point> = Vec::new();
    let mut twa_vec: Vec<Point> = Vec::new();

    for ((((p_awa, p_aws), p_hdg), p_cog), p_sog) in awa_vec
        .iter()
        .zip(aws_vec)
        .zip(hdg_vec)
        .zip(cog_vec)
        .zip(sog_vec)
    {
        let (awa, aws, hdg, cog, sog) = (p_awa.y, p_aws.y, p_hdg.y, p_cog.y, p_sog.y);

        let awd = if (hdg + awa) < 360.0 {
            hdg + awa
        } else {
            hdg + awa - 360.0
        };

        let u = sog * cog.to_radians().sin() - aws * awd.to_radians().sin();
        let v = sog * cog.to_radians().cos() - aws * awd.to_radians().cos();

        let tws = (u.powi(2) + v.powi(2)).sqrt();
        //Not entirely sure why we need to add 180 degrees here ... but it works so ..
        let twd = u.atan2(v).to_degrees() + 180.0;

        let twa = if (twd - hdg) >= 0.0 {
            twd - hdg
        } else {
            twd - hdg + 360.0
        };

        let time = &p_awa.x;

        awd_vec.push(Point {
            x: time.clone(),
            y: awd,
        });
        tws_vec.push(Point {
            x: time.clone(),
            y: tws,
        });
        twd_vec.push(Point {
            x: time.clone(),
            y: twd,
        });
        twa_vec.push(Point {
            x: time.clone(),
            y: twa,
        });
    }

    db.insert("awd".to_string(), awd_vec);
    db.insert("twa".to_string(), twa_vec);
    db.insert("twd".to_string(), twd_vec);
    db.insert("tws".to_string(), tws_vec);

    Ok(())
}

/// Smoothes over data obtained by sensors (low pass filter, moving average).
/// Takes a vector as input and returns a new one with values computed in relation
/// to window size wsize.
fn low_pass(raw_vec: &[Point], wsize: usize) -> Vec<Point> {
    assert!(raw_vec.len() > wsize);

    //Copy half the window size to the new vector, we need that as initialization
    let mut lp_vec = raw_vec[..wsize / 2].to_vec();
    //Calculate the first value and push it
    lp_vec.push(Point {
        x: raw_vec[wsize / 2].x.to_string(),
        y: raw_vec[..wsize].iter().map(|p| p.y).sum::<f32>() / wsize as f32,
    });
    //The new vector is now wsize/2+1 in length, i.e., its last element is at index wsize/2

    //Starting at the next element in the raw vector, i.e. wsize/2+1,
    //until the window touches the end of the raw vector, i.e. raw_vec.len()-wsize/2
    for i in wsize / 2 + 1..raw_vec.len() - wsize / 2 {
        //lp_vec.push(raw_vec[i].clone());
        lp_vec.push(Point {
            x: raw_vec[i].x.to_string(),
            y: lp_vec[i - 1].y
                + (raw_vec[i + wsize / 2].y - raw_vec[i - wsize / 2].y) / wsize as f32,
        });
    }

    //Append the last values, i.e., the remaining half window size
    lp_vec.append(&mut raw_vec[raw_vec.len() - wsize / 2..].to_vec());

    assert!(raw_vec.len() == lp_vec.len());
    lp_vec
}

#[tauri::command]
async fn get_stats(
    fields: Vec<String>,
    start: usize,
    stop: usize,
    data: State<'_, Data>,
) -> Result<HashMap<String, (f32, f32, f32)>, String> {
    let mut res: HashMap<String, (f32, f32, f32)> = HashMap::new();
    let db = data.lock().or(Err("Could not lock data."))?;
    for field in fields {
        let stop = cmp::min(
            stop,
            db.get(&field)
                .ok_or_else(|| format!("Could not get field '{}'.", field))?
                .len(),
        );

        let f = &db
            .get(&field)
            .ok_or_else(|| format!("Could not get field '{}'.", field))?[start..stop];
        let min = f
            .iter()
            .map(|p| p.y)
            .reduce(f32::min)
            .ok_or_else(|| format!("Could not get minimum for field '{}'.", field))?;
        let avg = f.iter().map(|p| p.y).sum::<f32>() / (stop - start) as f32;
        let max = f
            .iter()
            .map(|p| p.y)
            .reduce(f32::max)
            .ok_or_else(|| format!("Could not get minimum for field '{}'.", field))?;
        res.insert(field.clone(), (min, avg, max));
    }
    Ok(res)
}

#[tauri::command]
fn get_file_lines(data: State<Data>) -> Result<usize, String> {
    Ok(data
        .lock()
        .or(Err("Could not lock data."))?
        .get(FILE_FIELD_NAMES[0])
        .ok_or_else(|| format!("Could not get field {}", FILE_FIELD_NAMES[0]))?
        .len())
}

fn main() {
    tauri::Builder::default()
        .manage(Data(Default::default()))
        .invoke_handler(tauri::generate_handler![
            read_file,
            get_slices,
            get_radars,
            get_stats,
            get_file_lines,
            get_path
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
