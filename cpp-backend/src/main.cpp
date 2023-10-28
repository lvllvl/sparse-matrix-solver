#include "crow.h"
#include "SparseMatrixGenerator.h" // Assuming you have such a file

int main() {
    crow::SimpleApp app;

    CROW_ROUTE(app, "/generateMatrix")
    ([&]() {
        // Generate the sparse matrix
        SparseMatrixGenerator generator; // Assuming you have such a class
        auto matrix = generator.generate();

        // Convert the matrix to JSON and return
        crow::json::wvalue x;
        // Assuming matrix is a 2D vector
        for (int i = 0; i < matrix.size(); i++) {
            for (int j = 0; j < matrix[i].size(); j++) {
                x[i][j] = matrix[i][j];
            }
        }
        return x;
    });

    app.port(4000).multithreaded().run();
}
