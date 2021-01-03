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
    if ((*os_name)[0] == '"')
        (++(*os_name))[strlen(*os_name) - 1] = 0;
}

void get_kernel(char **kernel_name)
{
    FILE *kernel_file;
    int i = 0;
    if ((kernel_file = fopen("/proc/version", "r")) == NULL)
    {
        fprintf(stderr, "error opening /proc/version");
        exit(1);
    }
    fscanf(kernel_file, "%[^\n]", *kernel_name);
    for (i; (*kernel_name)[i] != '('; i++);
    (*kernel_name)[i] = 0;
}

/*
 *void get_model(char **model_name, char **model_version)
 *{
 *    FILE *model_name_file;
 *    FILE *model_version_file;
 *    if ((model_name_file = fopen("/sys/devices/virtual/dmi/id/product_name", "r")) == NULL) {
 *        fprintf(stderr, "error opening /sys/devices/virtual/dmi/id/product_name");
 *        exit(1);
 *    }
 *    if ((model_name_file = fopen("/sys/devices/virtual/dmi/id/product_version", "r")) == NULL) {
 *        fprintf(stderr, "error opening /sys/devices/virtual/dmi/id/product_version");
 *        exit(1);
 *    }
 *}
 */

void get_uptime(char **uptime)
{
    FILE *uptime_file;
    if ((uptime_file = fopen("/proc/uptime", "r")) == NULL)
    {
        fprintf(stderr, "error opening /proc/uptime");
        exit(1);
    }
    fscanf(uptime_file, "%[^ ]", *uptime);
    int uptime_long = strtol((*uptime), NULL, 10);
}

void get_shell(char **shell)
{
}

void get_user(char **user)
{
    FILE *user_name = popen("whoami", "r");
    fscanf(user_name, "%s", (*user));
}

void get_cpuinfo(char **cpu_name)
{
    FILE *cpu_name_file;
    char *line = malloc(1024);
    if ((cpu_name_file = fopen("/proc/cpuinfo", "r")) == NULL)
    {
        fprintf(stderr, "error opening /proc/cpuinfo");
        exit(1);
    }
    while (fgets(line, 1024, cpu_name_file) != NULL)
    {
        if(strstr(line, "model name")){
            (*cpu_name) = line;
            for(; (*cpu_name)[0] != ':';)
                (*cpu_name)++;
            (*cpu_name) += 2;
            (*cpu_name)[strlen(*cpu_name) - 1] = 0;
            break;
        }
    }
}
