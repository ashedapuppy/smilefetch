#include "include/main.h"

#define reset "\x1b[0m"
#define bold "\x1b[1m"
#define black "\x1b[30m"
#define red "\x1b[31m"
#define green "\x1b[32m"
#define yellow "\x1b[33m"
#define blue "\x1b[34m"
#define magenta "\x1b[35m"
#define cyan "\x1b[36m"
#define white "\x1b[37m"

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

    fprintf(stdout,red "user" reset ":" "\t\t%s\n",user);
    fprintf(stdout,red "distro" reset ":" "\t\t%s\n", os_name);
    fprintf(stdout,red "kernel" reset ":" "\t\t%s\n", kernel_name);
    fprintf(stdout,red "uptime" reset ":" "\t\t%s seconds\n", uptime);
    fprintf(stdout,red "cpu" reset ":" "\t\t%s\n", cpu_name);
    exit(0);
}

