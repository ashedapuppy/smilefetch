#include "include/main.h"

void get_os(char **os_name)
{
    FILE *os_file;
    if ((os_file = fopen("/etc/os-release", "r")) == NULL)
    {
        fprintf(stderr, "error opening /etc/os-release\n");
        exit(1);
    }
    fscanf(os_file, "%[^\n]", *os_name);
    for (; (*os_name)[0] != '=';)
        (*os_name)++;
    (*os_name)++;
    if ((*os_name)[0] == '"')
        (++(*os_name))[strlen(*os_name) - 1] = 0;
    fclose(os_file);
}

void get_kernel(char **kernel_name)
{
    FILE *kernel_file;
    int i = 0;
    if ((kernel_file = fopen("/proc/version", "r")) == NULL)
    {
        fprintf(stderr, "error opening /proc/version\n");
        exit(1);
    }
    fscanf(kernel_file, "%[^\n]", *kernel_name);
    for (i; (*kernel_name)[i] != '('; i++);
    (*kernel_name)[i] = 0;
    fclose(kernel_file);
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
        fprintf(stderr, "error opening /proc/uptime\n");
        exit(1);
    }
    fscanf(uptime_file, "%[^ ]", *uptime);
    long uptime_long = strtol((*uptime), NULL, 10);
    long uptime_days = uptime_long / 86400; // /60 / 60 / 24 to get num of hours
    long uptime_hours = uptime_long / 3600 % 24; // /60 /60 %24 to get hours - days
    long uptime_minutes = uptime_long / 60 % 60; // minutes - hours
    long uptime_seconds = uptime_long % 60;
    if (uptime_days != 0) {
        sprintf(*uptime, "%ld days, %ld hours, %ld minutes", uptime_days, uptime_hours, uptime_minutes);
    } else if (uptime_hours != 0) {
        sprintf(*uptime, "%ld hours, %ld minutes", uptime_hours, uptime_minutes);
    } else if (uptime_minutes != 0) {
        sprintf(*uptime, "%ld minutes, %ld seconds", uptime_minutes, uptime_seconds);
    } else {
        fprintf(stderr, "Uptime = 0? sounds like an error\n");
        exit(1);
    }
    fclose(uptime_file);
}

void get_shell(char **shell, char *user)
{
    FILE *shell_file;
    char *line = malloc(1024);
    if ((shell_file = fopen("/etc/passwd", "r")) == NULL)
    {
        fprintf(stderr, "error opening /etc/passwd\n");
        exit(1);
    }
    while (fgets(line, 1024, shell_file) != NULL)
    {
        if (strstr(line, user)) {
            (*shell) = line;
            if (strstr((*shell), "bash"))
                (*shell) = "bash";
            if (strstr((*shell), "fish"))
                (*shell) = "fish";
            if (strstr((*shell), "zsh"))
                (*shell) = "zsh";
            if (strstr((*shell), "dash"))
                (*shell) = "dash";
            if (strstr((*shell), "ksh"))
                (*shell) = "ksh";
            if (strstr((*shell), "csh"))
                (*shell) = "csh";
            if (strstr((*shell), "tcsh"))
                (*shell) = "tcsh";
            break;
        }
    }
    fclose(shell_file);
}

void get_hostname(char **hostname)
{
    FILE *hostname_file;
    if ((hostname_file = fopen("/etc/hostname", "r")) == NULL)
    {
        fprintf(stderr, "error opening /etc/hostname\n");
        exit(1);
    }
    fscanf(hostname_file, "%[^\n]", *hostname);
    fclose(hostname_file);
}

void get_user(char **user)
{
    FILE *user_name = popen("whoami", "r");
    fscanf(user_name, "%s", (*user));
    fclose(user_name);
}

void get_cpuinfo(char **cpu_name)
{
    FILE *cpu_name_file;
    char *line = malloc(1024);
    if ((cpu_name_file = fopen("/proc/cpuinfo", "r")) == NULL)
    {
        fprintf(stderr, "error opening /proc/cpuinfo\n");
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
    fclose(cpu_name_file);
}

long num_from_line(FILE *f)
{
    char buff[1024];
    char *p;
    long out = 0;
    fgets(buff, 1024, f);
    p = buff;
    while (*p) {
        if ( isdigit(*p) || ( (*p=='-'||*p=='+') && isdigit(*(p+1))  ) ) {
            out = strtol(p, &p, 10);
        } else {
            p++;
        }}
    return out;
}

void get_raminfo(char **ram_str)
{
    FILE *ram_file;
    long total_long;
    long free_long;
    float ratio;
    if ((ram_file = fopen("/proc/meminfo", "r")) == NULL)
    {
        fprintf(stderr, "error opening /proc/meminfo\n");
        exit(1);
    }
    total_long = num_from_line(ram_file) / 1024;
    free_long = num_from_line(ram_file) / 1024;
    ratio = ((total_long - free_long) / (double) total_long) * 100;
    sprintf(*ram_str, "%ld MiB / %ld MiB (%.1f %%)",
           total_long - free_long, total_long, ratio);
    fclose(ram_file);
}
