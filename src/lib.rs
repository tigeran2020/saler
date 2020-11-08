use calamine::{self, DataType, Range, Reader};
use chrono::Local;
use simple_excel_writer::Workbook;
use std::{collections::HashMap, env, path::Path};

mod opr;
mod order;
use order::Order;

fn build_index(range: &Range<DataType>) -> Result<HashMap<String, usize>, String> {
    let mut title_index: HashMap<String, usize> = HashMap::new();
    let first_row = range
        .rows()
        .nth(0)
        .ok_or(String::from("first sheet can not be empty"))?;
    first_row.iter().enumerate().for_each(|(i, title)| {
        if let Some(title) = title.get_string() {
            title_index.insert(String::from(title), i);
        }
    });
    Ok(title_index)
}

struct Config {
    src_path: String,
    item_no: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, String> {
        args.next();

        let src_path = match args.next() {
            Some(arg) => arg,
            None => return Err(String::from("Didn't get a src_path")),
        };

        let item_no = match args.next() {
            Some(arg) => arg,
            None => return Err(String::from("Didn't get a item_no")),
        };

        Ok(Config { src_path, item_no })
    }
}

fn generate_dst_path(item_no: &String, total_count: i64) -> String {
    return format!(
        "{}{} 总数{}.xlsx",
        Local::now().format("%Y%m%d"),
        item_no,
        total_count
    );
}

pub fn work() -> Result<(), String> {
    let config = Config::new(env::args())?;

    let orders = read_orders(config.src_path)?;
    println!("read orderes finished, order count: {}", orders.len());
    let orders = opr::remove_repeat(orders);
    println!("order count after removing repeat: {}", orders.len());

    let orders = opr::remove_invalid_item(orders, &config.item_no);
    println!("order count after removing ivalid: {}", orders.len());

    let orders = opr::merge_same_order(orders);
    println!("merge same orderes finished, order count: {}", orders.len(),);

    let mut orders = opr::merge_diff_order(orders);
    println!(
        "merge different orderes finished, order count: {}",
        orders.len(),
    );

    opr::mark_same_phone_order(&mut orders);
    println!("mark same phone order finished");

    let mut count = 0;
    orders.iter().for_each(|order| {
        count += order.total_count;
    });

    save_orders_to_xlsx(&generate_dst_path(&config.item_no, count), &orders)?;
    println!("save order finished");

    Ok(())
}

fn read_orders<P>(path: P) -> Result<Vec<Order>, String>
where
    P: AsRef<Path>,
{
    let mut workbook =
        calamine::open_workbook_auto(path).map_err(|err| format!("can't open file: {}", err))?;
    let range = match workbook
        .worksheet_range_at(0)
        .ok_or(String::from("no sheet"))?
    {
        Ok(range) => range,
        Err(e) => return Err(format!("{}", e)),
    };

    let title_index = &build_index(&range)?;

    let mut res: Vec<Order> = Vec::new();
    range.rows().skip(1).enumerate().for_each(|(i, item)| {
        res.push(Order::from_row(
            item,
            title_index,
            res.last().unwrap_or(&Order::empty()),
            (i + 1) as u32,
        ));
    });

    Ok(res)
}

// save_orders_to_xlsx 保存订单到 xlsx 文件
fn save_orders_to_xlsx(path: &str, orders: &Vec<Order>) -> Result<(), String> {
    let mut wb = Workbook::create(path);
    let mut sheet = wb.create_sheet("default");

    wb.write_sheet(&mut sheet, |sheet_writer| {
        let sw = sheet_writer;
        sw.append_row(Order::excel_title_row())?;
        for order in orders.iter() {
            sw.append_row(order.as_excel_row())?;
        }
        Ok(())
    })
    .map_err(|err| format!("write order failed: {}", err))?;

    wb.close()
        .map(|_| ())
        .map_err(|err| format!("close dst file failed: {}", err))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_build_index() {
        let mut range = Range::<DataType>::new((0, 0), (5, 2));
        range.set_value((0, 1), DataType::String(String::from("hello")));
        range.set_value((0, 2), DataType::String(String::from("world")));

        let title_index = build_index(&range).unwrap();
        assert_eq!(title_index.len(), 2);
        assert_eq!(title_index.get("hello").unwrap(), &1);
        assert_eq!(title_index.get("world").unwrap(), &2);
    }

    #[test]
    fn test_read_orders() {
        read_orders("./testdatas/src.xls").expect("read orders failed");
    }
}
