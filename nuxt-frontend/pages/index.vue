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
    <!-- Displaying the generated sparse matrix -->
    <div v-if="sparseMatrix.length">
      <table>
        <tr v-for="row in sparseMatrix" :key="row">
          <td v-for="cell in row" :key="cell">{{ cell }}</td>
        </tr>
      </table>
    </div>
    <div v-else-if="error">
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
      error: null // To handle any errors during the API request
    };
  },
  methods: {
    async generateMatrix() {
      try {
        const response = await fetch('http://localhost:3000/generateMatrix');
        if (!response.ok) {
          throw new Error('Network response was not ok');
        }
        const matrix = await response.json(); // Assuming the backend returns the matrix as a JSON
        this.sparseMatrix = matrix;
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
  border-collapse: collapse;
}

table, th, td {
  border: 1px solid black;
}

th, td {
  padding: 8px 12px;
  text-align: center;
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
