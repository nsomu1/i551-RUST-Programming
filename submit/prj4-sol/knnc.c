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
char serv_Dir[17]="../../../../cs551", *dt_Dir;
int n;
const struct LabeledDataListKnn *trainingi , *testi ;
       if(argc == 3){
  	dt_Dir   = argv[1];
  	n = atoi(argv[2]);
  	printf("Arguments are properly assigned");
  	} 
  	else {
  	  printf("Arguments are not properly assigned please pass correct number of arguments");
  	}
  if (chdir(serv_Dir) != 0) {
  perror("changing directory to serv_Dir failed");
  }

trainingi= read_labeled_data_knn(dt_Dir,TRAINING_DATA,TRAINING_LABELS);
testi= read_labeled_data_knn(dt_Dir,TEST_DATA,TEST_LABELS);
    int fdisc;  
    char *directory="/server-dir";
    char *fifo = strcat(serv_Dir,directory);
     struct stat buff;
    if (stat(serv_Dir, &buff) == -1) {
    if(mkfifo(fifo, 0777) == -1) {
        if(errno == EEXIST) {
            printf("File exists already.\n");
            }
    } 
    else
    {
    printf("file created");}
   }
   
   double total=0;
   
   
    for(int j =0 ;j<n; j++ )
    {
		    char test1[80], test2[80];;
		    sprintf(test1,"%d",j);
		const struct LabeledDataKnn *labelknn1, *labelknn2;
  		const struct DataKnn *ft;
  		int templ1, op_index, templ2;
		   
		    fdisc = open(fifo,O_WRONLY);
		    write(fdisc, test1, sizeof(test1));
		    close(fdisc);
		    fdisc = open(fifo, O_RDONLY);
		    read(fdisc, test2, sizeof(test2));
		    labelknn1 = labeled_data_at_index_knn(testi,j);
		     templ1 = labeled_data_label_knn(labelknn1);
		    ft = labeled_data_data_knn(labelknn1);
	             op_index = knn(trainingi,ft,5);
		    labelknn2 = labeled_data_at_index_knn(trainingi,op_index);
		    templ2 = labeled_data_label_knn(labelknn2);
		    if(!(atoi(test2)) && templ2!=templ1)
		    {
	   		printf("\n %d[%d] %d[%s]",templ2,op_index,templ1,test1);
	   		total++;
		    }
		    close(fdisc);
    }
    
    printf(" \n%0.1f %% success\n",(((n-total)*100)/n));
    


 return 0;
 
 
}
