#include <stdio.h>
#include <stdlib.h>
#include <sys/mount.h>
#include <unistd.h>

int mount_sysfs()
{
    if (0 != mount("none", "/sys", "sysfs", 0, "")) {
        /* handle error */
        printf("mount sysfs error.\n");
        return -1;
    }

    return 0;
}

int mount_procfs()
{
    if (0 != mount("none", "/proc", "proc", 0, "")) {
        /* handle error */
        printf("mount procfs error.\n");
        return -1;
    }

    return 0;
}

void print_file(const char *filename)
{
    int c;
    FILE *file;
    file = fopen(filename, "r");
    if (file) {
        while ((c = getc(file)) != EOF)
            putchar(c);
        fclose(file);
    }
}

int main(int argc, char *argv[])
{
    mount_procfs();
    mount_sysfs();

    int count = 0;
    for (;;) {
        count++;
        printf(".");
        if (count % 5 == 0) {
            printf("\n");
            print_file("/proc/interrupts");
        }
        sleep(1);
    }

    return 0;
}
