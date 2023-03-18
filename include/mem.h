#pragma(once)

struct Memory
{
    char *data;
    long long size;
    Memory(int size){
        this->data = new char[size];
        this->size = size;
    }
    void write_byte(long long, char);
    void write_short(long long, short);
    void write_int(long long, int);
    void write_longlong(long long, long long);
    void read_byte(long long, char);
    void read_short(long long, short);
    void read_int(long long, int);
    void read_longlong(long long, long long);
};

