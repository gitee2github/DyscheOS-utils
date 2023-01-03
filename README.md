# DyscheOS-utils
仓库状态设置为关闭的原因: https://gitee.com/openeuler/community/pulls/3792

#### 介绍
本仓库是 Dysche 解决方案用户态工具的仓库。Master 分支可能不会包含最新最全的工具
集， 可查看分支列表获取更全面的及时信息。

#### 软件架构
Dysche 的整体架构来源于 Linux AMP 架构，大体上分为两个部分。一个是 Linux 内核
部分，另一个是用户态部分。本仓库专注于用户态部分。

用户态工具逻辑上分为如下几个部分：

- 对 APP-OS 镜像的加载与校验

- 与 Dysche 内核部分提供的接口进行交互，将 APP-OS 启动起来

- 常驻 Linux 的系统服务，通过与 Dysche 内核模块交互，提供在线功能扩展，如设备
模拟，维护，用户管理操作等。


#### 使用说明

详情请参见每一个子目录中的 README 文件。

#### 参与贡献

1.  Fork 本仓库
2.  新建分支
3.  提交代码
4.  新建 Pull Request

#### 致谢

Thanks.

