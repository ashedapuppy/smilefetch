#ifndef MAIN_H_
#define MAIN_H_

#include <stdio.h> // for reading files
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

// from getdata.c
void get_os(char **);
void get_kernel(char **kernel_name);
void get_uptime(char **uptime);
void get_shell(char **shell);
void get_cpuinfo(char **cpu_name);
void get_user(char **user);

#endif /* MAIN_H_ */
