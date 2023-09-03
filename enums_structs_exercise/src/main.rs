#[derive(Debug)]
enum Ticket {
    Backstage(f32,String),
    Vip(f32,String),
    Standard(f32)
}
fn main() {
    let tickets = vec![
        Ticket::Backstage(35.00, String::from("Paul")),
        Ticket::Vip(135.00, String::from("Saul")),
        Ticket::Standard(15.00)
    ];
    for (_,ticket) in tickets.iter().enumerate(){
        match ticket {
            Ticket::Backstage(price,ticket_holder) => {
                println!("This is a Backstage ticket on the name of {} worth ${}",ticket_holder,price)
            },
            Ticket::Vip(price,ticket_holder ) => {
                println!("This is a VIP ticket on the name of {} worth ${}",ticket_holder,price)
            },
            Ticket::Standard(price) => {
                println!("This is a Standard ticket worth ${}",price)
            }
        }
    }
    println!("Hello, world!");
}
