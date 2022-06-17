# 基于实体的代码生成器(目前只适用于JAVA后端)

``` shell
# 输出用法
regn 
# 输出版本
regn -v
# 解析当前目录底下配置，生产代码
rgen -g
# 解析当前目录下的实体文件生成代码
rgen -g ./image_cloud.rge
```

## 模板语法

 借鉴 Jinja2 and Django

## 剩余待办

todo:: ~~解析当前目录配置文件 生成全局数据~~

todo:: 解析命令行参数 并验证参数

todo:: 解析本地.rge 文件生成实体列表

todo:: 连接数据库 生成实体列表
