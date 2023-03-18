#pragma(once)

struct Memory
{
    char *data;
    long long size;
    Memory(int size){   //size以KB为单位
        this->data = new char[size*1024];
        this->size = size;
    }
    void write_byte(long long, char);
    void write_short(long long, short);
    void write_int(long long, int);
    void write_longlong(long long, long long);
    void write_any(long long, void*, long long);
    char read_byte(long long);
    short read_short(long long);
    int read_int(long long);
    long long read_longlong(long long);
};

