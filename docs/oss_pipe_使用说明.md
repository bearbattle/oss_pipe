# OSS PIPE V 0.2.0

项目地址
<https://github.com/jiashiwen/oss_pipe>

oss_pipe 是rust编写的文件迁移工具，旨在支撑大规模的文件迁移场景。相比java 或 golang 构建的同类型产品，借助rust语言的优势，oss_pipe具备无GC、高并发、部署便利、OOM风险低等优势。

## 主要功能

### transfer 
文件迁移，包括oss 间文件迁移和本地到oss的文件迁移

* 主要功能
  * 全量迁移
  * 存量迁移
  * 增量迁移
  * 断点续传
  * 大文件拆分上传
  * 正则表达式过滤
  * 线程数与上传块大小组合控制带宽

* 存储适配及支持列表
  * 京东云对象存储
  * 阿里云对象存储
  * 腾讯云对象存储
  * 华为云对象存储
  * AWS对象存储
  * Minio
  * 本地

### compare 
文件校验，检查源文件与迁移完成后目标文件的差异

* 主要功能
  * 存在性校验
  * 文件长度校验
  * meta数据校验
  * 过期时间校验
  * 全字节流校验

## Getting Stated

### How to build

* 安装rust编译环境

```rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

* 构建

```shell
apt update
apt install openssl
apt install libssl1.1
apt install libssl-dev
apt install -y pkg-config
```

```shell
git clone https://github.com/jiashiwen/oss_pipe.git
cd oss_pipe
git fetch origin
git checkout -b 0.2.0 origin/0.2.0
cargo build --release
```

### 基本使用

oss_pipe 支持命令执行模式和交互式执行模式。
您可以通过 oss_pipe [subcommand] 执行任务，比如

```shell
oss_pipe parameters provider
```

也可以通过 oss_pipe -i 命里进入交互模式。交互模式支持按 'tab' 键进行子命令提示。

### 定义任务

通过 oss_pipe template 命令生成模板

```shell
oss_pipe template transfer oss2oss /tmp/transfer.yml
```

transfer.yml 文件内容

```yml
type: transfer
task_id: '7143131817338605569'
name: transfer_oss2oss
source:
  provider: ALI
  access_key_id: access_key_id
  secret_access_key: secret_access_key
  endpoint: http://oss-cn-beijing.aliyuncs.com
  region: cn-north-1
  bucket: bucket_name
  prefix: test/samples/
  request_style: VirtualHostedStyle
target:
  provider: JD
  access_key_id: access_key_id
  secret_access_key: secret_access_key
  endpoint: http://s3.cn-north-1.jdcloud-oss.com
  region: cn-north-1
  bucket: bucket_name
  prefix: test/samples_bak/
  request_style: VirtualHostedStyle
attributes:
  objects_transfer_batch: 64
  task_parallelism: 12
  meta_dir: /tmp/meta_dir
  target_exists_skip: false
  start_from_checkpoint: false
  large_file_size: 64m
  multi_part_chunk_size: 8m
  multi_part_chunks_per_batch: 16
  multi_part_parallelism: 8
  multi_part_max_parallelism: 12
  exclude:
  - ^test/t3/.*
  - ^test/t4/.*
  include:
  - ^test/t1/.*
  - ^test/t2/.*
  transfer_type: stock
  objects_list_batch: 512
  objects_list_file_max_line: 1000000
  objects_list_files: null
  increment_mode:
    interval: 3600
  preserve_prefix: false
  last_modify_filter:
    filter_type: Greater
    timestamp: 1703055338
