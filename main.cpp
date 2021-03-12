#include <iostream>

extern "C" {
    int add_two(int);
}

int main() {
    std::cout << "1 + 2 = " << add_two(1) << std::endl;
}

