#define _GNU_SOURCE
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include "main.h"

char *get_os(void)
{
    char *os;
    char *head = "PRETTY_NAME=";
    char *line_buf = NULL;
    size_t line_buf_size = 0;
    size_t line_size;
    FILE *fp = fopen("/etc/os-release", "r");
    if (!fp)
        return NULL;
    line_size = getline(&line_buf, &line_buf_size, fp);
    while ( line_size != 0 && !strstr(line_buf, head)) {
        line_size = getline(&line_buf, &line_buf_size, fp);
    }
    strtok(line_buf, "\"");
    os = strdup(strtok(NULL, "\""));
    free(line_buf);
    fclose(fp);
    return os;
}

char *get_kernel(void)
{
    char *kernel;
    char *line_buf = NULL;
    size_t line_buf_size = 0;
    FILE *fp = fopen("/proc/version", "r");
    if (!fp)
        return NULL;
    getline(&line_buf, &line_buf_size, fp);
    strtok(line_buf, " ");
    strtok(NULL, " ");
    kernel = strdup(strtok(NULL, " "));
    free(line_buf);
    fclose(fp);
    return kernel;
}

char *get_uptime(void)
{
    int days;
    int hours;
    int minutes;
    int seconds;
    char *uptime = malloc(30);
    char *line_buf = NULL;
    size_t line_buf_size = 0;
    FILE *fp = fopen("/proc/uptime", "r");
    if (!fp) {
        free(uptime);
        return NULL;
    }
    getline(&line_buf, &line_buf_size, fp);
    seconds = atoi(strtok(line_buf, " "));
    free(line_buf);
    fclose(fp);
    days = seconds / (24 * 3600);
    seconds %= (24 * 3600);
    hours = seconds / 3600;
    seconds %= 3600;
    minutes = seconds / 60;
    snprintf(uptime, 30, "%d Days %d Hours %d Minutes", days, hours, minutes);
    return uptime;
}

char *get_shell(void)
{
    char *shell;
    shell = strdup(strrchr(getenv("SHELL"), '/') + 1);
    return shell;
}

char *get_hostname(void)
{
    char *hostname;
    char *line_buf = NULL;
    size_t line_buf_size = 0;
    FILE *fp = fopen("/etc/hostname", "r");
    if (!fp)
        return NULL;
    getline(&line_buf, &line_buf_size, fp);
    hostname = strdup(strtok(line_buf, "\n"));
    free(line_buf);
    fclose(fp);
    return hostname;
}

char *get_user(void)
{
    /*find something better than whoami*/
    char *user;
    user = strdup(getenv("USERNAME"));
    return user;
}

char *get_cpuinfo(void)
{
    char *cpu;
    char *head = "model name";
    char *line_buf = NULL;
    size_t line_buf_size = 0;
    size_t line_size;
    FILE *fp = fopen("/proc/cpuinfo", "r");
    if (!fp)
        return NULL;
    line_size = getline(&line_buf, &line_buf_size, fp);
    while ( line_size != 0 && !strstr(line_buf, head)) {
        line_size = getline(&line_buf, &line_buf_size, fp);
    }
    strtok(line_buf, ":");
    cpu = strdup(strtok(NULL, "\n") + 1);
    free(line_buf);
    fclose(fp);
    return cpu;
}

char *get_raminfo(void)
{
    /*/proc/meminfo*/
    return NULL;
}
