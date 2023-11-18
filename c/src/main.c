#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/types.h>

char *input() {
	u_int mem = 8;
	u_int mem_i = 1;
	
	char *s;
	s = malloc(mem);
	
	int i = 0;
	while (1) {
		int c = getchar();

		if(i == mem_i*mem) { 
			mem_i++;
			s = realloc(s, mem_i*mem);
		}

		if (c != 10 && c >= 32) {
			s[i] = c;
			i++;
		}

		if (c == 10) {
			s[i] = '\0';
			break;
		}
	}
	return s;
}

int main() {
	char* usr_in;
	usr_in = input();
	printf("%s\n", usr_in);
	return 0;
}
