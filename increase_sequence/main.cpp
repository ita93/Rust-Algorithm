#include <bits/stdc++>
using namespace std;

vec<int> num_list;

int main(){
  int temp;
  ifstream inFile;

  inFile.open("input.txt");
  if (!inFile) {
    cout<<"Unable to open file";
    exit(1);
  }

  while (inFile >> temp) {
    num_list.push_back(temp);
  }


  inFile.close();
}

