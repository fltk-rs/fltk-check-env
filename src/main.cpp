#include <string>
#include <iostream>

int main() {
    auto s = std::string("Hello"); // checks for C++ std and C++ 11
    std::cout << s << std::endl;
    return (int)s.length();
}