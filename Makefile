# Whomst the fuck needs fancy makefiles

CC = g++

EXE = bin/out.exe

SRC = src
INC = inc

OBJECTS = src/main.o src/ws/codegenerator.o

default: exe clean

run: exe runexe clean

%.o: %.cpp
	$(CC) -o $@ -c $< 

exe: $(OBJECTS)
	$(CC) $(OBJECTS) -o $(EXE)

runexe:
	$(EXE)

clean:
	rm $(OBJECTS)