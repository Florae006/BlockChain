# Rust编程区块链项目 BlockChain in Rust

## 使用

启动项目，开启本地服务器

```bash
cargo run
```

本地服务器接口使用：

- 添加交易

```bash
curl -X POST http://127.0.0.1:3030/transaction -H "Content-Type: application/json" -d '{"value":100,"lock_time":0}'
```

- 查看交易池

```bash
curl http://127.0.0.1:3030/pool
```

- 查看区块链区块部分

```bash
curl http://127.0.0.1:3030/blocks
```

- 挖矿

```bash
 curl -X POST http://127.0.0.1:3030/mine
```

## 实验截图

建立交易及交易池状态

![](https://img.dodolalorc.cn/i/2025/03/06/67c9967b9075f.png)

挖矿

![](https://img.dodolalorc.cn/i/2025/03/06/67c99785d7f18.png)

## 项目要求

一：计划

1. 完成概念学习：区块链
2. 完成需求设计：产品方案、技术方案
3. 完成项目实现：Rust 实现，至少输出创世区块

二：要求

1. 结合下面给的和自己搜索到的参考资料、书籍、视频等学习区块链基础知识
2. 结合个人兴趣和所学内容，自己确定一个区块链领域的项目目标（如，实现基本的挖矿）
3. 有了目标，自己学习怎么写产品方案、技术方案、可以多人组队研究
4. 基于产品方案和技术方案实现你的需求，要结合之前讲的数据结构和算法内容

*注意：至少是实现一个简单的区块链，运行并输出创世区块*

三：参考资料
1. 区块链教程 https://liaoxuefeng.com/books/blockchain/introduction/index.html
2. 区块链学习路线 https://zjubca.github.io/roadmap/
3. go 实现的 demo https://github.com/Jeiwan/blockchain_go
4. B 站区块链项目实战 https://www.bilibili.com/video/BV145411t7qp/?vd_source=ca616b8d8161186b30bdd62e4e044e42

![](https://img.dodolalorc.cn/i/2025/03/03/67c5515c5fd95.png)

## 项目实现

区块链交易流程：