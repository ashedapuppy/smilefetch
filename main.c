#include "include/main.h"

int main(int arc, char **arv)
{
    char *os_name = malloc(sizeof(char) * 100);
    char *kernel_name = malloc(sizeof(char) * 100);

    get_os(&os_name);
    get_kernel(&kernel_name);
    fprintf(stdout,"distro: \t\t%s\n", os_name);
    fprintf(stdout,"kernel version: \t%s\n", kernel_name);
    return 0;
}

