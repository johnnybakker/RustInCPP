#include <stdio.h>


#ifdef __cplusplus
extern "C"{
#endif
 
  void play();
 
#ifdef __cplusplus
}
#endif

int main() {
	printf("Hello, world!\n");
	play();

	return 0;
}