#pragma once

#include <vector>

// You can use a typedef or 'using' directive for ease of readability
using Matrix = std::vector<std::vector<double>>;

// Function declaration
Matrix generateSparseMatrix(int m, int n, double s);
