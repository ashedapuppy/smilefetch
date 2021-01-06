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
    char *hostname = malloc(sizeof(char) * 100);
    char *cpu_name = malloc(sizeof(char) * 100);
    char *ram_str = malloc(sizeof(char) * 100);
    char *shell = malloc(sizeof(char) * 100);

    get_os(&os_name);
    get_kernel(&kernel_name);
    get_uptime(&uptime);
    get_user(&user);
    get_hostname(&hostname);
    get_cpuinfo(&cpu_name);
    get_raminfo(&ram_str);
    get_shell(&shell, user);

    if (arc == 1){
        fprintf(stdout,red "%s" reset "@" red "%s\n\n" reset,user, hostname);
        fprintf(stdout,red "Shell" reset ":" "\t\t%s\n",shell);
        fprintf(stdout,red "Distro" reset ":" "\t\t%s\n", os_name);
        fprintf(stdout,red "Kernel" reset ":" "\t\t%s\n", kernel_name);
        fprintf(stdout,red "Uptime" reset ":" "\t\t%s\n", uptime);
        fprintf(stdout,red "CPU" reset ":" "\t\t%s\n", cpu_name);
        /*fprintf(stdout,red "RAM" reset ":" "\t\t%s\n", ram_str);*/
        printf("\n%s██%s██%s██%s██%s██%s██%s██%s██%s\n", black, red, green, yellow, blue, magenta, cyan, white, reset);
        printf("%s%s██%s██%s██%s██%s██%s██%s██%s██%s\n\n", bold, black, red, green, yellow, blue, magenta, cyan, white, reset);
    } else {
        if (arv[1][0] == '-' && arv[1][1] == 'b') {
            fprintf(stdout,"*---------------------------------------------------------------*\n");
            fprintf(stdout,"| " red "User" reset ":" "\t\t%s \t\t\t\t\t|\n",user);
            fprintf(stdout,"| " red "Shell" reset ":" "\t%s \t\t\t\t\t\t|\n",shell);
            fprintf(stdout,"| " red "Distro" reset ":" "\t%s \t\t\t\t\t\t|\n", os_name);
            fprintf(stdout,"| " red "Kernel" reset ":" "\t%s \t|\n", kernel_name);
            fprintf(stdout,"| " red "Uptime" reset ":" "\t%s \t\t\t|\n", uptime);
            fprintf(stdout,"| " red "CPU" reset ":" "\t\t%s \t\t|\n", cpu_name);
            fprintf(stdout,"*---------------------------------------------------------------*\n");
        } else if ((arv[1][0] == '-' && arv[1][1] == 'h') || (arv[1][0] == '-' && arv[1][1] == '-' && arv[1][2] == 'h')) {
            fprintf(stdout,"-h:\tprints this help message\n"
                    "-b:\tprints the information with a box\n"
                    "-l:\tprints the information with colour on the side\n");
        } else if ((arv[1][0] == '-' && arv[1][1] == 'l')) {
            fprintf(stdout,black "████" bold "████\n" reset);
            fprintf(stdout,red "████" bold "████" reset  red " user" reset ":" "\t\t%s\n",user);
            fprintf(stdout,green "████" bold "████" reset red " shell" reset ":" "\t\t%s\n",shell);
            fprintf(stdout,yellow "████" bold "████" reset red " distro" reset ":" "\t%s\n", os_name);
            fprintf(stdout,blue "████" bold "████" reset red " kernel" reset ":" "\t%s\n", kernel_name);
            fprintf(stdout,magenta "████" bold "████" reset red " uptime" reset ":" "\t%s\n", uptime);
            fprintf(stdout,cyan "████" bold "████" reset red " cpu" reset ":" "\t\t%s\n", cpu_name);
            fprintf(stdout,white "████" bold "████\n" reset);
        }
    }
    exit(0);
}

