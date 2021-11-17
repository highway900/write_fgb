use std::io::BufWriter;

use flatgeobuf::{ColumnType, FgbWriter, GeometryType};
use geo::LineString;
use geozero::{error::Result, ColumnValue, PropertyProcessor};

pub fn write_fgb_file(filename: String, linestrings: &Vec<LineString<f64>>) -> Result<()> {
    let mut fgb = FgbWriter::create(
        "countries",
        GeometryType::LineString,
        Some(4326),
        |header| {
            header.description = Some(FgbWriter::create_string("Country polygons"));
        },
    )?;
    fgb.add_column("fid", ColumnType::ULong, |col| {
        col.nullable = false;
    });
    fgb.add_column("name", ColumnType::String, |_| {});

    for ls in linestrings.iter() {
        //let geom: geo_types::Geometry<f64> = geo::Point::new(10.0, 20.0).into();
        let geom: geo_types::Geometry<f64> = ls.to_owned().into();

        fgb.add_feature_geom(geom, |feat| {
            feat.property(0, "fid", &ColumnValue::Long(43)).unwrap();
            feat.property(1, "name", &ColumnValue::String("South Africa"))
                .unwrap();
        })
        .ok();
    }
    let mut file = BufWriter::new(std::fs::File::create(filename)?);
    fgb.write(&mut file)?;

    Ok(())
}

fn main() {
    let linestrings: Vec<LineString<f64>> = vec![
        LineString::from(vec![(0., 0.), (1., 1.), (1., 0.), (0., 0.)]),
        LineString::from(vec![(0., 0.), (2., 3.)]),
    ];
    write_fgb_file("test_linestrings.fgb".to_string(), &linestrings)
        .expect("failed to write fgb file");
    println!("done!");
}
