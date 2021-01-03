#include "include/main.h"

int main(int arc, char **arv)
{
    char *os_name = malloc(sizeof(char) * 100);
    char *kernel_name = malloc(sizeof(char) * 100);
    char *uptime = malloc(sizeof(char) * 100);
    char *user = malloc(sizeof(char) * 100);
    char *cpu_name = malloc(sizeof(char) * 100);

    get_os(&os_name);
    get_kernel(&kernel_name);
    get_uptime(&uptime);
    get_user(&user);
    get_cpuinfo(&cpu_name);

    fprintf(stdout,"user: \t\t%s\n",user);
    fprintf(stdout,"distro: \t%s\n", os_name);
    fprintf(stdout,"kernel: \t%s\n", kernel_name);
    fprintf(stdout,"uptime: \t%s seconds\n", uptime);
    fprintf(stdout,"cpu: \t\t%s\n", cpu_name);
    exit(0);
}

