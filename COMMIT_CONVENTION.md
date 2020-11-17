# Git Commit Message Convention
> 这是根据[Angular的提交惯例](https://github.com/angular/angular/blob/master/CONTRIBUTING.md#commit)改编的。

信息必须与以下的regex匹配。

```
^Merge.+|(build|ci|docs|feat|fix|perf|refactor|test)(\(.+\))?
```

### Examples

```
feat(core): 添加配置选项
```

### Commit Message Format

提交消息由**header**、**body**和**footer**组成。 header有**type**、**scope**和**subject**。

```
<type>(<scope>): <subject>
<BLANK LINE>
<body>
<BLANK LINE>
<footer>
```

**header**是强制性的，header的**scope**是可选的。

### Type

**type**必须是下列之一：

- **build**: 影响构建系统或外部依赖关系的变化
- **ci**: 改变CI配置文件和脚本
- **docs**: 只修改文档
- **feat**: 一个新的功能
- **fix**: 修复一个错误
- **perf**: 提高性能的代码修改
- **refactor**: 既不修复bug也不增加功能的代码修改
- **test**: 增加缺失的测试或纠正现有测试

### Scope

**scope**可以是任何指定提交修改地点的东西，包含但不限于以下几种:

- **core**: 核心模块
- **system**: 系统模块
- **uaa**: 用户账号与授权模块
- **oa**: 人事OA模块

### Subject

**subject**一句话简要的描述这次修改的内容。

### Body

**body**应该包括改变的动机，并与之前的行为形成对比。

### Footer

**footer**应包含任何有关**Breaking Changes**的信息

**Breaking Changes**应该以`BREAKING CHANGE:`开头，并加上一个空格或两个新行。然后，提交消息的其余部分将用于此。

```
BREAKING CHANGE: <breaking change summary>
<BLANK LINE>
<breaking change description + migration instructions>
<BLANK LINE>
<BLANK LINE>
Fixes #<issue number>
```