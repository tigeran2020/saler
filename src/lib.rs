use calamine::{self, DataType, Range, Reader, Xls};
use std::{collections::HashMap, env, fs::File, io::prelude::*, path::Path};

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
    dst_path: String,
    item_no: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, String> {
        args.next();

        let src_path = match args.next() {
            Some(arg) => arg,
            None => return Err(String::from("Didn't get a src_path")),
        };

        let dst_path = match args.next() {
            Some(arg) => arg,
            None => return Err(String::from("Didn't get a dst_path")),
        };

        let item_no = match args.next() {
            Some(arg) => arg,
            None => return Err(String::from("Didn't get a item_no")),
        };

        Ok(Config {
            src_path,
            dst_path,
            item_no,
        })
    }
}

pub fn work() -> Result<(), String> {
    let config = Config::new(env::args())?;

    let orders = read_orders(config.src_path)?;
    println!("read orderes finished, order count: {}", orders.len());
    let orders = opr::remove_repeat(orders);
    println!("order count after removing repeat: {}", orders.len());

    let orders = opr::remove_invalid_item(orders, config.item_no);
    println!("order count after removing ivalid: {}", orders.len());

    let orders = opr::merge_same_order(orders);
    println!("merge same orderes finished, order count: {}", orders.len());

    let orders = opr::merge_diff_order(orders);
    println!(
        "merge different orderes finished, order count: {}",
        orders.len()
    );

    save_orders(config.dst_path, &orders)?;
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

fn save_orders<P>(path: P, orders: &Vec<Order>) -> Result<(), String>
where
    P: AsRef<Path>,
{
    let mut file = File::create(path).map_err(|err| format!("can't create dst file: {}", err))?;
    file.write(
        "订单编号,合入订单,是否拆分,实付款(元),订单状态,收货人姓名,收货地址,联系手机,货品标题,数量\n".as_bytes(),
    )
    .map_err(|err| format!("write header failed: {}", err))?;
    for order in orders.iter() {
        file.write(order.as_csv_row().as_bytes())
            .map_err(|err| format!("write order failed: {}", err))?;
    }

    Ok(())
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
