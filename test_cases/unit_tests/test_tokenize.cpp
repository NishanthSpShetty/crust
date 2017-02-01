protected class SomeClassName
{
  public:
    SomeClassName()
    {
    }
    static int a;
};

int main()
{
    /*printf("hello world");
this is C ..
so */
    //let write some c++
    cout << "hello \\ \t \r \f \b \" world\n"
	 << endl;
    float a = 100.123 + 100;
	double b = 122.0253553 * 645.7689 / 346;
	long c = 5999999;
	bool d = false || true;
	unsigned short short1 = 4;
	unsigned short short2 = (short1 << 1) >> 2;
    if (a == 100 && b == 10)
	cout << "i dont know";

	char e = 'c';
    e = '\n';
    e = '\'';

	switch (a){
		case '\n' : do_something();
				break;
		default:
			    do_the_same_damn_thing();
	}

	while(){
		continue;
	}
	do{}while(1);

}