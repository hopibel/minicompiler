#include <iostream>
#include <string>

using namespace std;

#include "vm.h"

void showVMRes(Optional<int> r) {
  if(r.isNothing())
    cout << "\nVM stack (top): empty";

  cout << "\nVM stack (top):" << r.fromJust();
}

void testVM() {
  {
    vector<Code> vc{
      newPush(1),
      newPush(2),
      newPush(3),
      newMult(),
      newPlus() };

    Optional<int> res = VM(vc).run();

    showVMRes(res);
  }

  {
    vector<Code> vc{
      newPush(2),
      newPush(3),
      newPush(5),
      newPlus(),
      newMult() };

    Optional<int> res = VM(vc).run();

    showVMRes(res);
  }
}


int main() {
  testVM();

  return 1;
}