#include "codegenerator.h"

/*
*   WS_BaseGenerator::push(std::string) Appends m_sString with a another std::string.
*/

void WS_BaseGenerator::push(std::string str)
{
    m_sString += str;
}

/*
*   WS_BaseGenerator::newln(std::string) Appends m_sString with a newline ("\n").
*/

void WS_BaseGenerator::newLine()
{
    m_sString += "\n";
}

/*
*   WS_BaseGenerator::newln(std::string) Pushes add operator.
*/

void WS_BaseGenerator::pushAdd(std::string a, std::string b, std::string c)
{
    push("int " + a + " = " + b + " + " + c);
    newLine();
}

std::string WS_BaseGenerator::getString()
{
    return m_sString;
}