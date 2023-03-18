#include <iostream>
#include <reg.h>
using namespace std;

int main(){
    char buf[8] = {0};
    Reg r(8,buf);
    r.write(10);
    printf("%lld\n",r.read());
    return 0;
}

