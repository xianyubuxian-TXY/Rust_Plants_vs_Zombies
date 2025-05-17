# README

### 1.项目介绍

##### 该项目用rust语言实现了一个简易版的"Plants vs Zombies"，分为两种模式：冒险模式、困难模式,冒险模式相对较为简单,每个模式分为3个关卡，僵尸数量和种类会随着关卡数的增加而增加。

##### 

### 2.特性

* 🪴 **植物系统**: 向日葵、豌豆射手、坚果墙基础植物
* 🧟 **僵尸系统**: 普通僵尸、路障僵尸、撑杆僵尸敌人类型
* ☀️ **资源系统**: 阳光收集机制
* 🎮 **游戏循环**: 基于 `ggez`​ 引擎的游戏主循环
* 🎵 **音效支持**: 使用 `rodio`​ 播放游戏音效

‍

### 3.控制方式

* ##### 鼠标点击: 选择/放置植物
* ##### 游戏开始后在左上角会有4个按钮，分别为暂停、运行、重玩、返回上一页，其中暂停和启动也可以通过按Esc实现。

‍

### 4.库环境依赖

* **ggez**：2D 游戏框架，用于窗口与渲染
* **image**：图片加载与处理
* **glam**：数学库，用于向量、矩阵等运算
* **rand**：随机数生成，用于僵尸出现等机制
* **async-std** & **crossbeam**：多线程与并发支持
* **tokio**：异步运行时（全功能特性）
* **rodio**：音频播放
* **lazy_static**：延迟初始化静态变量

‍

### 4.运行要求

* ##### Rust 1.65+
* ##### Cargo

‍

### 5.运行与发布

* #### 构建项目

  * ##### cargo build
* #### 运行项目

  * ##### cargo run
* ##### 发布项目

  * ##### cargo release
  * ##### 注：发布后，需手动将项目资源文件夹asserts复制到release目录

‍

‍

### 6.目录结构描述
|-- Cargo.lock
|-- Cargo.toml
|-- Rust\263\314\320\362\311\350\274\306\323\357\321\324\277\316\263\314\317\356\304\277--\326\262\316\357\264\363\325\275\275\251\312\254\323\316\317\267.
|-- assets
|   |-- audio
|   `-- images
|       |-- background
|       |-- bullets
|       |-- buttons
|       |-- cards
|       |-- cars
|       |-- plants
|       |-- sunshine
|       `-- zm
`-- src
    |-- entities
    |   |-- bullet.rs
    |   |-- button.rs
    |   |-- car.rs
    |   |-- card.rs
    |   |-- grass.rs
    |   |-- mod.rs
    |   |-- my_enum.rs
    |   |-- plant.rs
    |   |-- sunshine.rs
    |   `-- zombie.rs
    |-- entities_managers
    |   |-- background_manager.rs
    |   |-- buttons_manager.rs
    |   |-- cards_manager.rs
    |   |-- map_manager.rs
    |   |-- mod.rs
    |   |-- sunshines_manager.rs
    |   `-- zombie_manager.rs
    |-- game.rs
    |-- main.rs
    |-- my_trait.rs
    |-- threads
    |   |-- audio_thread.rs
    |   `-- mod.rs
    `-- tools.rs

‍

‍
