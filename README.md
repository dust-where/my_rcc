
### 关于

本文借鉴于[github](https://github.com/shioyama18/rcc)并对其进行了说明

我认为原文的汇编代码写的有一定的问题，所以进行了部分修改，但不保证正确

### 如何使用

```
$ cargo run test.c > test.s
$ gcc -o test test.s && ./test
$ echo $?
```

第一行为要编译的程序路径和编译结果存储路径，
第二行为gcc运行x86
第三行为结果

### About

This article draws on [github](https://github.com/shioyama18/rcc) 



**这是我的大学第一个可以称为项目的东西，如果夭折了，那这就是大学的最后一个项目，证明我也不过如此**

一个简单的基于rust的c编译器，参考lcc。
