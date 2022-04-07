# The name of the executable
KNND = knnd
KNNC = knnc

FLAGS = -Wall -Wextra -g

SHARED_LIB= -L /home/nsomu1/projects/cs551/lib 
INCLUDE_LIBRARIES= -I /home/nsomu1/projects/cs551/include

# FLAGS = -Wall -O3
CC = gcc
BIN = bin
OBJ = obj

# Store executables in a subdirectory
all: $(KNNC) $(KNND)

$(KNND): $(OBJ)/knnd.o 
	$(CC) $(FLAGS) $(OBJ)/knnd.o $(SHARED_LIB) -lcs551 -lknn_ocr -o $@ 

$(OBJ)/knnd.o: knnd.c
	$(CC) $(FLAGS) -c knnd.c -o $@ $(INCLUDE_LIBRARIES) 

$(KNNC): $(OBJ)/knnc.o 
	$(CC) $(FLAGS) $(OBJ)/knnc.o $(SHARED_LIB) -lcs551 -lknn_ocr -o $@

$(OBJ)/knnc.o: knnc.c 
	$(CC) $(FLAGS) -c knnc.c -o $@ $(INCLUDE_LIBRARIES)
	
	
