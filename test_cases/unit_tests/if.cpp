#include<stdio.h>
#include<stdlib.h>
#include<math.h>
int main() {

    int *b;
    int a = 3;
    if(a==b) return 100;
    b = func(32, a);
    cout<<b;
    cout<<"hello world";

}

int func(int a, int b) {
    return a+b;
}