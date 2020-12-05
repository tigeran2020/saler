use crate::order::Order;
use std::{collections::HashMap, hash::Hash, hash::Hasher};

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
pub fn remove_invalid_item(orders: Vec<Order>, item_no: &String) -> Vec<Order> {
    let mut removed_orders = HashMap::<String, bool>::new();
    let mut orders: Vec<Order> = orders
        .into_iter()
        .filter(|order| {
            let res = order.item_no().to_lowercase() == item_no.to_lowercase();
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

#[derive(Debug)]
struct PhoneAndTele {
    phone: String,     // 联系手机
    telephone: String, // 联系电话
}

impl PartialEq for PhoneAndTele {
    fn eq(&self, other: &Self) -> bool {
        // 手机号都为空时，以联系电话为准
        // 手机号不为空时，以手机号为准
        match self.phone.len() {
            0 => other.phone.len() == 0 && self.telephone == other.telephone,
            _ => self.phone == other.phone,
        }
    }
}

// 只计算 phone 的 hash，已便放入到 HashMap 中
impl Hash for PhoneAndTele {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.phone.hash(state);
    }
}

impl Eq for PhoneAndTele {}

// mark_same_phone_order 为存在同号码的订单加上标记
pub fn mark_same_phone_order(orders: &mut Vec<Order>) {
    let mut phone_map = HashMap::<PhoneAndTele, usize>::new();

    orders.iter().for_each(|order| {
        let count = phone_map
            .entry(PhoneAndTele {
                phone: order.phone.clone(),
                telephone: order.telephone.clone(),
            })
            .or_insert(0);
        *count += 1;
    });

    orders.iter_mut().for_each(|order| {
        if phone_map[&PhoneAndTele {
            phone: order.phone.clone(),
            telephone: order.telephone.clone(),
        }] > 1
        {
            order.has_same_phone_order = true;
        }
    });
}

#[derive(Hash, Debug, PartialEq)]
struct OrderKey {
    consignee: String,        // 收货人
    shipping_address: String, // 收货地址
    status: String,           // 订单状态
    phones: PhoneAndTele,     // 手机号
}

impl Eq for OrderKey {}

pub fn merge_diff_order(orders: Vec<Order>) -> Vec<Order> {
    let mut order_map = HashMap::<OrderKey, usize>::new();
    let mut res_orders: Vec<Order> = Vec::new();

    orders.into_iter().for_each(|order| {
        let key = OrderKey {
            consignee: order.consignee.clone(),
            shipping_address: order.shipping_address.clone(),
            status: order.status.clone(),
            phones: PhoneAndTele {
                phone: order.phone.clone(),
                telephone: order.telephone.clone(),
            },
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
    fn test_order_key_eq() {
        assert_eq!(
            OrderKey {
                consignee: "zhangsan".to_string(),
                shipping_address: "shenzhen".to_string(),
                status: "等待发货".to_string(),
                phones: PhoneAndTele {
                    phone: "123456789".to_string(),
                    telephone: "".to_string(),
                },
            },
            OrderKey {
                consignee: "zhangsan".to_string(),
                shipping_address: "shenzhen".to_string(),
                status: "等待发货".to_string(),
                phones: PhoneAndTele {
                    phone: "123456789".to_string(),
                    telephone: "".to_string(),
                },
            }
        );

        // 手机号不为空，以手机号为准
        assert_eq!(
            OrderKey {
                consignee: "zhangsan".to_string(),
                shipping_address: "shenzhen".to_string(),
                status: "等待发货".to_string(),
                phones: PhoneAndTele {
                    phone: "123456789".to_string(),
                    telephone: "".to_string(),
                },
            },
            OrderKey {
                consignee: "zhangsan".to_string(),
                shipping_address: "shenzhen".to_string(),
                status: "等待发货".to_string(),
                phones: PhoneAndTele {
                    phone: "123456789".to_string(),
                    telephone: "123".to_string(),
                },
            }
        );

        assert_ne!(
            OrderKey {
                consignee: "zhangsan".to_string(),
                shipping_address: "shenzhen".to_string(),
                status: "等待发货".to_string(),
                phones: PhoneAndTele {
                    phone: "1234567890".to_string(),
                    telephone: "".to_string(),
                },
            },
            OrderKey {
                consignee: "zhangsan".to_string(),
                shipping_address: "shenzhen".to_string(),
                status: "等待发货".to_string(),
                phones: PhoneAndTele {
                    phone: "123456789".to_string(),
                    telephone: "123".to_string(),
                },
            }
        );

        // 手机号为空，以 telephone 为准
        assert_eq!(
            OrderKey {
                consignee: "zhangsan".to_string(),
                shipping_address: "shenzhen".to_string(),
                status: "等待发货".to_string(),
                phones: PhoneAndTele {
                    phone: "".to_string(),
                    telephone: "123".to_string(),
                }
            },
            OrderKey {
                consignee: "zhangsan".to_string(),
                shipping_address: "shenzhen".to_string(),
                status: "等待发货".to_string(),
                phones: PhoneAndTele {
                    phone: "".to_string(),
                    telephone: "123".to_string(),
                }
            }
        );

        assert_ne!(
            OrderKey {
                consignee: "zhangsan".to_string(),
                shipping_address: "shenzhen".to_string(),
                status: "等待发货".to_string(),
                phones: PhoneAndTele {
                    phone: "".to_string(),
                    telephone: "1234".to_string(),
                }
            },
            OrderKey {
                consignee: "zhangsan".to_string(),
                shipping_address: "shenzhen".to_string(),
                status: "等待发货".to_string(),
                phones: PhoneAndTele {
                    phone: "".to_string(),
                    telephone: "123".to_string(),
                }
            }
        );
    }

    #[test]
    fn test_mark_same_phone_order() {
        let mut orders = Vec::new();
        let mut order = Order::empty();
        order.id = String::from("order-1");
        order.phone = String::from("123456");
        orders.push(order);

        let mut order = Order::empty();
        order.id = String::from("order-2");
        order.phone = String::from("123456");
        orders.push(order);

        let mut order = Order::empty();
        order.id = String::from("order-3");
        order.phone = String::from("123457");
        orders.push(order);

        // 和 1, 2 相同，忽略 telephone
        let mut order = Order::empty();
        order.id = String::from("order-4");
        order.phone = String::from("123456");
        order.telephone = String::from("123456");
        orders.push(order);

        let mut order = Order::empty();
        order.id = String::from("order-5");
        order.phone = String::from("");
        order.telephone = String::from("123456");
        orders.push(order);

        let mut order = Order::empty();
        order.id = String::from("order-6");
        order.phone = String::from("");
        order.telephone = String::from("123456");
        orders.push(order);

        let mut order = Order::empty();
        order.id = String::from("order-7");
        order.phone = String::from("");
        order.telephone = String::from("1234567");
        orders.push(order);

        mark_same_phone_order(&mut orders);
        assert!(orders[0].has_same_phone_order);
        assert!(orders[1].has_same_phone_order);
        assert!(!orders[2].has_same_phone_order);

        // 和 order-1, order-2 相同，忽略 telephone
        assert!(orders[3].has_same_phone_order);

        // order-5, order-6 相同，是因为 phone 为空， telephone 相同
        // order-7 是因为 telephone 不同
        assert!(orders[4].has_same_phone_order);
        assert!(orders[5].has_same_phone_order);
        assert!(!orders[6].has_same_phone_order);
    }

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
        assert_eq!(orders[0].item_name, "AJ001 helloworld\nAJ003 helloworld");
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
        assert_eq!(orders[0].item_name, "AJ001 helloworld\nAJ003 helloworld");
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

        orders = remove_invalid_item(orders, &String::from("aJ001"));
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
