#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <string.h>
#include "main.h"

static void clean_values(values_t *all)
{
    if (all->user == NULL) {
        all->user = "(null)";
    } if (all->hostname == NULL) {
        all->hostname = "(null)";
    } if (all->shell == NULL) {
        all->shell = "(null)";
    } if (all->os == NULL) {
        all->os = "(null)";
    } if (all->kernel == NULL) {
        all->kernel = "(null)";
    } if (all->uptime == NULL) {
        all->uptime = "(null)";
    } if (all->cpu_info == NULL) {
        all->cpu_info = "(null)";
    }
}

static void print_all(values_t *all)
{
    printf("%s%s%s@%s%s%s\n\n", red, all->user, reset, red, all->hostname, reset);
    printf("%sDistro%s:\t\t%s\n", red, reset, all->os);
    printf("%sKernel%s:\t\t%s\n", red, reset, all->kernel);
    printf("%sUptime%s:\t\t%s\n", red, reset, all->uptime);
    printf("%sShell%s:\t\t%s\n", red, reset, all->shell);
    printf("%sCpu%s:\t\t%s\n", red, reset, all->cpu_info);
    printf("\n%s██%s██%s██%s██%s██%s██%s██%s██%s\n",
            black, red, green, yellow, blue,
            magenta, cyan, white, reset);
    printf("%s%s██%s██%s██%s██%s██%s██%s██%s██%s\n\n",
            bold, black, red, green, yellow,
            blue, magenta, cyan, white, reset);
}

static void free_all(values_t *all)
{
    free(all->os);
    free(all->kernel);
    free(all->uptime);
    free(all->hostname);
    free(all->user);
    free(all->shell);
    free(all->cpu_info);
}

static void assign_all(values_t *all)
{
    all->user = get_user();
    all->hostname = get_hostname();
    all->shell = get_shell();
    all->os = get_os();
    all->kernel = get_kernel();
    all->uptime = get_uptime();
    all->cpu_info = get_cpuinfo();
}

int main(int argc, char *argv[])
{
    values_t *all;
    enum { normal, help, clear } mode = normal;
    int opt;
    /*getopt does the heavy lifting of enabling command line options for us*/
    /*dont use this program in solaris, supposedly this implementation of getop*/
    /*is insecure on version 5.5*/
    while ((opt = getopt(argc, argv, "hc")) != -1) {
        if (opt == 'h') {
            mode = help;
            break;
        } else if (opt == 'c') {
            mode = clear;
        } else {
            exit(EXIT_FAILURE);
        }
    }
    if (mode == help) {
        fprintf(stdout,"%s", info);
        exit(EXIT_SUCCESS);
    } else if (mode == clear ) {
        printf("\033[2J");
        printf ("\033[H");
    }
    /*everything inside is set to 0, marginal amounts of speed for security*/
    all = calloc(1, sizeof(values_t));
    if (all == NULL) {
        free(all);
        exit(EXIT_FAILURE);
    }

    assign_all(all);
    /*make sure all values have been assigned before trying to print*/
    clean_values(all);
    print_all(all);
    free_all(all);
    free(all);
    exit(EXIT_SUCCESS);
}
