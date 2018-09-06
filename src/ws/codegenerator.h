#include <string>

class WS_BaseGenerator
{
public:
    void pushAdd(std::string, std::string, std::string);
    std::string getString();
private:
    std::string m_sString;

    void push(std::string);
    void newLine();
};