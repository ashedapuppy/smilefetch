#include "include/main.h"

void get_os(char **os_name)
{
    FILE *os_file;
    if ((os_file = fopen("/etc/os-release", "r")) == NULL)
    {
        fprintf(stderr, "error opening /etc/os-release");
        exit(1);
    }
    fscanf(os_file, "%[^\n]", *os_name);
    for (; (*os_name)[0] != '=';)
        (*os_name)++;
    (*os_name)++;
}
