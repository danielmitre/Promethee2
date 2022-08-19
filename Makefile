CC=g++-11
CFLAGS = -std=c++14
LIBS = -lpthread -L/usr/local/Cellar/libtiff/4.4.0_1/lib/ -ltiff
INC=-I/usr/local/Cellar/libtiff/4.4.0_1/include
TARGET=-

DEPS = promethee/*.h

SRCS = $(shell find promethee -name *.cpp)
OBJS := $(addsuffix .o,$(basename $(SRCS)))

all: run

promethee/%.o: promethee/%.cpp $(DEPS)
	$(CC) -c -o $@ $< $(CFLAGS) $(LIBS) $(INC)

promethee/vanilla/%.o: promethee/vanilla/%.cpp $(DEPS)
	$(CC) -c -o $@ $< $(CFLAGS) $(LIBS) $(INC)

run: $(OBJS)
	$(CC) -o $@ $^ $(LIBS)

clean: 
	rm -rf $(OBJS)
	rm -f run
