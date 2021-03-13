#include <iostream>

extern "C" {
    char hello(char);
}

int main() {
    char c = 'd';
    std::cout << "test('" << c << "') = " << hello(c) << std::endl;
}

