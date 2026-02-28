#ifndef _SCHED_H
#define _SCHED_H

#include <windows.h>

// Map the Linux-style yield to the Windows-style yield
static inline int sched_yield(void) {
    return SwitchToThread() ? 0 : -1;
}

#endif