```

修改 access_key_id secret_access_key 等参数，适配自己的任务。template 命令按照任务类型创建模版,模板描述请参考[参考手册](reference_cn.md)。parameters 支持参数查询，包括支持的provider 以及 任务类型

注意：
- 当前任务 YAML 的 `type/source/target/attributes` 是顶层字段，不要再包一层 `task_desc`。
- `include` 和 `exclude` 使用正则表达式，不是 shell 通配符。需要匹配目录下所有对象时，建议写成 `^test/t1/.*`，不要写成 `test/t1/*`。
- 对象存储 `prefix` 会按字符串直接拼接，工具不会自动补 `/`。要同步目录前缀时建议写成 `tenant/`、`tenant_bak/`。
- 首次运行或修改源/目标/过滤规则后，建议设置 `start_from_checkpoint: false`，避免复用旧的 `/tmp/meta_dir` 元数据。

#### 执行任务

task 子命令用于执行任务

```shell
oss_pipe task exec filepath/task.yml
```

## 参考手册

### 命令详解

oss_pipe 同时支持命令行模式和交互模式 oss_pipe -i 进入交互模式。交互模式使用'tab'键进行子命令提示。

* task
  通过yaml描述文件执行相关任务
  * 命令格式
  
    ```shell
    task exec <filepath>
    ```

  * 命令行模式示例

    ```shell
    oss_pipe task exec yourpath/exec.yml
    ```

  * 交互模式示例

    ```shell
    oss_pipe> task exec yourpath/exec.yml
    ```

* template  
  生成任务模板,通过子命令指定任务类型
  * 命令格式
  
    ```shell
    template [subcommand] [file]
    ```

  * 命令行模式示例

    ```shell
    oss_pipe template transfer oss2oss /tmp/transfer_oss2oss.yml
    ```

  * 交互模式示例

    ```shell
    oss_pipe> template transfer oss2oss /tmp/transfer_oss2oss.yml
    ```

* parameters  
  参数查询，输出所支持的 oss 供应商，以及任务类型。
  * 命令格式
  
    ```shell
    oss_pipe parameters [subcommand]
    ```

  * 命令行模式示例

    ```shell
    oss_pipe parameters provider
    ```

  * 交互模式示例

    ```shell
    oss_pipe> parameters task_type
    ```

* tree  
  显示命令树。
  * 命令格式
  
    ```shell
    tree
    ```

* exit  
  退出交互模式。
  * 命令格式
  
    ```shell
    exit
    ```

### Oss 提供商支持

| 提供商代码 | 提供商描述          |
| ---------- | ------------------- |
| S3         | S3 兼容对象存储     |
| ALI        | 阿里云              |
| JD         | 京东云              |
| JRSS       | 京东云 JRSS         |
| HUAWEI     | 华为云              |
| COS        | 腾讯云 COS          |
| MINIO      | MinIO               |

### 任务类型

#### Transfer

##### oss2local

```yml
type: transfer
task_id: '7132612445025210369'
name: transfer_oss2local
source:
  provider: JD
  access_key_id: access_key_id
  secret_access_key: secret_access_key
  session_token: session_token
  endpoint: http://s3.cn-north-1.jdcloud-oss.com
  region: cn-north-1
  bucket: bucket_name
  prefix: test/samples/
  request_style: VirtualHostedStyle
target: /tmp
attributes:
  objects_transfer_batch: 64
  task_parallelism: 12
  meta_dir: /tmp/meta_dir
  target_exists_skip: false
  start_from_checkpoint: false
  large_file_size: 64m
  multi_part_chunk_size: 8m
  multi_part_chunks_per_batch: 16
  multi_part_parallelism: 8
  multi_part_max_parallelism: 12
  exclude:
  - ^test/t3/.*
  - ^test/t4/.*
  include:
  - ^test/t1/.*
  - ^test/t2/.*
  transfer_type: stock
  objects_list_batch: 512
  objects_list_file_max_line: 1000000
  objects_list_files: null
  preserve_prefix: true
```

##### oss2oss

```yml
type: transfer
task_id: '7132566496848515073'
name: transfer_oss2oss
source:
  provider: ALI
  access_key_id: access_key_id
  secret_access_key: secret_access_key
  session_token: session_token
  endpoint: http://oss-cn-beijing.aliyuncs.com
  region: cn-north-1
  bucket: bucket_name
  prefix: test/samples/
  request_style: VirtualHostedStyle
target:
  provider: JD
  access_key_id: access_key_id
  secret_access_key: secret_access_key
  session_token: session_token
  endpoint: http://s3.cn-north-1.jdcloud-oss.com
  region: cn-north-1
  bucket: bucket_name
  prefix: test/samples_bak/
  request_style: VirtualHostedStyle
attributes:
  objects_transfer_batch: 64
  task_parallelism: 12
  meta_dir: /tmp/meta_dir
  target_exists_skip: false
  start_from_checkpoint: false
  large_file_size: 64m
  multi_part_chunk_size: 8m
  multi_part_chunks_per_batch: 16
  multi_part_parallelism: 8
  multi_part_max_parallelism: 12
  exclude:
  - ^test/t3/.*
  - ^test/t4/.*
  include:
  - ^test/t1/.*
  - ^test/t2/.*
  transfer_type: stock
  objects_list_batch: 512
  objects_list_file_max_line: 1000000
  objects_list_files: null
  increment_mode:
    interval: 3600
  preserve_prefix: false
```

##### local2oss
  
```yml
type: transfer
task_id: '7132614104178626561'
name: transfer_local2oss
source: /tmp/source
target:
  provider: JD
  access_key_id: access_key_id
  secret_access_key: secret_access_key
  session_token: session_token
  endpoint: http://s3.cn-north-1.jdcloud-oss.com
  region: cn-north-1
  bucket: bucket_name
  prefix: test/samples/
  request_style: VirtualHostedStyle
attributes:
  objects_transfer_batch: 64
  task_parallelism: 12
  meta_dir: /tmp/meta_dir
  target_exists_skip: false
  start_from_checkpoint: false
  large_file_size: 64m
  multi_part_chunk_size: 8m
  multi_part_chunks_per_batch: 16
  multi_part_parallelism: 8
  multi_part_max_parallelism: 12
  exclude:
  - ^test/t3/.*
  - ^test/t4/.*
  include:
  - ^test/t1/.*
  - ^test/t2/.*
  transfer_type: stock
  objects_list_batch: 512
  objects_list_file_max_line: 1000000
  objects_list_files: null
  preserve_prefix: true
```

#### local2local
  
```yml
type: transfer
task_id: '7132615010349617153'
name: transfer_local2local
source: /tmp/source
target: /tmp/target
attributes:
  objects_transfer_batch: 64
  task_parallelism: 12
  meta_dir: /tmp/meta_dir
  target_exists_skip: false
  start_from_checkpoint: false
  large_file_size: 64m
  multi_part_chunk_size: 8m
  multi_part_chunks_per_batch: 16
  multi_part_parallelism: 8
  multi_part_max_parallelism: 12
  exclude:
  - ^test/t3/.*
  - ^test/t4/.*
  include:
  - ^test/t1/.*
  - ^test/t2/.*
  transfer_type: stock
  objects_list_batch: 512
  objects_list_file_max_line: 1000000
  objects_list_files: null
  preserve_prefix: true
```

#### TruncateBucket  
  
```yml
type: deletebucket
task_id: '7064088180835880961'
name: delete_bucket_task
target:
  provider: JD
  access_key_id: access_key_id
  secret_access_key: secret_access_key
  session_token: session_token
  endpoint: http://s3.cn-north-1.jdcloud-oss.com
  region: cn-north-1
  bucket: bucket_name
  prefix: test/samples/
  request_style: VirtualHostedStyle
attributes:
  objects_per_batch: 64
  task_parallelism: 12
  meta_dir: /tmp/meta_dir
  start_from_checkpoint: false
  exclude:
  - ^test/t3/.*
  - ^test/t4/.*
  include:
  - ^test/t1/.*
  - ^test/t2/.*
  objects_list_files_max_line: 1000000
```
  
#### OssCompare
  
```yml
type: compare
task_id: '7064090414587973633'
name: compare_oss2oss
source:
  provider: ALI
  access_key_id: access_key_id
  secret_access_key: secret_access_key
  session_token: session_token
  endpoint: http://oss-cn-beijing.aliyuncs.com
  region: cn-north-1
  bucket: bucket_name
  prefix: test/samples/
  request_style: VirtualHostedStyle
target:
  provider: JD
  access_key_id: access_key_id
  secret_access_key: secret_access_key
  session_token: session_token
  endpoint: http://s3.cn-north-1.jdcloud-oss.com
  region: cn-north-1
  bucket: bucket_name
  prefix: test/samples_bak/
  request_style: VirtualHostedStyle
check_option:
  check_content_length: true
  check_expires: false
  check_content: false
  check_meta_data: false
attributes:
  objects_per_batch: 256
  task_parallelism: 12
  meta_dir: /tmp/meta_dir
  start_from_checkpoint: false
  large_file_size: 64m
  multi_part_chunk: 8m
  multi_part_max_parallelism: 12
  exclude:
  - ^test/t3/.*
  - ^test/t4/.*
  include:
  - ^test/t1/.*
  - ^test/t2/.*
  exprirs_diff_scope: 10
  objects_list_batch: 512
  objects_list_files_max_line: 1000000
```

## 同步任务流程

![同步任务流程](./images/同步流程图-v3.png)
