#pragma(once)

#include <reg.h>

#define CPU_BITS 64

struct CPU {    //CPU结构体
    Reg al,ah,bl,bh,cl,ch,dl,dh;    //8byte
    Reg ax,bx,cx,dx;                //8byte
    Reg eax,ebx,ecx,edx;            //16byte
    Reg cr0;                        //24byte
    Reg rflags;                     //32byte
    Reg rax,rbx,rcx,rdx,rsp,rip;    //64byte
    char buffer[CPU_BITS];  //CPU有多少位就有多少位储存
};
