fn main() {
    println!("=== Concert Ticket ===");

    let city = "London";
    let venue = "Barbican Centre";
    println!("Venue: {1}, {0}", city, venue);

    let artist = "London Symphony Orchestra";
    println!("Artist: {artist}");

    let price = 34.5;
    println!("Price: £{price:.2}");

    let seat = "A12";
    println!("Seat: {seat:>10}");

    let footer = format!("Tonight: {artist} at {venue}, {city}");
    println!("{footer:!<67}");

    eprintln!("ticket printed successfully");
}
