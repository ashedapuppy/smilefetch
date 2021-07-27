#ifndef MAIN_H_
#define MAIN_H_

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

char *get_os(void);
char *get_kernel(void);
char *get_uptime(void);
char *get_shell(void);
char *get_hostname(void);
char *get_user(void);
char *get_cpuinfo(void);

static const char info[] = "smilefetch.\n"
"A neofetch clone made for speed and efficiency.\n\n"
"Usage:\n"
"\tsmilefetch -h\n"
"\tsmilefetch -c\n\n"
"Options:\n"
"\t-h\tShow this screen.\n"
"\t-c\tClear terminal before printing\n";

typedef struct {
    char *os;
    char *kernel;
    char *uptime;
    char *shell;
    char *hostname;
    char *user;
    char *cpu_info;
} values_t;

#endif/* MAIN_H_ */
