COURSE_DIR:= ../../../../cs551
COURSE_INCLUDE_DIR:= $(COURSE_DIR)/include
COURSE_LIB_DIR:= $(COURSE_DIR)/lib

CFLAGS:= -g -Wall -pedantic -std=c18 -I $(COURSE_INCLUDE_DIR)
LDFLAGS:= -L $(COURSE_LIB_DIR) -Wl,-rpath,$(COURSE_LIB_DIR)
LDLIBS:= -lcs551 -lknn_ocr 

TARGETS:= knnc knnd

all:		$(TARGETS)

knnc:		knnc.o 
knnd:		knnd.o 

knnc.o:		knnc.c 
knnd.o:		knnd.c 


.PHONY:		clean

clean:
	rm -f *.o $(TARGETS) *~
