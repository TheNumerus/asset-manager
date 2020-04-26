#ifndef CONFIG_H
#define CONFIG_H

#include "copydog.h"
namespace copydog {
    class Config{
    public:
        Config(const char* input) {
            c = config_generate(input);
            if (c == nullptr) {
                throw 0;
            }
        }
        ~Config() {
            config_free(c);
            c = nullptr;
        }
        ConfigFfi* get_inner() const {
            return c;
        }
    private:
        ConfigFfi *c;
    };
}

#endif // CONFIG_H
