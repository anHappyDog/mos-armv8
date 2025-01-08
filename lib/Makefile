INCLUDES    := -I../include/

%.o: %.c
	$(CC) $(CFLAGS) $(INCLUDES) -c -o $@ $<

%.o: %.S
	$(CC) $(CFLAGS) $(INCLUDES) -c -o $@ $<

.PHONY: all clean

all: print.o fdt.o string.o

clean:
	rm -rf *~ *.o
