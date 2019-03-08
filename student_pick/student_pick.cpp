#include<bits/stdc++.h> 

using namespace std;

void print_vec(vector<string> student, vector<int> idxs){
  for (int i = 0; i< idxs.size(); i++){
    int std_idx = idxs[i];
    cout<<student[std_idx]<<" ";
  }
  cout<<endl;
}

void student_pick(const vector<string>& students, vector<int> res, int i, int k, int n){
  int pivot = (i == 0)?-1:res[i-1];
  for (int j = pivot + 1; j < n - k + i + 1; j++){
    res[i] = j;
    if (i == k-1){
      print_vec(students, res);
    } else {
      student_pick(students, res, i+1, k, n);
    }
  }
}

int main(){
  int k;
  vector<string> students;
  string read_str;
  ifstream inFile;

  inFile.open("input.txt");
  if (!inFile) {
    cout<<"Unable to open file";
    exit(1);
  }
  
  inFile>>k;

  while (inFile >> read_str) {
    students.push_back(read_str);
  }

  vector<int> res(k);

  student_pick(students, res, 0, k, students.size());

  inFile.close();
}
