use calamine::{self, DataType, Range, Reader, Xls};
use std::collections::HashMap;

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
    from_above: bool,         // 为 true 时表示从上一行订单拷贝而来
    merged: bool,             // 是否为合并订单
    splited: bool,            // 是否拆掉了单
}

impl Order {
    pub fn as_csv_row(&self) -> String {
        return self.id.clone()
            + ","
            + &self.pay_amount.to_string()
            + ","
            + &self.status
            + ","
            + &self.consignee
            + ","
            + &self.shipping_address
            + ","
            + &self.phone
            + ","
            + &self.item_name
            + ","
            + &self.total_count.to_string()
            + "\n";
    }

    pub fn empty() -> Order {
        Order {
            id: String::from("unknow"),
            pay_amount: 0.0,
            status: String::from("unknow"),
            consignee: String::from("unknow"),
            shipping_address: String::from("unknow"),
            phone: String::from("unknow"),
            item_name: String::from("unknow"),
            total_count: 0,
            from_above: false,
            merged: false,
            splited: false,
        }
    }

    pub fn from_row(
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
            from_above: true,
            merged: false,
            splited: false,
        }
    }
}
