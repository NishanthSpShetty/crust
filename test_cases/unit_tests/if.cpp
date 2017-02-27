int main() {
    int a[3] = {5,4,65};
    int b = 34;
    b = a[2];
    b = func() * a[1] + 234 + func();
}

int func() {
    return 0;
}