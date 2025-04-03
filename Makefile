CC = clang
CFLAGS = -O3

alias: alias.o
	$(CC) $(CFLAGS) -o alias alias.o

clean:
	-rm -f alias alias.o
