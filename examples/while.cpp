int main() {
  int i = 0;

  while (i < 100) {

    // do something here
    if (i % 2 == 0)
      continue;
    else
      // should  replace this call with rust equivalent printer func
      print_it(i);
    i++;
  }
}
