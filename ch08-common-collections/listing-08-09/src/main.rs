fn main() {
    // ANCHOR: here
    #[derive(Debug)]  // K_22711 要加这句才能打印debug信息出来,而且要加在枚举定义的前面
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.52),
    ];
    // ANCHOR_END: here

    println!("{:#?}", row); 
}
