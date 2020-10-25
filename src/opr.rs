use crate::order::Order;
use std::{collections::HashMap, hash::Hash};

// remove_repeat 删除重复的订单
pub fn remove_repeat(orders: Vec<Order>) -> Vec<Order> {
    let mut order_group = HashMap::<String, u32>::new();

    orders
        .into_iter()
        .filter(|order| match order_group.get(&order.id) {
            Some(group) => order.group == *group,
            None => {
                order_group.insert(order.id.clone(), order.group);
                true
            }
        })
        .collect()
}

// remove_invalid_item 移除非条件商品
pub fn remove_invalid_item(orders: Vec<Order>, item_no: String) -> Vec<Order> {
    let mut removed_orders = HashMap::<String, bool>::new();
    let mut orders: Vec<Order> = orders
        .into_iter()
        .filter(|order| {
            let res = order.item_no() == item_no;
            if !res {
                removed_orders.insert(order.id.clone(), true);
            }
            res
        })
        .collect();

    orders.iter_mut().for_each(|order| {
        if removed_orders.get(&order.id).is_some() {
            order.splited = true;
        }
    });

    orders
}

// merge_same_order 合并同id的订单
pub fn merge_same_order(orders: Vec<Order>) -> Vec<Order> {
    let mut order_map = HashMap::<String, usize>::new();
    let mut res_orders: Vec<Order> = Vec::new();

    orders.into_iter().for_each(|order| {
        order_map
            .entry(order.id.clone())
            .and_modify(|i| res_orders[*i].merge(&order))
            .or_insert_with(|| {
                res_orders.push(order);
                res_orders.len() - 1
            });
    });

    res_orders
}

#[derive(PartialEq, Hash)]
struct OrderKey {
    consignee: String,        // 收货人
    shipping_address: String, // 收货地址
    phone: String,            // 联系手机
}

impl Eq for OrderKey {}

pub fn merge_diff_order(orders: Vec<Order>) -> Vec<Order> {
    let mut order_map = HashMap::<OrderKey, usize>::new();
    let mut res_orders: Vec<Order> = Vec::new();

    orders.into_iter().for_each(|order| {
        let key = OrderKey {
            consignee: order.consignee.clone(),
            shipping_address: order.shipping_address.clone(),
            phone: order.phone.clone(),
        };
        order_map
            .entry(key)
            .and_modify(|i| res_orders[*i].merge_diff(&order))
            .or_insert_with(|| {
                res_orders.push(order);
                res_orders.len() - 1
            });
    });

    res_orders
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_merge_diff_order() {
        let mut orders = Vec::new();
        let mut order = Order::empty();
        order.id = String::from("order-1");
        order.consignee = String::from("xiaoming");
        order.shipping_address = String::from("beijing");
        order.phone = String::from("123456789");
        order.item_name = String::from("AJ001 helloworld");
        orders.push(order);
        let mut order = Order::empty();
        order.id = String::from("order-2");
        order.consignee = String::from("xiaoming");
        order.shipping_address = String::from("beijing");
        order.phone = String::from("123456789");
        order.item_name = String::from("AJ003 helloworld");
        orders.push(order);
        let mut order = Order::empty();
        order.id = String::from("order-3");
        order.consignee = String::from("xiaohuang");
        order.shipping_address = String::from("beijing");
        order.phone = String::from("123456789");
        order.item_name = String::from("AJ002 helloworld");
        orders.push(order);

        orders = merge_diff_order(orders);
        assert_eq!(orders.len(), 2);
        assert_eq!(orders[0].id, "order-1");
        assert_eq!(orders[0].item_name, "AJ001 helloworld&&AJ003 helloworld");
        assert_eq!(orders[0].merged, vec![String::from("order-2")]);
        assert_eq!(orders[1].id, "order-3");
    }

    #[test]
    fn test_merge_same_order() {
        let mut orders = Vec::new();
        let mut order = Order::empty();
        order.id = String::from("order-1");
        order.item_name = String::from("AJ001 helloworld");
        orders.push(order);
        let mut order = Order::empty();
        order.id = String::from("order-1");
        order.item_name = String::from("AJ003 helloworld");
        orders.push(order);
        let mut order = Order::empty();
        order.id = String::from("order-2");
        order.item_name = String::from("AJ002 helloworld");
        orders.push(order);

        orders = merge_same_order(orders);
        assert_eq!(orders.len(), 2);
        assert_eq!(orders[0].id, "order-1");
        assert_eq!(orders[0].item_name, "AJ001 helloworld&&AJ003 helloworld");
    }

    #[test]
    fn test_remove_invalid_item() {
        let mut orders = Vec::new();

        let mut order = Order::empty();
        order.id = String::from("order-1");
        order.item_name = String::from("AJ001 helloworld");
        orders.push(order);
        let mut order = Order::empty();
        order.id = String::from("order-1");
        order.item_name = String::from("AJ003 helloworld");
        orders.push(order);
        let mut order = Order::empty();
        order.id = String::from("order-2");
        order.item_name = String::from("AJ002 helloworld");
        orders.push(order);

        orders = remove_invalid_item(orders, String::from("AJ001"));
        assert_eq!(orders.len(), 1);
        assert_eq!(orders[0].id, "order-1");
        assert!(orders[0].splited);
    }

    #[test]
    fn test_remove_repeat_on_diff_order() {
        let mut orders = Vec::new();

        let mut order = Order::empty();
        order.id = String::from("order-1");
        order.group = 1;
        orders.push(order);
        let mut order = Order::empty();
        order.id = String::from("order-2");
        order.group = 2;
        orders.push(order);

        orders = remove_repeat(orders);
        assert_eq!(orders.len(), 2);
    }

    #[test]
    fn test_remove_repeat_on_diff_group() {
        let mut orders = Vec::new();

        let mut order = Order::empty();
        order.id = String::from("order-1");
        order.group = 1;
        orders.push(order);
        let mut order = Order::empty();
        order.id = String::from("order-1");
        order.group = 2;
        orders.push(order);

        orders = remove_repeat(orders);
        assert_eq!(orders.len(), 1);
    }

    #[test]
    fn test_remove_repeat_on_same_group() {
        let mut orders = Vec::new();

        let mut order = Order::empty();
        order.id = String::from("order-1");
        order.group = 1;
        orders.push(order);
        let mut order = Order::empty();
        order.id = String::from("order-1");
        order.group = 1;
        orders.push(order);

        orders = remove_repeat(orders);
        assert_eq!(orders.len(), 2);
    }
}
