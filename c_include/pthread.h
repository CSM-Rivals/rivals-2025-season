#ifndef _PTHREAD_H
#define _PTHREAD_H

typedef int pthread_mutex_t;
typedef int pthread_t;
typedef int pthread_attr_t;
typedef int pthread_cond_t;

#define pthread_mutex_init(m, a) 0
#define pthread_mutex_lock(m) 0
#define pthread_mutex_unlock(m) 0
#define pthread_mutex_destroy(m) 0
#define pthread_create(t, a, f, arg) 0
#define pthread_join(t, r) 0
#define pthread_cond_init(c, a) 0
#define pthread_cond_wait(c, m) 0
#define pthread_cond_broadcast(c) 0
#define pthread_cond_destroy(c) 0
#define pthread_yield() 0

#endif