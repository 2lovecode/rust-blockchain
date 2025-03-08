# 产品方案
### 目标
- 用Rust实现简单的区块链Demo
### 核心功能
1. 区块链初始化（创建创世区块）
2. 新区块生成与添加
3. 基础数据验证
4. 区块链数据展示

### 功能说明
- 创世区块：硬编码实现区块链初始化
- 添加区块：基于工作量证明机制（简单版）生成新区块
- 数据存储：内存存储区块链数据
- 哈希验证：SHA-256哈希链式验证



# 技术方案
### 1. 数据结构：
   - Block结构体：index/timestamp/data/prev_hash/hash/nonce
   - Blockchain结构体：Vec<Block>存储链式结构

### 2. 核心算法：
   - 工作量证明：前导零验证（简化版）
   - 哈希计算：SHA-256加密
   - 时间戳：UTC时间戳生成

### 3. 依赖库：
   - sha2: 哈希计算
   - chrono: 时间处理
   - serde: 数据序列化

# 项目说明
### 运行指南
1. 安装Rust环境
2. 添加依赖：sha2 = "0.10.8", chrono = "0.4.40", serde = { version = "1.0.218", features = ["derive"] }
3. 编译运行：cargo run

### 预期输出
```
Index: 0 (Genesis Block)
Timestamp: 时间戳
Data: 数据
Prev Hash: 前一个区块的哈希值
Hash: 00a7c3d2... (前导包含两个零)
Nonce: 根据计算次数不同而变化
```


### 后续优化
1. 添加P2P网络通信
2. 实现交易系统
3. 增加Merkle树结构
4. 完善共识算法