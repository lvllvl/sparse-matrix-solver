#include "SparseMatrixGenerator.hpp"
#include <iostream>

int main() {
    // Example usage
    int rows = 5;
    int cols = 5;
    double sparsity = 0.1;

    Matrix matrix = generateSparseMatrix(rows, cols, sparsity);

    // Display the matrix (or any other operations you want to perform)
    for (const auto& row : matrix) {
        for (double val : row) {
            std::cout << val << " ";
        }
        std::cout << std::endl;
    }

    return 0;
}
