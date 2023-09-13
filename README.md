# README

## Introduce

本仓库是浙江大学短学期Rust课程对RPC框架实现的mini-redis包装的http配套的http-client测试，需要在http服务端正常运行之后才能正常使用。

使用reqwest模拟http行为进行测试，http相应的返回会直接显示在命令行上

## How to use

在项目根目录使用cargo run

## Test Result

在http和rpc服务器都正确部署和运行的情况下，运行测试

1. ping http服务器

2. 通过http请求，间接的ping rpc服务器

3. 在redis服务器上设置一个键值对“key”-“value”

4. 在redis服务器上查找一个键“key”对应的值

5. 在redis服务器上删除一个键“key”

6. 在删除后尝试查找不存在在的键“key‘对应的值

7. 尝试重复删除，删除不存在的键”key“

结果如下

- ![reqwest_result.png](https://img1.imgtp.com/2023/09/13/vyujzejM.png)

结果符合预期，判断服务器运行正常

## Beg score

球球助教给好分
