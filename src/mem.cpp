#include <mem.h>
#include <cstring>

void Memory::write_byte(long long addr,char bytes){
    memcpy(this->data+addr,&bytes,sizeof(bytes));
}

void Memory::write_short(long long addr,short bytes){
    memcpy(this->data+addr,&bytes,sizeof(bytes));
}

void Memory::write_int(long long addr,int bytes){
    memcpy(this->data+addr,&bytes,sizeof(bytes));
}

void Memory::write_longlong(long long addr,long long bytes){
    memcpy(this->data+addr,&bytes,sizeof(bytes));
}

void Memory::write_any(long long addr,void * bytes,long long size){
    memcpy(this->data+addr,bytes,size);
}

char Memory::read_byte(long long addr){
    char bytes;
    memcpy(&bytes,this->data+addr,sizeof(bytes));
    return bytes;
}

short Memory::read_short(long long addr){
    short bytes;
    memcpy(&bytes,this->data+addr,sizeof(bytes));
    return bytes;
}

int Memory::read_int(long long addr){
    int bytes;
    memcpy(&bytes,this->data+addr,sizeof(bytes));
    return bytes;
}

long long Memory::read_longlong(long long addr){
    long long bytes;
    memcpy(&bytes,this->data+addr,sizeof(bytes));
    return bytes;
}
