#include <iostream>

extern "C" {
    int hello(int);
    int test(int);
}

int main() {
    int i = 1;
    std::cout << "hello(" << i << ") = " << hello(i) << std::endl;
    std::cout << "test(" << i << ") = " << test(i) << std::endl;
}

