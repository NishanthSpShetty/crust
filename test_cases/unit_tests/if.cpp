#include<stdio.h>
#include<stdlib.h>
#include<math.h>
int main() {
    int a = 3;
    int *b;
    if(a==b) return 100;
    b = func(32, a);
}

int func(int a, int b) {
    return a+b;
}