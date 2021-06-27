# DyscheOS-utils

#### 介绍
It provides utility tools for DyscheOS, including management tools, scripts, user-guide and kernel modules.

#### 软件架构
软件架构说明


#### 安装教程

1.  xxxx
2.  xxxx
3.  xxxx

#### 使用说明

1.  xxxx
2.  xxxx
3.  xxxx

#### 参与贡献

1.  Fork 本仓库
2.  新建 Feat_xxx 分支
3.  提交代码
4.  新建 Pull Request


#### 特技

1.  使用 Readme\_XXX.md 来支持不同的语言，例如 Readme\_en.md, Readme\_zh.md
2.  Gitee 官方博客 [blog.gitee.com](https://blog.gitee.com)
3.  你可以 [https://gitee.com/explore](https://gitee.com/explore) 这个地址来了解 Gitee 上的优秀开源项目
4.  [GVP](https://gitee.com/gvp) 全称是 Gitee 最有价值开源项目，是综合评定出的优秀开源项目
5.  Gitee 官方提供的使用手册 [https://gitee.com/help](https://gitee.com/help)
6.  Gitee 封面人物是一档用来展示 Gitee 会员风采的栏目 [https://gitee.com/gitee-stars/](https://gitee.com/gitee-stars/)


####CRIU-RISC-V HOWTO
######CRIU架构相关代码

criu需要针对每个架构生成对应的系统调用列表，处理当前架构的syscall，以aarch64为例，在compel/arch/aarch64/plugins/std/syscalls目录下。Makefile.syscalls根据syscall.def、syscal-common.S、syscall-aux.S生成系统调用表：

syscall.def： gen-sys***.pl根据syscall.def文件生成系统调用表

syscall-common.S里面定义了通用syscall的函数实现，可以借鉴

syscall-aux.S为当前架构不支持syscall的替代实现，对于simple_loop可以暂时不考虑。后续需要分析

criu跟踪目标进程后要通过compel_syscall执行系统调用，此时设置的寄存器与架构相关，code_syscall也不同。比如x86下就是int $0x80,aarch64下就是SVC #0（需要找到这几个指令对应的二进制码形式，通过反汇编或查找手册应该可以找到）
以aarch64为例，执行compel_syscall的目录在compel/arch/aarch64/src/lib/infect.c中，需要将对应参数放到指定寄存器中。

* code blobs

code blob也是criu中较为核心的功能，在checkpoint和restore过程中都会涉及；比如checkpoint阶段使用paraside寄生代码侵入tracee进程获取进程的相关信息（VMA、进程fd等）。restore阶段使用blobs设置vma，然后从指定代码段CS：IP处继续执行。

parasite-code的入口函数是架构相关，每种架构都有自己对应实现，aarch64就在compel/arch/aarch64/plugins/std/parasite-head.S

compel/arch/aarch64/scripts/compel-pack.lds.S是链接规则文件，定义如果通过编译的*.a链接生成Code-blobs.h文件内容，可以参考aarch64和x86实现。

* elf文件解析

criu通过compel-host-bin完成elf文件解析，生成对应blob.h文件。这部分内容为架构相关；

aarch64下可参考文件compel/arch/aarch64/src/lib/handle-elf.c

* cpu信息保存恢复

这部分内容当前只有x86实现较好，aach64下未实现，可参考实现，全部return 0.

```
criu/include/cpu.h
compel/arch/{arm/aarch64/x86}/src/lib/include/uapi/asm/cpu.h
criu/arch/aarch64/cpu.c
compel/arch/aarch64/src/lib/cpu.c
```

* 进程信息

部分进程信息为架构相关，如：

* compel_task_size 计算 task_size

* arch_can_dump_task 检查 task 是否可被 dump

* get_task_regs 获取进程寄存器内容

* parasite_setup_regs 设置 stack、ip 信息

以及ptrace信息：

* ptrace_get_regs/ptrace_set_regs 获取进程 regs 信息

* ptrace_set_breakpoint/ptrace_flush_breakpoints 设置进程 breakpoint

===============================

risc-v汇编指令学习可以参考这个文档：https://github.com/riscv/riscv-asm-manual/blob/master/riscv-asm.md

===================================


criu社区： https://github.com/checkpoint-restore/criu
