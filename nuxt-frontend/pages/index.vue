<template>

  <div class="root"> <!-- Main container -->
    <header>
      <h1>Sparse Matrix Solver</h1>
    </header>
  
    <!-- Matrix and Divider Section -->
    <div class="container">

       <!-- Left Side -->
    <div class="matrix-section">
      <div class="matrix-box red-border">
        <!-- Displaying the generated sparse matrix using MathJax -->
        <div v-html="matrixLatex"></div>
        <div v-if="error">
          Error: {{ error }}
        </div>
      </div>
    </div>



      <div class="divider"></div>

      <!-- Right Side -->
      <div class="matrix-section">
        <div class="matrix-box blue-border"></div>
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
      matrixLatex: '', // This will store the LaTex representation of the matrix 
      error: null // To handle any errors during the API request
    };
  },
  computed: {
    formattedMatrixLatex() {
          let latex = '\\begin{bmatrix}'; // Start of matrix in LaTeX
          for ( let row of this.sparseMatrix ) {
            latex += row.join(' & ') + ' \\\\'; 
          }
          latex += '\\end{bmatrix}'; // End of matrix in LaTeX
          return `$${latex}$`; // Wrap with `$` delimiters
          }
    },
  methods: {
    async generateMatrix() {
      try {
        const response = await fetch('http://localhost:8000/generateMatrix');
        if (!response.ok) {
          const text = await response.text(); // Get response as text
          console.log( "Server Response: ", text ); // Log it ! 
          throw new Error('Network response was not ok: ${text}');
        }
        const data = await response.json(); // Assuming the backend returns the matrix as a JSON
        this.sparseMatrix = data.matrix;
        this.matrixLatex = this.formattedMatrixLatex; // Update the LaTex representation of the matrix
        console.log( this.matrixLatex ); // Log the LaTex representation of the matrix

        // Update MathJax rendering 
        if ( window.MathJax && typeof window.MathJax.typesetPromise === 'function' ){
          await window.MathJax.typesetPromise(); // Force mathJax to update
        }
      } catch (error) {
        this.error = "Failed to generate matrix: " + error.message;
      }
    },
    solveMatrix() {
      // TODO: Implement logic to solve the matrix
      console.log("Solving matrix...");
      }
   },
  head() {
    return {
      title: 'Sparse Matrix Solver'
    };
  }
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

table, th, td {
  border: 1px solid black;
}

th, td {
  padding: 4px 8px;
  text-align: center;
  font-size: 0.8em; /* Smaller font size for larger matrices */ 
}

.blue-border {
  border: 2px solid blue;
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
  align-items: center;
  width: 100%; /* makes sure container takes full width available */ 
  padding: 2% 10%;
}

.divider {
  width: 2px; /* width of the vertical line */
  height: 45vw; 
  background-color: gray; /* color of the vertical line */
  margin: 0 10px; /* . some spacing between the divider and the boxes */
}

.matrix-section {
  flex: 1; /* it will take equal space as its sibling */
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 50vw; 
}

.matrix-box {
  width: 40vw; /* this ensures the box is a square */
  height: 40vw; /* this ensures the box is a square */
  margin-bottom: 20px;
  overflow: auto; /* makes the content scrollable if it overflows */
}

.red-border {
  border: 2px solid red;
}

.root {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 1% 5%;
}



</style>
