#include <iostream>

using namespace std;

int main()
{
    int x = 10;
    int y = 20;
    int &r = x;
    r = y;
    // cout << "x = " << x << ", r = " << r << endl;
    cout << "x = " << x << ", y = " << y << ", r = " << r << endl;
}
