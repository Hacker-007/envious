#include <iostream>

extern "C" {
    int test(int);
}

int main() {
    std::cout << "test: " << test(1) << std::endl;
}