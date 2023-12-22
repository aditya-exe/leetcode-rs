pub fn buy_choco(mut prices: Vec<i32>, money: i32) -> i32 {
    let (mut mn1, mut mn2) = (i32::MAX, i32::MAX);

    prices.iter().for_each(|x| {
        if x < &mn1 {
            mn2 = mn1;
            mn1 = *x;
        } else if x < &mn2 {
            mn2 = *x;
        }
    });

    let price = mn1 + mn2;

    if price > money {
        return money;
    } else {
        return money - price;
    }
}
