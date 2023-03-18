#include <cstdio>
#include <mem.h>
using namespace std;

int main(){
    char str[] = "Hello Memory!";
    long long str_len = sizeof(str)/sizeof(char);
    Memory mem(1);
    mem.write_any(0,str,str_len);
    for (long long i=0;i<str_len;i++){
        printf("%c",mem.read_byte(i));
    }
    printf("\n");
    return 0;
}

