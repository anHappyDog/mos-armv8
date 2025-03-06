## 内核启动

aarch64总共有4种特权级，分别是EL0~3，我们只需要使用EL0与EL1两个特权级。EL0可被看作是用户态，EL1可被看作是内核态。

SCR_EL3 RW bit 


## 外设访问



## 异常处理

aarch64存在EL0~3总计4个特权级，我们只需要使用EL0与EL1两个特权级。并且两个特权级都使用aarch64执行状态。EL0可被看作是用户态，EL1可被看作是内核态。

每个特权级存在SP_ELx与ELR_ELx寄存器，SP_ELx用于保存栈指针，ELR_ELx用于保存异常返回地址。当异常发生时，处理器会选择对应的SP_ELx与ELR_ELx。

aarch64使用异常向量表存储对应入口地址。

![image-20250303054641578](https://lonelywatch-1306651324.cos.ap-beijing.myqcloud.com/image-20250303054641578.png)

![image-20250303054658518](https://lonelywatch-1306651324.cos.ap-beijing.myqcloud.com/image-20250303054658518.png)

VBAR_ELx（Vector Base Address Register）：定义异常向量表基地址。
SPSR_ELx（Saved Program State Register）：保存异常前的 PSTATE。
ELR_ELx（Exception Link Register）：保存返回地址。
ESR_ELx（Exception Syndrome Register）：记录异常原因（如 EC、IL、ISS）。
SCR_EL3（Secure Configuration Register）：控制安全状态和异常路由。
HCR_EL2（Hypervisor Configuration Register）：虚拟化相关的异常控制。
PSTATE：保存处理器状态（包括中断屏蔽位 DAIF）。


当异常发生时，PE进行以下内容:

需要保存到内容有:

30个通用寄存器，1个sp寄存器,ESR_EL1,FAR_EL1，SPSR_EL1,ELR_EL1。

栈的使用取决于Spsel位，如果Spsel为0，那么使用SP_EL0，否则使用SP_ELn。

## 内存管理

aarch64将内存区域分为Device与Normal两种类别，Device区域不可Cache，向外Sharable；Normal区域则取决于对应区域的属性。

aarch64存在VMSAv8-64与VMSAv8-128两种映射方式，我们使用VMSAv8-64模式。VMSAv8-64支持4KB,16KB,64KB粒度大小的页面，我们使用4KB大小的页面。

aarch64的地址翻译同样通过MMU进行，主要包含两个阶段：Stage 1与Stage 2，Stage 1 将VA转换为IPA，Stage 2将IPA转换为PA。Stage 2只有在实现虚拟机才会使用，主要用于不同Guest OS的隔离，这里我们只需要实现Stage 1.

aarch64的页表寄存器为TTBR_ELx。这里我们使用TTBR_EL1存放内核页表，TTBR_EL0存放用户页表。 aarch64页表同样支持巨页。

![image-20250303042717388](https://lonelywatch-1306651324.cos.ap-beijing.myqcloud.com/image-20250303042717388.png)

TLB会缓存所有有效且访问不会出现异常的页表项。

aarch64使用四级页表，同时VA可以支持51或者48位的地址空间，这里我们只使用48位地址空间，TCR_ELx.DS为0。

aarch64支持两个VA 范围，如果支持，那么TTBR0_ELx指向低内存区域，TTBR1_ELx指向高内存区域,这两个寄存器的使用在地址翻译时取决于VA[55]，为1则使用TTBR1_ELx，否则使用TTBR0_ELx。范围大小由TCR_ELx控制。

VMSAv8-64的页表项为8字节，页表中所包含页表项数取决于TCR_ELx.TG0与TCR_ELx.TG1，这里我们使用4KB大小的页面，所以每个页表项对应一个4KB的物理页。


![image-20250303043148798](https://lonelywatch-1306651324.cos.ap-beijing.myqcloud.com/image-20250303043148798.png)

![image-20250303044122543](https://lonelywatch-1306651324.cos.ap-beijing.myqcloud.com/image-20250303044122543.png)


页表项格式如下：

![image-20250303044436207](https://lonelywatch-1306651324.cos.ap-beijing.myqcloud.com/image-20250303044436207.png)

![image-20250303044830117](https://lonelywatch-1306651324.cos.ap-beijing.myqcloud.com/image-20250303044830117.png)

![image-20250303045208650](https://lonelywatch-1306651324.cos.ap-beijing.myqcloud.com/image-20250303045208650.png)

![image-20250303045240851](https://lonelywatch-1306651324.cos.ap-beijing.myqcloud.com/image-20250303045240851.png)