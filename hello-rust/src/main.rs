fn main() {
    println!("Hello, world!");
    println!("I'm a Rustacean!");

    let x = 5 + /* 90 + */ 5;

    println!("{}", x);
    println!("{} {} days", 31, 32);

    println!("{0}, this is {1}, {0}, this is {1}", "Hello", "This is mars");

    // Named arguments

    println!("{subject} {verb} {object}", 
              object="the lazy dog",
              subject="the quick brown fox",
              verb="jumps over");

              // Special formatting after a `:`

              println!("Base 10:       {}", 648290);
              println!("Base 2 (Binary):    {:b}",  648290);
              println!("Base 8 (Octal):   {:o}",  648290);
              println!("Base 16 (Hex): {:x}",  648290);
              println!("Base 16 (Hex): {:X}",  648290);

              
}
