/*
 * SniffnetLauncher — signed CFBundleExecutable for the notarized macOS app.
 *
 * This is a Mach-O equivalent of wrapper.sh: it asks for administrator
 * privileges through osascript, then starts the sniffnet binary that sits
 * next to it in Contents/MacOS. A shell script cannot be the main
 * executable of a notarized app, which is why this exists.
 *
 * The AppleScript prompt text and the "run sniffnet in the background as
 * root" behaviour are intentionally the same as wrapper.sh.
 */

#include <libgen.h>
#include <limits.h>
#include <mach-o/dyld.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

int main(void) {
    char raw[PATH_MAX];
    uint32_t size = sizeof(raw);
    if (_NSGetExecutablePath(raw, &size) != 0) {
        fprintf(stderr, "SniffnetLauncher: could not resolve executable path\n");
        return 1;
    }

    char resolved[PATH_MAX];
    if (realpath(raw, resolved) == NULL) {
        perror("SniffnetLauncher: realpath");
        return 1;
    }

    char dirbuf[PATH_MAX];
    strncpy(dirbuf, resolved, sizeof(dirbuf) - 1);
    dirbuf[sizeof(dirbuf) - 1] = '\0';
    char *dir = dirname(dirbuf);

    char sniffnet[PATH_MAX];
    int n = snprintf(sniffnet, sizeof(sniffnet), "%s/sniffnet", dir);
    if (n < 0 || (size_t)n >= sizeof(sniffnet)) {
        fprintf(stderr, "SniffnetLauncher: path too long\n");
        return 1;
    }
    if (access(sniffnet, X_OK) != 0) {
        perror("SniffnetLauncher: sniffnet binary");
        return 1;
    }

    /* Same quoting as wrapper.sh: the path is wrapped in single quotes
       inside the shell command. The official install path does not
       contain single quotes. */
    char script[PATH_MAX * 2];
    n = snprintf(
        script,
        sizeof(script),
        "do shell script \"'%s' >/dev/null 2>&1 &\" with prompt "
        "\"Comfortably monitor your network traffic.\" with administrator "
        "privileges",
        sniffnet);
    if (n < 0 || (size_t)n >= sizeof(script)) {
        fprintf(stderr, "SniffnetLauncher: AppleScript too long\n");
        return 1;
    }

    char *args[] = {"osascript", "-e", script, NULL};
    execvp("/usr/bin/osascript", args);
    perror("SniffnetLauncher: exec osascript");
    return 1;
}
