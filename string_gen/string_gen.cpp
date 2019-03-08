#include <bits/stdc++.h>
using namespace std;

void print_res(vector<int> idx_list, string word){
  for (int i=0; i < idx_list.size(); i++){
    int idx = idx_list[i];
    cout<<word[idx];
  }

  cout<<endl;
}

void try_gen(vector<int> idx_list, string word, vector<bool>checker, int k){
  int length = idx_list.size();
  for (int i=0; i<length; i++){
    if (checker[i] == true){
      idx_list[k] = i;
      if (k == length-1){
        print_res(idx_list, word);
      }else{
        checker[i] = false;
        try_gen(idx_list, word, checker,k + 1);
        checker[i] = true;
      }
    }
  }
}

int main(){
  string seed_str;
  ifstream inFile;

  inFile.open("input.txt");
  if (!inFile) {
    cout<<"Unable to open file";
    exit(1);
  }

  inFile >> seed_str;
  vector<int> idx_list(seed_str.size());
  vector<bool> checker(seed_str.size(), true);
  try_gen(idx_list, seed_str,checker, 0);

  inFile.close();
  return 0;
}
