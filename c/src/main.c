#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main() {
	// -- static string
	char x[15];
	fgets(x, 15, stdin);
	printf("%s\n", x);

	int a;
	int b;
	scanf("%d %d\n", &a, &b);
	printf("a:%d b:%d", a, b);
	// -- dynamic string
	printf("Dynamic string testing: \n");
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
