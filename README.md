# rust_lab

## 特点

### 性能

- 无运行时
- 无GC
- 零开销抽象

### 可靠

- 内存安全
    - 所有权
    - 无空引用
    - 生命周期
    - 不使用GC
- 线程安全
    - 无畏并发

### 开发效率

- 社区活跃，开源库丰富
    - 文档注释
- 构建工具、包管理器易用且功能丰富（引包、编译、测试、打包、发布）
- IDE
    - Rust官方提供VSCode插件
    - JetBrains提供IDEA内的插件
- 测试
    - 单元测试
    - 集成测试
    - 文档注释测试
    - 覆盖率统计

## 所有权

- 定义
    - 每一个值都有一个称为所有者的变量
    - 一个值只能有一个所有者
    - 当所有者超出范围时，该值会被`drop`

## 引用

- 引用规则
    - 任何时候，只能有一个可变引用，或者只有任意数量的不可变引用
    - 引用总是有效的
- 生命周期

## 线程安全

## 零开销抽象

## Trait

## 异步编程

## 宏

## 范型