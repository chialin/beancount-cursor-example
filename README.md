# Beancount Generator / Beancount 生成器

## English

This project is a WebAssembly-based Beancount file generator. It provides a simple web interface for creating Beancount transactions and exporting them to a file.

### Features

- Create Beancount transactions with date, payee, and narration
- Add multiple postings to a transaction
- Generate Beancount-formatted output
- Export generated transactions to a .beancount file

### Usage

1. Open the `index.html` file in a web browser
2. Click the "Generate Beancount File" button
3. A file named `transaction.beancount` will be downloaded with the generated transaction

### Development

This project uses Rust compiled to WebAssembly. To modify the project:

1. Update the Rust code in `src/lib.rs`
2. Compile the project using `wasm-pack build --target web`
3. The generated JavaScript and WebAssembly files will be in the `pkg` directory
4. Update `index.html` to use the new generated files if necessary

## 中文

這個項目是一個基於 WebAssembly 的 Beancount 文件生成器。它提供了一個簡單的網頁界面，用於創建 Beancount 交易並將其導出為文件。

### 功能

- 創建包含日期、收款人和說明的 Beancount 交易
- 為交易添加多個過帳
- 生成 Beancount 格式的輸出
- 將生成的交易導出為 .beancount 文件

### 使用方法

1. 在網頁瀏覽器中打開 `index.html` 文件
2. 點擊"生成 Beancount 文件"按鈕
3. 一個名為 `transaction.beancount` 的文件將被下載，其中包含生成的交易

### 開發

這個項目使用 Rust 編譯為 WebAssembly。要修改項目：

1. 更新 `src/lib.rs` 中的 Rust 代碼
2. 使用 `wasm-pack build --target web` 編譯項目
3. 生成的 JavaScript 和 WebAssembly 文件將位於 `pkg` 目錄中
4. 如有必要，更新 `index.html` 以使用新生成的文件
