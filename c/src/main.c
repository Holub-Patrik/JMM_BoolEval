#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main() {
	char *usr_in;
	usr_in = (char *) malloc(10);
	scanf("%[^\n]s", usr_in);

	printf("%s\nString length: %lu\n", usr_in, strlen(usr_in));
	printf("I will write the tenth char: %c\n", usr_in[10]);

	for(int i = 0; i < strlen(usr_in); i++) {
		printf("%c", usr_in[i]);
	}
	printf("\n");
	return 0;
}
