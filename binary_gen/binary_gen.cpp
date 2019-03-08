#include <bits/stdc++.h>
using namespace std;

void print_binary(vector<int> bin){
  for (int i = 0; i < bin.size(); i++) {
    cout<<bin[i];
  }
  cout<<endl;
}

//We are generating digit ith
void try_gen(vector<int> bin, int i){
  for (int j=0; j<=1; j++){
    bin[i] = j;
    if (i == bin.size() - 1) {
      print_binary(bin);
    } else {
      try_gen(bin, i + 1);
    }
  }
}

int main(){
  int n;
  cout << "Enter the length of binary number: ";
  cin>>n;
  vector<int> temp(n);
  try_gen(temp, 0);
  return 0;
}
