use std::io::{self, Write};
use reqwest;
use serde_json::Value;
use tokio::io::AsyncWriteExt;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    println!("请输入你要下载的东西");
    println!("1: html");
    print!("请选择: ");
    io::stdout().flush()?; // 确保提示信息立即输出

    // 读取用户输入
    io::stdin().read_line(&mut input)?;
    let input = input.trim();

    match input {
        "1" => {
            println!("你选择了下载 html");
            download_html().await?;
        }
        _ => {
            println!("无效的选项，请输入1来下载 html");
        }
    }

    Ok(())
}

async fn download_html() -> Result<(), Box<dyn std::error::Error>> {
    // 请求最新发布信息的接口
    let url = "https://api.github.com/repos/onodera2007/html/releases/latest";
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header("User-Agent", "Rust-release-fetcher")
        .send()
        .await?;
    let latest_release: Value = response.json().await?;
    
    // 输出最新版本、更新内容
    println!("最新版本: {}", latest_release["tag_name"]);
    println!("更新内容: {}", latest_release["body"]);
    // 获取用户指定的下载路径。如果用户直接回车，则使用默认 "gm_form.html"
    print!("请指定你要下载的位置 (例如 C:\\Downloads\\file.zip) [默认: gm_form.html]: ");
    io::stdout().flush()?; // 确保提示信息先输出

    let mut location_input = String::new();
    io::stdin().read_line(&mut location_input)?;
    let location = if location_input.trim().is_empty() {
    String::from("gm_form.html")
    } else {
        location_input.trim().to_string()
    };
    // 从 assets 数组中获取第一个文件的下载链接
    let file_url = match latest_release["assets"].as_array() {
    Some(assets) if !assets.is_empty() => {
        assets[0]["browser_download_url"]
            .as_str()
            .ok_or("无效的下载链接")?
    },
    _ => return Err("没有发布文件".into()),
    };

    // 使用 reqwest 进行流式下载
    let mut response = client
    .get(file_url)
    .header("User-Agent", "Rust-release-fetcher")
    .send()
    .await?;

    // 创建目标文件
    let mut file = tokio::fs::File::create(&location).await?;

    // 流式下载并写入文件，每次写入一个数据块
    while let Some(chunk) = response.chunk().await? {
    file.write_all(&chunk).await?;
    }

    println!("文件已下载到: {}", location);
Ok(())
}