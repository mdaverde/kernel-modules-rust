ifneq ($(KERNELRELEASE),)

# In kbuild context
proc_iter-objs := main.o
obj-m := proc_iter.o

else
# In normal make context
KERNELDIR ?= /lib/modules/$(shell uname -r)/build
PWD := $(shell pwd)

.PHONY: modules
modules:
	$(MAKE) -C $(KERNELDIR) M=$(PWD) modules

.PHONY: clean
clean:
	$(MAKE) -C $(KERNELDIR) M=$(PWD) clean

endif
