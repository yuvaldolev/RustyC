int ret3() asm("ret3");

int ret5() asm("ret5");

int add(int x, int y) asm("add");

int sub(int x, int y) asm("sub");

int add6(int a, int b, int c, int d, int e, int f) asm("add6");

int sub8(int a, int b, int c, int d, int e, int f, int g, int h) asm("sub8");

int ret3() { return 3; }

int ret5() { return 5; }

int add(int x, int y) { return x + y; }

int sub(int x, int y) { return x - y; }

int add6(int a, int b, int c, int d, int e, int f) {
  return a + b + c + d + e + f;
}

int sub8(int a, int b, int c, int d, int e, int f, int g, int h) {
  return a - b - c - d - e - f - g - h;
}
