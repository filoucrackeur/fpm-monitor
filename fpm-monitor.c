#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <dirent.h>
#include <ctype.h>

#define MAX_POOLS 64
#define MAX_LINE 512

typedef struct {
    char name[128];
    int master_pid;
    int worker_pids[256];
    int worker_count;
    int running;
    int idle;
} pool_info_t;

pool_info_t pools[MAX_POOLS];
int pool_count = 0;

// Lit /proc/[pid]/cmdline et retourne le nom du pool si c'est un worker/master FPM
int parse_fpm_cmdline(int pid, char *pool_name, int *is_master) {
    char path[64], buf[MAX_LINE];
    snprintf(path, sizeof(path), "/proc/%d/cmdline", pid);
    FILE *f = fopen(path, "r");
    if (!f) return 0;

    size_t n = fread(buf, 1, sizeof(buf) - 1, f);
    fclose(f);
    if (n == 0) return 0;
    buf[n] = '\0';

    // cmdline: "php-fpm: pool nginx1" ou "php-fpm: master process (...)"
    if (strstr(buf, "php-fpm:") == NULL) return 0;

    if (strstr(buf, "master process")) {
        *is_master = 1;
        strcpy(pool_name, "master");
        return 1;
    }

    char *p = strstr(buf, "pool ");
    if (p) {
        p += 5;
        strncpy(pool_name, p, 127);
        pool_name[127] = '\0';
        *is_master = 0;
        return 1;
    }
    return 0;
}

// Lit l'état du process (R=running, S=sleeping/idle, D=disk wait, Z=zombie)
char get_proc_state(int pid) {
    char path[64], buf[256];
    snprintf(path, sizeof(path), "/proc/%d/stat", pid);
    FILE *f = fopen(path, "r");
    if (!f) return '?';
    fgets(buf, sizeof(buf), f);
    fclose(f);

    // format: pid (comm) state ...
    char *paren = strrchr(buf, ')');
    if (!paren) return '?';
    return paren[2]; // après ") "
}

// Récupère la mémoire RSS en Ko
long get_proc_rss_kb(int pid) {
    char path[64], line[256];
    snprintf(path, sizeof(path), "/proc/%d/status", pid);
    FILE *f = fopen(path, "r");
    if (!f) return -1;
    long rss = -1;
    while (fgets(line, sizeof(line), f)) {
        if (strncmp(line, "VmRSS:", 6) == 0) {
            sscanf(line, "VmRSS: %ld kB", &rss);
            break;
        }
    }
    fclose(f);
    return rss;
}

pool_info_t* find_or_create_pool(const char *name) {
    for (int i = 0; i < pool_count; i++) {
        if (strcmp(pools[i].name, name) == 0) return &pools[i];
    }
    pool_info_t *p = &pools[pool_count++];
    memset(p, 0, sizeof(pool_info_t));
    strncpy(p->name, name, 127);
    return p;
}

int main() {
    DIR *proc = opendir("/proc");
    if (!proc) { perror("opendir /proc"); return 1; }

    struct dirent *entry;
    while ((entry = readdir(proc)) != NULL) {
        if (!isdigit(entry->d_name[0])) continue;
        int pid = atoi(entry->d_name);

        char pool_name[128];
        int is_master;
        if (!parse_fpm_cmdline(pid, pool_name, &is_master)) continue;
        if (is_master) continue; // on ignore le master lui-même pour le comptage workers

        pool_info_t *pool = find_or_create_pool(pool_name);
        pool->worker_pids[pool->worker_count++] = pid;

        char state = get_proc_state(pid);
        if (state == 'R') pool->running++;
        else if (state == 'S') pool->idle++;
    }
    closedir(proc);

    // Affichage générique, tous pools détectés automatiquement
    printf("%-20s %-10s %-10s %-10s\n", "POOL", "WORKERS", "RUNNING", "IDLE");
    for (int i = 0; i < pool_count; i++) {
        pool_info_t *p = &pools[i];
        printf("%-20s %-10d %-10d %-10d\n",
               p->name, p->worker_count, p->running, p->idle);

        for (int j = 0; j < p->worker_count; j++) {
            int pid = p->worker_pids[j];
            long rss = get_proc_rss_kb(pid);
            char state = get_proc_state(pid);
            printf("  └─ pid=%-8d state=%-3c rss=%ldKo\n", pid, state, rss);
        }
    }

    return 0;
}
