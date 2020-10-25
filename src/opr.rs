use crate::order::Order;
use std::collections::HashMap;

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
    orders
        .into_iter()
        .filter(|order| order.item_no() == item_no)
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_remove_invalid_item() {
        let mut orders = Vec::new();

        let mut order = Order::empty();
        order.id = String::from("order-1");
        order.item_name = String::from("AJ001 helloworld");
        orders.push(order);
        let mut order = Order::empty();
        order.id = String::from("order-2");
        order.item_name = String::from("AJ002 helloworld");
        orders.push(order);

        orders = remove_invalid_item(orders, String::from("AJ001"));
        assert_eq!(orders.len(), 1);
        assert_eq!(orders[0].id, "order-1");
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
