#include<stdio.h>
#include<sys/types.h>
#include<unistd.h>
#include<sys/wait.h>
#include<stdlib.h>
#include <limits.h>

static unsigned long get_num(const char *str)
{
	char *end, c;
	unsigned long val;

	if (!str)
		return 0;
	val = strtoul(str, &end, 0);
	if (!val || val == ULONG_MAX)
		return 0;
	while ((c = *end++) != 0) {
		switch (c) {
		case 'k':
			val <<= 10;
			break;
		case 'M':
			val <<= 20;
			break;
		case 'G':
			val <<= 30;
			break;
		default:
			return 0;
		}
	}
	return val;
}

int main(int argc, char *argv[])
{
	unsigned long size = get_num(argv[1]);
	// printf("%d", size);
	int status;
    int* ptr;
    ptr = (int*)malloc(size);
	for (int i = 0; i < 40000; i++) 
	{
		int pid = fork();
		if (pid < 0) return -1;
		if (pid == 0) return 0;
		waitpid(pid, &status, 0);
	}
    free(ptr);
	return 0;
}