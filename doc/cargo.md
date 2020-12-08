发布配置是预定义的、可定制的带有不同选项的配置，它们允许程序员更灵活的控制代码编译的多种选项，每一个配置都彼此项目独立。

 Cargo有两个主要的配置： 

（1）运行cargo build时采用的dev配置；（生成目标文件在target/debug中） 

（2）运行cargo build --release时的release配置。（生成目标文件在target/release中） 

当没有专门配置时，Rust会使用默认配置，相当于如下配置： 

 [profile.dev] 

opt-level = 0 #dev的默认配置

 [profile.release] 

opt-level = 3 #release的默认配置

 #其中，opt-level时优化级别，此值的配置是从0-3，越大表示优化越多。优化越多编译时间越长。  

例子： 

可以在Cargo.toml中添加： 

 [profile.dev] 

opt-level = 1 #修改dev的默认配置，观察编译时间是否会变长。

