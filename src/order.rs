use calamine::{self, DataType};
use simple_excel_writer::{self as excel, sheet::Row};
use std::collections::HashMap;

fn get_string(
    item: &[DataType],
    title_index: &HashMap<String, usize>,
    title: &str,
) -> Option<String> {
    let index = title_index.get(title)?;
    let v = item[*index].get_string()?;
    Some(String::from(v))
}

fn get_float(item: &[DataType], title_index: &HashMap<String, usize>, title: &str) -> Option<f64> {
    let index = title_index.get(title)?;
    let v = item[*index].get_float()?;
    Some(v)
}

#[derive(Debug)]
pub struct Order {
    pub id: String,                 // 订单编号
    pub total_price: f64,           // 总价
    pay_amount: f64,                // 实付款(元)
    pub status: String,             // 订单状态
    pub consignee: String,          // 收货人
    pub shipping_address: String,   // 收货地址
    pub phone: String,              // 联系手机
    pub telephone: String,          // 联系电话
    pub item_name: String,          // 货品标题
    pub total_count: i64,           // 数量
    pub price: f64,                 // 单价
    pub leave_msg: String,          // 买家留言
    pub group: u32,                 // 所属组，即该订单的第一个商品的位置
    pub merged: Vec<String>,        // 合并了哪些订单
    pub splited: bool,              // 是否拆掉了单
    pub has_same_phone_order: bool, // 是否存在同手机号的其它订单
}

impl Order {
    // 返回货号
    pub fn item_no(&self) -> String {
        match self.item_name.find(" ") {
            None => String::from(""),
            Some(i) => String::from(&self.item_name[..i]),
        }
    }

    pub fn excel_title_row() -> Row {
        excel::row![
            "订单编号",
            "手动处理",
            "实付款(元)",
            "订单状态",
            "收货人姓名",
            "收货地址",
            "联系手机",
            "货品标题",
            "数量",
            "买家留言"
        ]
    }

    pub fn as_excel_row(&self) -> Row {
        let mut flag = String::new();
        if self.has_same_phone_order {
            flag += "未合";
        }
        if self.merged.len() > 0 {
            flag += "已合";
        }
        if self.splited {
            flag += "已拆";
        }

        let mut phone = &self.phone;
        if phone.len() == 0 {
            phone = &self.telephone;
        }

        let mut money = self.pay_amount;
        // money 合并或拆分就按照数量*单价求和来
        if self.merged.len() > 0 || self.splited {
            money = self.total_price;
        }

        excel::row![
            self.id.clone(),
            flag,
            money,
            self.status.clone(),
            self.consignee.clone(),
            self.shipping_address.clone(),
            phone.clone(),
            self.item_name.clone(),
            self.total_count as f64,
            self.leave_msg.clone()
        ]
    }

    pub fn empty() -> Order {
        Order {
            id: String::from("unknow"),
            total_price: 0.0,
            pay_amount: 0.0,
            status: String::from("unknow"),
            consignee: String::from("unknow"),
            shipping_address: String::from("unknow"),
            phone: String::from(""),
            telephone: "".to_string(),
            item_name: String::from("unknow"),
            total_count: 0,
            price: 0.0,
            leave_msg: String::from(""),
            group: 0,
            merged: vec![],
            splited: false,
            has_same_phone_order: false,
        }
    }

    pub fn from_row(
        item: &[calamine::DataType],
        title_index: &HashMap<String, usize>,
        last_order: &Order,
        row_index: u32,
    ) -> Order {
        let total_count = get_float(item, title_index, "数量").unwrap_or(0.0) as i64;
        let price = get_float(item, title_index, "单价(元)").unwrap_or(0.0);
        let mut group = row_index;
        // same_group 表示和上个订单是否为同一组
        let mut same_group = false;
        let id = get_string(item, title_index, "订单编号").unwrap_or_else(|| {
            same_group = true;
            group = last_order.group;
            last_order.id.clone()
        });

        let order = Order {
            id,
            total_price: total_count as f64 * price,
            pay_amount: get_float(item, title_index, "实付款(元)").unwrap_or(last_order.pay_amount),
            status: get_string(item, title_index, "订单状态").unwrap_or(last_order.status.clone()),
            consignee: get_string(item, title_index, "收货人姓名")
                .unwrap_or(last_order.consignee.clone()),
            shipping_address: get_string(item, title_index, "收货地址")
                .unwrap_or(last_order.shipping_address.clone()),
            phone: if same_group {
                last_order.phone.clone()
            } else {
                get_string(item, title_index, "联系手机").unwrap_or("".to_string())
            },
            telephone: if same_group {
                last_order.telephone.clone()
            } else {
                get_string(item, title_index, "联系电话").unwrap_or("".to_string())
            },

            item_name: get_string(item, title_index, "货品标题").unwrap_or(String::from("unknow"))
                + " * "
                + &total_count.to_string(),
            total_count,
            price,
            leave_msg: get_string(item, title_index, "买家留言").unwrap_or(String::from("")),
            group,
            merged: vec![],
            splited: false,
            has_same_phone_order: false,
        };

        order
    }

    pub fn merge(&mut self, other: &Order) {
        self.item_name += &("\n".to_owned() + &other.item_name);
        self.total_count += other.total_count;
        self.total_price += other.total_price;
        if self.leave_msg.len() > 0 && other.leave_msg.len() > 0 {
            self.leave_msg += &("\n".to_owned() + &other.leave_msg);
        } else {
            self.leave_msg += &other.leave_msg;
        }
    }
    pub fn merge_diff(&mut self, other: &Order) {
        self.merge(other);
        self.merged.push(other.id.clone());
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test_item_no() {
        let mut order = Order::empty();
        order.item_name = String::from("AJJ0 helloworld");
        assert_eq!(order.item_no(), String::from("AJJ0"));
    }

    #[test]
    pub fn test_empty_item_no() {
        let mut order = Order::empty();
        order.item_name = String::from("AJJ0helloworld");
        assert_eq!(order.item_no().len(), 0);
    }
}
