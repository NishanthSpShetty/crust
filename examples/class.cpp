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

class B {
int aa,bb;
};

class Address {
	int id;
char name;
	char postal;
	int pin;
};

int main(){
  B b = B();
	A a = A();
  a.getInt();
  a.getfloat();
 
 Address add = Address();
  
  //do some thing else
}

