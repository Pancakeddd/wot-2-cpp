#include <iostream>
#include "ws/codegenerator.h"

int main(int argc, char **argv)
{
    // TODO: Actual Code
    WS_BaseGenerator a = WS_BaseGenerator();
    a.pushAdd("test", "1", "5");
    std::cout << a.getString();
    return 0;
}
