class A{
    int a;
    int b;

  private:  float getfloat(){ return 1.23; }

    public : int getInt(int a){ return a; }

    A() {
      a = 5;
      b = 6;
    }
};

int main(){
  A a = A();
}

