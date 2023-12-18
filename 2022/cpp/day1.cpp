#include <fstream>
#include <iostream>
#include <vector>
#include <string>
#include <charconv>
using namespace std;

int main()
{
  // relative to where the CWD is when program is run
  const string sample = "../../inputs/sample1.txt";
  const string input = "../../inputs/day1.txt";
  cout << "Welcome to Day 1!\n";

  ifstream in_sample;
  in_sample.open(sample);
  vector<string> lines;
  string line;

  vector<int> carrying;

  while (getline(in_sample, line))
  {
    lines.push_back(line);
  }

  for (const auto &str : lines)
  {
    cout << str << '\n';
    // int num = std::stoi(str);
    int num = 0;
    auto [ptr, ec] = std::from_chars(str.data(), str.data() + str.size(), num);
    std::cout << num << ' ';
  }
  std::cout << "\n";

  return 0;
}
