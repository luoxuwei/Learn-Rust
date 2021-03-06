# rust语言学习观

## 建议一：从整体出发，不要让自己陷入到细节中去

学习要有掌控感，先从外围从整体慢慢了解rust，有哪些特性，设计哲学是什么，社区生态是怎么样的，可以对rust有一个高屋建瓴的结构性认识，就像画画一样，先画好一个轮廓再画细节，有利于建立知识网络，可以增加掌控感，并且可以促使信息进入长时记忆，有效降低信息的损耗，直接进入到细节中就会很迷茫。因为rust语言中的细节很多，最典型的是rust中的字符串，如果对rust整体设计没有任何概念就会头疼，怎么rust里面这么多字符串，但其实这些字符串背后都有一致性规则，了解这些规则，字符串就会变得非常简单，同理可以推广到rust语言很多特性。

## 建议二：抛弃一次性学会的念头，分层次递进式学习

rust语言中蕴含的知识体系本身就是有层次的，任何知识体系都是有层次的。抱着一次性学会的念头就是和自己作对。

rust语言从整体上可以分为四个层次，rust语言基于现代化的类型系统，上可承载所有权系统和编程范式，下可半自动化管理内存，在学习的时候也需要分为这四个层次。

## 建议三：和你已知的知识建立联系

这个世界上没有任何一件事是凭空出现的，你不可能盖一座空中楼阁，知识也是一样，别看rust中有很多新概念，但其实每一个概念，都可以和旧的知识建立关联，新语言的出现本质上是为了解决某一个问题，比如rust中的所有权概念，所有权是解决问题的手段，想要理解它就要明白问题所在，也就是说明白什么是安全，想明白什么是安全就要搞清楚这个世界为什么不安全，安全和不安全是一体两面，明白为什么不安全自然就明白rust解决不安全的思路，也就自然理解了所有权的概念。顺着知识的脉络去寻找和你已知的知识建立联系。这样才有助于构建知识网络。如果不能建立联系那你的知识结构肯定有漏洞。那么你就顺着知识的脉络查漏补缺。

把rust里的概念和其他编程语言的概念相比较也是可以的，但是要避免陷入到下面的误区里面，为什么rust语言没有继承？为什么rust语言不提供三元操作符？等等诸如此类的问题。这就属于钻牛角尖了。继承虽然是实现代码复用的有力手段，但它并非永远是完成这项工作的最佳工具，rust采用的是组合的手段，相比于继承组合面向接口设计，可以极大程度的降低父子系统具体实现之间的耦合，rust语言不提供三元操作符，也是类似的原因，因为rust里的条件控制是表达式而非语句。所以在和已知的知识建立联系的时候，应该更多的关注rust语言提供了什么样的解决问题的思路和方法，而非批判rust语言没有这个特性没有那个特性。这样才能更好的理解rust语言。

## 建议四：学会阅读源码从源码中学习

首先rust标准库源码值得一读。不必通篇全读，只需要在学习过程中看到哪读到哪，比如你学习string时，看到源码就会发现，原来string就是一个u8类型的动态数组，那么它的一些方法是不是和动态数组的方法类似了？这一下就能找到共性，又看到u8是一个字节序列再结合文档说明就很容易发现字符串是一个utf-8编码的字节序列。

然后就是第三方库，可以找一些常用库学习最佳实践。其实阅读源码不要求你完全学会rust才行，掌握了基本语法即可阅读远吗，阅读的过程中编译器不会出来干扰你，你只需要阅读源码的整体组织结构，了解rust代码的组织方式，编码风格，设计风格，最佳实践即可，阅读源码也是一中实践

## 建议五：通过主题式阅读填补知识空白

举个例子，假如你不懂rust中的futher Async/await概念可以去翻翻javascript Python 中相关概念，概念都是相通的，不同的是具体的实现，先把概念理清了再回头看rust中的实现，就会发现rust异步功能之所以姗姗来迟，就是因为rust语言是一门高度要求一致性内存安全性能的语言，所以在异步语法设计上要做诸多考量。这就是一种主题式阅读补基础的方法。有助于拧清概念和实现，建立更容易迁移的知识结构。

## 建议六：时刻把握rust设计哲学

安全实用性能，我们说要尽力和已知的知识相关联，但也要注意以rust特性为出发点，否则已知的知识只会成为你学习的绊脚石，

## 建议七：有意识的构建rust语言的心智模型

心智模型就是人脑或者说思想和意志在客观世界的延伸，可以帮助你判断，编程也是一样，其实前面说的rust语言架构，就是rust语言在我心里构建的心智模型，我通过这个心智模型来和rust编译器达成一致，所以在写代码的时候可以尽可能准守rust语言规则，大幅度减低编译器报错的情况，即便是报错了也可以快速的通过阅读错误信息定位问题。

如何构建了，就是要从纷繁复杂的语言特性中寻找共性，寻找一致性，在心智里串起一个精简的知识网络

## 建议八：多分享多提问多交流

有人的地方就有bug，再怎么天才也有出错的时候，所以学习最忌讳的就是闭门造车，如果你不说又不知道自己的理解是否存在误区

## 建议九：为开源项目做贡献，锻炼自己

# rust 语言概览

搞清楚三个问题：rust从哪里来，是什么，到哪里去。

搞清楚这个三个问题，对了解rust社区文化，理解rust语法特性，以及掌握rust语言一致性，都会有所帮助

## rust从哪里来

一门语言的诞生，是为了解决一个问题。

Gradon对rust语言的期望：

