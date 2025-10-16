kernelsu-objs := ksu.o
kernelsu-objs += allowlist.o
kernelsu-objs += apk_sign.o
kernelsu-objs += sucompat.o
kernelsu-objs += throne_tracker.o
kernelsu-objs += core_hook.o
kernelsu-objs += ksud.o
kernelsu-objs += embed_ksud.o
kernelsu-objs += kernel_compat.o

kernelsu-objs += selinux/selinux.o
kernelsu-objs += selinux/sepolicy.o
kernelsu-objs += selinux/rules.o
ccflags-y += -I$(srctree)/security/selinux -I$(srctree)/security/selinux/include
ccflags-y += -I$(objtree)/security/selinux -include $(srctree)/include/uapi/asm-generic/errno.h

obj-$(CONFIG_KSU) += kernelsu.o

# 提示：此处的 KSU_VERSION 值须与manager项目目录中
# build.gradle.kts 文件的 appVersionCode 值保持一致
KSU_VERSION := 12128

# 在编译时打印手动设置的版本号
$(info -- KernelSU version: $(KSU_VERSION_FIXED))

# 将版本号作为宏 (KSU_VERSION) 注入到 C/C++ 源代码中
ccflags-y += -DKSU_VERSION=$(KSU_VERSION)

ifndef KSU_EXPECTED_SIZE
KSU_EXPECTED_SIZE := 0x057e
endif

ifndef KSU_EXPECTED_HASH
KSU_EXPECTED_HASH := c656015a34669996d529fad1dcb0f78d1965eb33543486788a7991042b07ed26
endif

ifdef KSU_MANAGER_PACKAGE
ccflags-y += -DKSU_MANAGER_PACKAGE=\"$(KSU_MANAGER_PACKAGE)\"
$(info -- KernelSU Manager package name: $(KSU_MANAGER_PACKAGE))
endif

$(info -- KernelSU Manager signature size: $(KSU_EXPECTED_SIZE))
$(info -- KernelSU Manager signature hash: $(KSU_EXPECTED_HASH))

ccflags-y += -DEXPECTED_SIZE=$(KSU_EXPECTED_SIZE)
ccflags-y += -DEXPECTED_HASH=\"$(KSU_EXPECTED_HASH)\"

ccflags-y += -Wno-implicit-function-declaration -Wno-strict-prototypes -Wno-int-conversion -Wno-gcc-compat
ccflags-y += -Wno-declaration-after-statement -Wno-unused-function

# Keep a new line here!! Because someone may append config
