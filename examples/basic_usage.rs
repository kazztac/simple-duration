use simple_duration::Duration;

fn main() {
    println!("=== simple_duration Usage Example ===\n");

    // Examples of various constructors
    println!("1. Constructors:");
    let d1 = Duration::from_seconds(3661);
    let d2 = Duration::from_minutes(90);
    let d3 = Duration::from_hours(2);
    let d4 = Duration::from_hms(1, 30, 45);

    println!("  from_seconds(3661): {}", d1.format());
    println!("  from_minutes(90):   {}", d2.format());
    println!("  from_hours(2):      {}", d3.format());
    println!("  from_hms(1,30,45):  {}", d4.format());

    // Example: get total amounts in each unit
    println!("\n2. Get total amounts (as_* methods):");
    let duration = Duration::from_hms(2, 15, 30); // 2 hours 15 minutes 30 seconds
    println!("  Duration: {}", duration.format());
    println!("  as_seconds(): {} seconds", duration.as_seconds());
    println!("  as_minutes(): {} minutes", duration.as_minutes());
    println!("  as_hours():   {} hours", duration.as_hours());

    // Example: get each component
    println!("\n3. Get each component (h:m:s format):");
    println!("  seconds_part(): {} (seconds part)", duration.seconds_part());
    println!("  minutes_part():      {} (minutes part)", duration.minutes_part());
    println!("  hours_part():        {} (hours part)", duration.hours_part());

    // Example: parse from string
    println!("\n4. Parse from string:");
    match Duration::parse("12:34:56") {
        Ok(d) => println!("  \"12:34:56\" -> {} ({} seconds)", d.format(), d.as_seconds()),
        Err(e) => println!("  Error: {:?}", e),
    }

    // Example: arithmetic
    println!("\n5. Arithmetic with Duration:");
    let d_a = Duration::from_minutes(45);
    let d_b = Duration::from_minutes(30);
    println!("  {} + {} = {}", d_a.format(), d_b.format(), (d_a + d_b).format());
    println!("  {} - {} = {}", d_a.format(), d_b.format(), (d_a - d_b).format());

    // Practical example
    println!("\n6. Practical example:");
    
    // Calculate total work time
    let morning_work = Duration::from_hms(4, 0, 0);   // 4 hours
    let afternoon_work = Duration::from_hms(3, 30, 0); // 3 hours 30 minutes
    let total_work = morning_work + afternoon_work;
    
    println!("  Morning work: {}", morning_work.format());
    println!("  Afternoon work: {}", afternoon_work.format());
    println!("  Total work time: {} ({} minutes)", total_work.format(), total_work.as_minutes());
    
    // Calculate remaining time to target
    let target_hours = Duration::from_hours(8); // 8 hour target
    let remaining = target_hours - total_work;
    println!("  Remaining to target: {}", remaining.format());

    println!("\n=== End ===");
}
