#include "include/main.h"

int main(int arc, char **arv)
{
    char *os_name = malloc(sizeof(char) * 100);

    get_os(&os_name);
    fprintf(stdout,"%s\n", os_name);
    return 0;
}

