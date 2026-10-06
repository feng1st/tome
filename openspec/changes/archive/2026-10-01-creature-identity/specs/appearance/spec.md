历史归档，不符合先英文后中文规范，请勿参考

## REMOVED Requirements

### Requirement: 形象词表注册表

**Reason**: 形象词表与外观表 1:1 闭合，分裂只因词表曾滞留 core；迁入 display 后合并为 figure capability 的形象表。
**Migration**: 见 figure spec 的"形象表注册表"需求。

### Requirement: 外观画法数据文件

**Reason**: 外观画法并入形象表条目，不再单独成表。
**Migration**: 见 figure spec 的"形象表注册表"与"动画回退 Idle"需求。

### Requirement: 形象与外观数据校验

**Reason**: 跨文件闭合校验随两表合并在结构上消失；单表校验由 figure capability 承接。
**Migration**: 见 figure spec 的"形象数据校验"需求。

### Requirement: 生物呈现形象

**Reason**: 呈现语义不变，归属并入 figure capability；主角出生形象的达成方式改经 creature-identity 的形象绑定链，不再由代码指定。
**Migration**: 见 figure spec 的"生物呈现形象"需求与 creature-identity spec 的"形象绑定表"需求。

### Requirement: warrior 外观数据

**Reason**: 归属并入 figure capability，数值不变。
**Migration**: 见 figure spec 的"warrior 形象数据"需求。
