#include "include/main.h"

void get_os(char **os_name)
{
    FILE *os_file;
    if ((os_file = fopen("/etc/os-release", "r")) == NULL) {
        fprintf(stderr, "error opening /etc/os-release");
        exit(1);
    }
    fscanf(os_file, "%[^\n]", *os_name);
    for (; (*os_name)[0] != '=';)
        (*os_name)++;
    (*os_name)++;
    if ((*os_name)[0] == '"') {
        (*os_name)++;
        (*os_name)[strlen(*os_name) - 1] = 0;
    }
}

void get_kernel(char **kernel_name)
{
    FILE *kernel_file;
    int i = 0;
    if ((kernel_file = fopen("/proc/version", "r")) == NULL) {
        fprintf(stderr, "error opening /proc/version");
        exit(1);
    }
    fscanf(kernel_file, "%[^\n]", *kernel_name);
    for (i; (*kernel_name)[i] != '('; i++);
    (*kernel_name)[i] = 0;
}
