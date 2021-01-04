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
    char *shell = malloc(sizeof(char) * 100);

    get_os(&os_name);
    get_kernel(&kernel_name);
    get_uptime(&uptime);
    get_user(&user);
    get_cpuinfo(&cpu_name);
    get_shell(&shell, user);

    if (arc == 1){
        fprintf(stdout,red "user" reset ":" "\t\t%s\n",user);
        fprintf(stdout,red "shell" reset ":" "\t\t%s\n",shell);
        fprintf(stdout,red "distro" reset ":" "\t\t%s\n", os_name);
        fprintf(stdout,red "kernel" reset ":" "\t\t%s\n", kernel_name);
        fprintf(stdout,red "uptime" reset ":" "\t\t%s\n", uptime);
        fprintf(stdout,red "cpu" reset ":" "\t\t%s\n", cpu_name);
        printf("\n\t\t%s██%s██%s██%s██%s██%s██%s██%s██%s\n", black, red, green, yellow, blue, magenta, cyan, white, reset);
        printf("%s\t\t%s██%s██%s██%s██%s██%s██%s██%s██%s\n\n", bold, black, red, green, yellow, blue, magenta, cyan, white, reset);
    } else {
        if (arv[1][0] == '-' && arv[1][1] == 'b') {
            fprintf(stdout,"*---------------------------------------------------------------*\n");
            fprintf(stdout,"| " red "user" reset ":" "\t\t%s \t\t\t\t\t|\n",user);
            fprintf(stdout,"| " red "shell" reset ":" "\t%s \t\t\t\t\t\t|\n",shell);
            fprintf(stdout,"| " red "distro" reset ":" "\t%s \t\t\t\t\t\t|\n", os_name);
            fprintf(stdout,"| " red "kernel" reset ":" "\t%s \t|\n", kernel_name);
            fprintf(stdout,"| " red "uptime" reset ":" "\t%s \t\t\t\t|\n", uptime);
            fprintf(stdout,"| " red "cpu" reset ":" "\t\t%s \t\t|\n", cpu_name);
            fprintf(stdout,"*---------------------------------------------------------------*\n");
        } else if ((arv[1][0] == '-' && arv[1][1] == 'h') || (arv[1][0] == '-' && arv[1][1] == '-' && arv[1][2] == 'h')) {
            fprintf(stdout,"-h:\tprints this help message\n"
                    "-b:\tprints the information with a box\n"
                    "-l:\tprints the information with colour on the side\n");
        } else if ((arv[1][0] == '-' && arv[1][1] == 'l')) {
            fprintf(stdout,black "██" bold "██\n" reset);
            fprintf(stdout,red "██" bold "██" reset  red "\tuser" reset ":" "\t\t%s\n",user);
            fprintf(stdout,green "██" bold "██" reset red "\tshell" reset ":" "\t\t%s\n",shell);
            fprintf(stdout,yellow "██" bold "██" reset red "\tdistro" reset ":" "\t\t%s\n", os_name);
            fprintf(stdout,blue "██" bold "██" reset red "\tkernel" reset ":" "\t\t%s\n", kernel_name);
            fprintf(stdout,magenta "██" bold "██" reset red "\tuptime" reset ":" "\t\t%s\n", uptime);
            fprintf(stdout,cyan "██" bold "██" reset red "\tcpu" reset ":" "\t\t%s\n", cpu_name);
            fprintf(stdout,white "██" bold "██\n" reset);
        }
    }
    exit(0);
}

