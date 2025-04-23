use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let body = reqwest::get("https://www.baidu.com").await?.text().await?;

    println!("body:{}", body);
    println!("hello world");
    worldlib::print_sth("this is from hello".to_string());
    let result = worldlib::div(10., 5.).unwrap();
    println!("div result:{}", result);
    Ok(())
}
#[allow(dead_code)]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let r = add(1, 2);
        assert_eq!(r, 3);
    }
}
