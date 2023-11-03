<template>
  <div class="root">
    <!-- Main container -->
    <header>
      <h1>Sparse Matrix Solver</h1>
    </header>

    <!-- Matrix and Divider Section -->
    <div class="container">
      <!-- Left Side -->
      <div class="matrix-section">
        <!-- Content of the left matrix box -->
        <div class="matrix-box red-border">
          <!-- Displaying the generated sparse matrix using MathJax -->
          <div v-html="matrixLatex"></div>
          <div v-if="error">Error: {{ error }}</div>
        </div>
      </div>

      <div class="divider"></div>

      <!-- Right Side -->
      <div class="matrix-section">
        <!-- Content of the right matrix box -->
        <div class="matrix-box blue-border">
          <!-- Displaying the solution matrix using MathJax-->
          <div v-html="solutionLatex"></div>
          <div v-if="error">Error: {{ error }}</div>
        </div>
      </div>
    </div>

    <!-- Buttons Section -->
    <div class="buttons-container">
      <button @click="generateMatrix">Generate Sparse Matrix</button>
      <button @click="solveMatrix">Solve Matrix</button>
    </div>
  </div>
</template>




<script>
export default {
  data() {
    return {
      sparseMatrix: [], // This will store the generated sparse matrix
      matrixLatex: "", // This will store the LaTex representation of the matrix
      error: null, // To handle any errors during the API request
      solution: null, // store the solution
      solutionLatex: "", // store the LaTeX representation of the solution
    };
  },
  computed: {
    formattedMatrixLatex() {
      let latex = "\\begin{bmatrix}"; // Start of matrix in LaTeX
      for (let row of this.sparseMatrix) {
        latex += row.join(" & ") + " \\\\";
      }
      latex += "\\end{bmatrix}"; // End of matrix in LaTeX
      return `$${latex}$`; // Wrap with `$` delimiters
    },
  },
  methods: {
    async generateMatrix() {
      this.error = null; // Clear previous errors, if any
      this.sparseMatrix = []; // Clear the previous matrix, if any
      try {
        const response = await fetch("http://localhost:8000/generateMatrix", {
          method: "POST",
          headers: {
            "Cache-Control": "no-cache", // Add this line to prevent caching
          },
        });
        console.log("Raw Response: ", response);

        if (response.ok) {
          const data = await response.json();
          this.sparseMatrix = data.matrix;
          this.matrixLatex = this.formattedMatrixLatex;
          console.log(this.matrixLatex);
          if (
            window.MathJax &&
            typeof window.MathJax.typesetPromise === "function"
          ) {
            await window.MathJax.typesetPromise();
          }
        } else {
          const text = await response.text();
          console.log("Server Response: ", text);
          throw new Error(`Network response was not ok: ${text}`);
        }
      } catch (error) {
        console.error("Failed to generate matrix: ", error.message);
        this.error = "Failed to generate matrix: " + error.message;
      }
    },
    async solveMatrix() {
      try {
        // First, solve the matrix
        await fetch("http://localhost:8000/solveMatrix", { method: "POST" });

        // Then, get the solution
        const response = await fetch("http://localhost:8000/getSolution");
        if (!response.ok) {
          throw new Error("Failed to retrieve solution");
        }

        const data = await response.json();
        this.solution = data.solution;
        console.log("Solution: ", this.solution);

        // Here you would update the UI to display the solution
        // For example, if you had a 'solutionLatex' data property, you could format the solution as LaTeX
        this.solutionLatex = this.formatSolutionToLatex(this.solution);

        // Update MathJax rendering
        if (
          window.MathJax &&
          typeof window.MathJax.typesetPromise === "function"
        ) {
          await window.MathJax.typesetPromise(); // Force MathJax to update
        }
      } catch (error) {
        console.error("Failed to solve matrix: ", error.message);
      }
    },

    // Add this new method to format the solution as LaTeX
    formatSolutionToLatex(solution) {
      if (!solution) return "";
      let latex = solution.map((value) => value.toFixed(2)).join(" & ");
      return `\\begin{bmatrix}${latex}\\end{bmatrix}`;
    },
  },
  head() {
    return {
      title: "Sparse Matrix Solver",
    };
  },
};
</script>

<style scoped>
button {
  padding: 10px 20px;
  font-size: 16px;
  margin: 0 5px;
}

header {
  text-align: center;
  margin-bottom: 1px;
}

h1 {
  font-size: 32px;
  color: #333;
}

table {
  width: 100%;
  max-height: 38vw;
  border-collapse: collapse;
  overflow: hidden; /* hides any content that overflows the table */
}

table,
th,
td {
  border: 1px solid black;
}

th,
td {
  padding: 4px 8px;
  text-align: center;
  font-size: 0.8em; /* Smaller font size for larger matrices */
}

.blue-border {
  border: none;
  /* border: 2px solid blue; */
}

.buttons-container {
  display: flex;
  justify-content: center;
  width: 100%;
  max-width: 80%; /* Adjust as per design needs */
  margin-top: 5px;
}

.container {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  /* width: 100%; makes sure container takes full width available */
  padding: 2% 10%;
}

.divider {
  width: 2px; /* width of the vertical line */
  height: 45vw;
  background-color: gray; /* color of the vertical line */
  margin: 0 10px; /* . some spacing between the divider and the boxes */
}

.matrix-box {
  max-width: 45vw; /* this ensures the box is a square */
  max-height: 85vw; /* this ensures the box is a square */
  font-size: 1.0em;
  overflow: auto;
  margin-bottom: 20px;
}

.matrix-section {
  flex: 1;
  padding: 5px;
}

.red-border {
  border: none;
  /*border: 2px solid red;*/
}

.root {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 1% 5%;
}
</style>
