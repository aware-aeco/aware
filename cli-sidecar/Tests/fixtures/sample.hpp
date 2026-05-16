// sample.hpp — C++ fixture for AWARE sidecar header tests

#pragma once

#include <string>

namespace Sample {

class Greeter {
public:
    std::string Greet(const std::string& name);
    int Counter();
private:
    int internal_count_;
};

} // namespace Sample

// Free function in C++ mode too
int sample_add_cpp(int a, int b);
