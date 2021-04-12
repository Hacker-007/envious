#include <iostream>

extern "C" {
    char hello(char);
    int test();
}

int main() {
    char c = 'd';
    std::cout << "hello('" << c << "') = " << hello(c) << std::endl;
    std::cout << "test() = " << test() << std::endl;
}

