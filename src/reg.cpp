#include <reg.h>
#include <cstring>

void Reg::write_byte(int byte,char value){
    if (byte < 0 || byte >= this->size){
        return;
    }
    this->buf[byte] = value;
}

char Reg::read_byte(int byte){
    if (byte < 0 || byte >= this->size){
        return 0;
    }
    return this->buf[byte];
}

void Reg::write(long long value){
    memcpy(this->buf,&value,this->size);
}

long long Reg::read(){
    long long value=0;
    memcpy(&value,this->buf,this->size);
    return value;
}

