template <typename T>
T add(T a, T b) {
    return a + b;
}
int main() {
    // 1. Instantiation with int
    int x = add<int>(5, 3);
    // 2. Instantiation with double
    double y = add<double>(5.5, 3.2);
    return 0;
}