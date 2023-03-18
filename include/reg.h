#pragma(once)

struct Reg      //寄存器结构体
{
    int size;   //多少位
    char *buf;
    Reg(int bits,char* buffer){
        size = bits;
        buf = buffer;
    }
    void write_byte(int,char);
    char read_byte(int);
    void write(long long);
    long long read();
};

