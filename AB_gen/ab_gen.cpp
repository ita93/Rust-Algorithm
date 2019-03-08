#include <bits/stdc++.h>

using namespace std;

//Global variable here
vector<char> res_str;
int n;

void print_vec(){
  for (int i=0; i<n; i++){
    cout<<res_str[i];
  }
  cout<<endl;
}

void try_gen(int k){
  bool can_b = (k>0 && res_str[k-1] == 'B')?false:true;
  res_str[k]='A';
  if (k == n-1)
    print_vec();
  else{
    try_gen(k+1);
  }
  //for B
  if (can_b){
    res_str[k] = 'B';
    if (k == n-1){
      print_vec();
    } else {
      try_gen(k+1);
    }
  }
}

int main(){
  cout<<"Enter n: ";
  cin>>n;

  res_str.resize(n);

  try_gen(0);
}
