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
            .and_modify(|i| res_orders[*i].merge_same(&order))
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
