#include <stdio.h>
#include <knn_ocr.h>
#include <errors.h>
#include <file-io.h>
#include <memalloc.h>
#include <trace.h>
#include <errno.h>
#include<stdlib.h>
#include <string.h>
#include <fcntl.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <unistd.h>

int main(int argc,char* argv[]){
char *serv_Dir, *dt_Dir;
int temp;
if(argc == 4){
  	serv_Dir = argv[1];
  	dt_Dir   = argv[2];
  	temp = atoi(argv[3]);
  	printf("Arguments are properly assigned\n");
  }
if (chdir(serv_Dir) != 0) {
perror("when changing directory to serv_Dir failed");
}
const struct LabeledDataListKnn *trainingi, *testi ;
trainingi= read_labeled_data_knn(dt_Dir,TRAINING_DATA,TRAINING_LABELS);
testi= read_labeled_data_knn(dt_Dir,TEST_DATA,TEST_LABELS);
    int fdisc, fdisc1;
    char *directory="/server-dir";
    char *fifo = strcat(serv_Dir,directory);
     struct stat buff;
    if (stat(serv_Dir, &buff) == -1) {
    if(mkfifo(fifo, 0777) == -1) {
        if(errno == EEXIST) 
             printf("File exists already.\n");
    } 
   }
   
  const struct LabeledDataKnn *labelknn1, *labelknn2;
  const struct DataKnn *ft;
  temp = atoi(argv[3]);
   while(1){
   	char test1[80],  test2[80];
   	int val, templ1, op_index, templ2;
   	fdisc = open(fifo, O_RDONLY);
	read(fdisc, test1, sizeof(test1));
	 val = atoi(test1);
	labelknn1 = labeled_data_at_index_knn(testi,val);
	 templ1 = labeled_data_label_knn(labelknn1);
	ft = labeled_data_data_knn(labelknn1);
	 op_index = knn(trainingi,ft,temp);
	labelknn2 = labeled_data_at_index_knn(trainingi,op_index);
	 templ2 = labeled_data_label_knn(labelknn2);
	
	close(fdisc);
	
  	
	if(templ2 != templ1) {
	   test2[0] = '0';
	}
	else{
	test2[0] = '1';}
	fdisc1 = open(fifo, O_WRONLY);
	write(fdisc1, test2, sizeof(test2));
	close(fdisc1);
	}


 return 0;
}
