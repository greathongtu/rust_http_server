# 简单的Rust HTTP服务器

此服务器可以解析HTTP请求并返回HTTP响应。它由以下几个部分组成:
- main.rs - 调用`server`并启动服务器
- server.rs - 启动TCP监听器,接受连接并调用`router`
- router.rs - 根据请求路径调用正确的`handler`
- handlers.rs - 不同的请求处理程序返回 HTTP 响应
要运行服务器:

```bash
cargo run -p httpserver  
``` 

服务器将在localhost:3000上监听。
你可以向它发出如下请求: <br>
http://localhost:3000/ <br>
http://localhost:3000/health<br>
http://localhost:3000/api/shipping/orders<br>


访问别的路径导向404页面。

此服务器是一个学习Rust如何处理HTTP请求的基本示例。还有很多可以改进和扩展的地方!
