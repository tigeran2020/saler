use calamine::{self, DataType, Range, RangeDeserializerBuilder, Reader, Xls};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug)]
pub struct Order {
    id: String,               // 订单编号
    pay_amount: f64,          // 实付款(元)
    status: String,           // 订单状态
    consignee: String,        // 收货人
    shipping_address: String, // 收货地址
    phone: String,            // 联系手机
    item_name: String,        // 货品标题
    total_count: i64,         // 数量
}

impl Order {
    fn empty() -> Order {
        Order {
            id: String::from("unknow"),
            pay_amount: 0.0,
            status: String::from("unknow"),
            consignee: String::from("unknow"),
            shipping_address: String::from("unknow"),
            phone: String::from("unknow"),
            item_name: String::from("unknow"),
            total_count: 0,
        }
    }

    fn from_row(
        item: &[calamine::DataType],
        title_index: &HashMap<String, usize>,
        last_order: &Order,
    ) -> Order {
        let total_count = get_float(item, title_index, "数量").unwrap_or(0.0) as i64;

        Order {
            id: get_string(item, title_index, "订单编号").unwrap_or(last_order.id.clone()),
            pay_amount: get_float(item, title_index, "实付款(元)").unwrap_or(last_order.pay_amount),
            status: get_string(item, title_index, "订单状态").unwrap_or(last_order.status.clone()),
            consignee: get_string(item, title_index, "收货人姓名")
                .unwrap_or(last_order.consignee.clone()),
            shipping_address: get_string(item, title_index, "收货地址")
                .unwrap_or(last_order.shipping_address.clone()),
            phone: get_string(item, title_index, "联系手机").unwrap_or(last_order.phone.clone()),
            item_name: get_string(item, title_index, "货品标题").unwrap_or(String::from("unknow"))
                + " * "
                + &total_count.to_string(),
            total_count: total_count,
        }
    }
}

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

fn get_string(
    item: &[calamine::DataType],
    title_index: &HashMap<String, usize>,
    title: &str,
) -> Option<String> {
    let index = title_index.get(title)?;
    let v = item[*index].get_string()?;
    Some(String::from(v))
}

fn get_float(
    item: &[calamine::DataType],
    title_index: &HashMap<String, usize>,
    title: &str,
) -> Option<f64> {
    let index = title_index.get(title)?;
    let v = item[*index].get_float()?;
    Some(v)
}

fn get_int(
    item: &[calamine::DataType],
    title_index: &HashMap<String, usize>,
    title: &str,
) -> Option<i64> {
    let index = title_index.get(title)?;
    let v = item[*index].get_int()?;
    Some(v)
}

pub fn work() -> Result<(), String> {
    let orders = read_orders("./testdatas/src.xls")?;
    save_orders("./testdata/dst.xls", &orders).expect("save order failed");
    Ok(())
}

fn read_orders<P>(path: P) -> Result<Vec<Order>, String>
where
    P: AsRef<Path>,
{
    let mut workbook: Xls<_> =
        calamine::open_workbook(path).map_err(|err| format!("can't open file: {}", err))?;
    let range = match workbook
        .worksheet_range_at(0)
        .ok_or(String::from("no sheet"))?
    {
        Ok(range) => range,
        Err(e) => return Err(format!("{}", e)),
    };

    let title_index = &build_index(&range)?;

    println!("build index success: {:?}", title_index);

    let mut res: Vec<Order> = Vec::new();
    range.rows().skip(1).for_each(|item| {
        res.push(Order::from_row(
            item,
            title_index,
            res.last().unwrap_or(&Order::empty()),
        ));
    });

    Ok(res)
}

fn save_orders<P>(path: P, orders: &Vec<Order>) -> Result<(), std::io::Error> {
    let mut file = File::create("./testdatas/dst.csv")?;
    file.write(
        "订单编号,实付款(元),订单状态,收货人姓名,收货地址,联系手机,货品标题,数量\n".as_bytes(),
    )?;
    orders.iter().for_each(|order| {
        let s = order.id.clone()
            + ","
            + &order.pay_amount.to_string()
            + ","
            + &order.status
            + ","
            + &order.consignee
            + ","
            + &order.shipping_address
            + ","
            + &order.phone
            + ","
            + &order.item_name
            + ","
            + &order.total_count.to_string()
            + "\n";
        file.write(s.as_bytes());
    });

    Ok(())
}

//id: "订单编号", pay_amount: 0.0, status: "订单状态", consignee: "收货人姓名", shipping_address: "收货地址", phone: "联系手机", item_name: "货品标题 * 0", total_count: 0

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
