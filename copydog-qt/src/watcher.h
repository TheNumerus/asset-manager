#ifndef WATCHER_H
#define WATCHER_H

#include "copydog.h"
#include "config.h"

namespace copydog {
    class Watcher{
    public:
        Watcher(Config &c) {
            w = watcher_new(c.get_inner());
            if (w == nullptr) {
                throw 0;
            }
            watching = false;
        }
        ~Watcher() {
            if (watching) {
                stop();
            }
            watcher_free(w);
            w = nullptr;
        }
        bool start() {
            watching = true;
            return watcher_start(w);
        }
        bool stop() {
            if (watching) {
                watching = false;
                return watcher_stop(w);
            }
            return false;
        }
    private:
        WatcherFfi *w;
        bool watching;
    };
}

#endif // WATCHER_H
