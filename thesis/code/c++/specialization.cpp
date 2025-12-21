template <typename T>
void myMethod(T value) {
    cout << "Generic implementation" << endl;
}

template <>
void myMethod<int>(int value) {
    cout << "Specialized for int" << endl;
}

int main() {
    myMethod(10);       // prints "Specialized for int"
    myMethod(10.5);     // prints "Generic implementation"
    return 0;
}