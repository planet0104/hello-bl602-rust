# BL602 Rust example

BL602 Rust HAL 的简单示例程序。

参考代码：[bl602-rust-example](https://github.com/9names/bl602-rust-example)

参考文章：[Rust on RISC-V BL602: Is It Sunny?](https://lupyuen.github.io/articles/adc)

## 安装环境

获取 bl602 的工具链
```
rustup target add riscv32imac-unknown-none-elf
```

安装 cargo-blflash
```
cargo install cargo-blflash
```

## 编译运行

```
cargo build
```

将 GPIO 8 (D8) 连接到开发板的 3.3V，按一下EN键。 

运行
```
cargo blflash --port COM4
```

断开 GPIO 8 的高电位，按下EN键，使用串口工具链接COM4，可看到输出结果。



