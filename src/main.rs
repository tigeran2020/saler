use calamine::{self, RangeDeserializerBuilder, Reader, Xls};
// use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize, Debug)]
// struct RawExcelRow {
//     metric: String,
//     #[serde(deserialize_with = "de_opt_f64")]
//     value: Option<f64>,
// }

// // Convert value cell to Some(f64) if float or int, else None
// fn de_opt_f64<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
// where
//     D: serde::Deserializer<'de>,
// {
//     let data_type = calamine::DataType::deserialize(deserializer);
//     match data_type {
//         Ok(calamine::DataType::Error(_)) => Ok(None),
//         Ok(calamine::DataType::Float(f)) => Ok(Some(f)),
//         Ok(calamine::DataType::Int(i)) => Ok(Some(i as f64)),
//         _ => Ok(None),
//     }
// }

fn main() -> Result<(), String> {
    qlion::work()?;
    Ok(())

    // TODO: 支持多种格式
    // let mut workbook: Xls<_> = calamine::open_workbook("./testdatas/src.xls")
    //     .map_err(|err| format!("can't open file: {}", err))?;
    // let range = match workbook
    //     .worksheet_range_at(0)
    //     .ok_or(String::from("no sheet"))?
    // {
    //     Ok(range) => range,
    //     Err(e) => return Err(format!("{}", e)),
    // };

    // range.rows().for_each(|row| println!("row: {:?}", row));

    // let total_cells = range.get_size().0 * range.get_size().1;
    // println!("total_cells: {}", total_cells);

    // Ok(())

    // Read whole worksheet data and provide some statistics
    // if let Some(Ok(range)) = workbook.worksheet_range("Sheet1") {
    //     let total_cells = range.get_size().0 * range.get_size().1;
    //     let non_empty_cells: usize = range.used_cells().count();
    //     println!(
    //         "Found {} cells in 'Sheet1', including {} non empty cells",
    //         total_cells, non_empty_cells
    //     );
    //     // alternatively, we can manually filter rows
    //     assert_eq!(
    //         non_empty_cells,
    //         range
    //             .rows()
    //             .flat_map(|r| r.iter().filter(|&c| c != &DataType::Empty))
    //             .count()
    //     );
    // }
}
