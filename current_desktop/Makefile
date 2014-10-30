CFLAGS += -Os -s

# Support cross compilation to/from 32/64 bit.
ARCH := $(word 1, $(subst -, ,$(TARGET)))
ifeq ($(ARCH),i686)
CFLAGS += -m32 -fPIC
else
CFLAGS += -m64 -fPIC
endif

all:
	$(CC) $(CFLAGS) cur_desk.c -c -o "$$OUT_DIR"/cur_desk.o
	$(AR) crus "$$OUT_DIR"/libcurrent_desktop.a "$$OUT_DIR"/cur_desk.o
