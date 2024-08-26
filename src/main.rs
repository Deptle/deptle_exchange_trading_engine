#[derive(Debug)]
enum OrderSide {
    Buy,
    Sell
}

#[derive(Debug)]
struct Order {
    order_id: u64,
    price: f64,
    symbol: String,
    quantity: f64,
    side: OrderSide,
    timestamp: u64
}

#[derive(Debug)]
struct OrderBook {
    bids: Vec<Order>,
    asks: Vec<Order>
}

impl OrderBook {
    fn new() -> Self {
        OrderBook {
            bids: Vec::new(),
            asks: Vec::new()
        } 
    }

    fn add_order(&mut self, order: Order) {
        match order.side {
            OrderSide::Buy => self.bids.push(order),
            OrderSide::Sell => self.asks.push(order)
        }
    }
}

fn main() {
    let mut orderbook = OrderBook::new();

    let order = Order {
        order_id: 1,
        symbol: String::from("BTCUSD"),
        price: 35000.00,
        quantity: 1.5,
        side: OrderSide::Buy,
        timestamp: 1627543200
    };

    orderbook.add_order(order);

    println!("Orderbook: {:?}", orderbook);
}