- 必须安全，不易崩溃（尤其是在操作内存的时候，这一点尤为重要）
- 不需要引入gc，注重性能
- rust也不是仅仅拥有一个主要特性的语言，而应该拥有一系列广泛的特性，这些特性之间又不乏一致性，这些特性可以很好的互相协作，从而使该语言更容易编写维护和调试，让程序员写出更安全更高效的代码。

rust在字形组合上糅合了trust和robust，暗示了信任和鲁棒性

- 内存安全为第一准则
- 注重并发安全，避免数据竞争
- 持续提升性能
- 保持语言的高度一致性
- 语言必须有可见的实用性
- 注重开发体验和学习体验
- 现代化的语言特性
- 拥抱开源社区

## rust是什么

rust是新时代的c语言。这句话指的是rust和c语言的历史地位是相似的。

理由

- rust是一门通用型语言 

  - 通用意味着适合所有领域绝大部分场景，裸机，操作系统，网络服务，上层应用等

  - 于其他语言横向比较，rust拥有现代化语言特性，应用范围覆盖到c/cpp/java/go/JavaScript等

- rust内存安全方案针对的是c语言的不足

    虽然rust内存安全机制是从cpp借鉴而来，但它本质上还是为了弥补c语言的缺陷，rust号称内存安全，但它并没有打算做到百分百安全，它旨在消除非法访问内存的安全隐患。 

    - 禁止对空指针和悬垂指针进行解引用

    - 读取未初始化的内存

    - 缓冲区溢出

    - 非法释放已经释放过或未分配的指针（即重复释放）

    值得一提的是内存泄漏并不属于rust承诺的内存安全范围，这也是让大多数初学者误解的地方。
  
- 安全且无缝的沟通c语言

    - 通过C-ABI零成本和c语言打交道
    - 划分了safe rust和unsafe rust

    rust语言包含safe rust和unsafe rust两大部分，因为在rust看来这个世界是不安全的，safe rust就是在这不安全的世界上重新构建出一片安全地带。

    在safe rust中编译器会对代码进行安全检查，满足安全规则的放行，违反安全规则的不会通过。safe rust就像是现实世界中的交通安全系统，你如果想合法开车就必须先考驾照，熟悉安全交通法规，行车的时候才会准守交通规则。

    而unsafe rust则是safe rust的超集，在unsafe rust下编译器依然会进行安全检查，但有如下5种情况是例外，解引用裸指针，调用unsafe 函数和犯法，访问和修改可变静态变量，实现unsafe trait，读写union 联合体中的字段。

    为什么要有unsafe rust 因为safe rust这篇安全地带，并不是孤立存在的，rust还是要和底层操作系统和其他语言打交道，此时编译器的安全检查将鞭长莫及，使用unsafe rust你可以方便的通过C-ABI零成本和c c++及其他语言打交道，复用现成的代码库，

    使用unsafe rust也可以在某些边界条件已知的情况下，跳过一些安全检查，可以提升性能。但此时并非unsafe 你已经了解了某些边界条件，

    如果说safe rust 是编译器对开发者的安全承诺，那么unsafe rust则是，开发者对编译器以及其他开发者的安全承诺或者是预警。

- rust是具有混合范式的“面向过程”式的编程语言。

    - rust包含了面向对象，函数式和泛型三种编程范式
    - oop和fp范式在rust语言中是作为语言特性而存在，并非是抽象方式
    - rust让你专注于解决问题本身，而不受编程范式思想框架的干扰

    为什么说rust是面向过程式语言？你用传统面向对象语言或函数式语言去写代码的时候，你需要使用面向对象思想或者是函数式思想，去对问题领域进行抽象和建模，而用rust语言则不然，虽然它是混合范式，但实际上，oop和fp范式在rust语言中是作为语言特性而存在，并非是抽象方式。用rust写代码的时候体验其实是和c这样的面向过程的语言是相似的，rust让你专注于解决问题本身，而不受编程范式思想的干扰，rust设计哲学之一实用性就是这样体现出来的。

- 和c语言类似，负担了时代的使命。

    - 操作系统遭遇发展瓶颈，rust来拯救
    - rust是wasi（WebAssembly System interface）推广和普及的背后推手
    - 基于rust实现的新语言如雨后春笋般冒出

    c语言的发展史等价于操作系统和高级编程语言的发展史，c语言伴随着linux 等操作系统发展而发展，c语言也是其他高级语言底层的实现语言，rust正站在新时代的拐点上，有两点原因，第一 操作系统目前正遭遇发展的瓶颈docker等容器化工具已经无法从根本上满足时代发展的需求，基于rust实现的新一代操作系统由此而诞生，比如google创建的fuchsia，其中rust代码量大约占百分之30，亚马孙则用rust基于linux开发了可以直接在裸机或者虚拟机上运行容器的操作系统等。第二点，由于WebAssembly 飞速发展，wasi的目标是成为一个通用性的概念性的系统调用接口，可以说是新时代的posix接口，

## rust到哪里去

rust将为各个领域的基础设施建设做出贡献，未来也许会在多个领域出现杀手级应用。

## 总结

在学习中要牢记这几点，第一rust是追求安全并发和性能三连击的语言，其语法特性设计都遵循这一目标，第二rust分为safe 和unsafe ，safe 是构建与unsafe 之上，unsafe rust不等于不安全，而是指编译器在5种特殊的操作时，把安全性交给了开发者自己来保证，第三rust是新时代的c语言，它拥有比肩c语言的通用性，具有广泛的应用领域，它针对性弥补了c语言内存不安全的不足，并且可以通过c-abi无缝于c语言沟通，虽然身具多范式但抽象方式和c语言类似，让开发者专注于问题的解决，而非范式的纠结，和c语言类似站在了时代的拐点推动着时代的发展，

