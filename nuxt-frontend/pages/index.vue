<template>

  <div class="root"> <!-- Main container -->
    <header>
      <h1>Sparse Matrix Solver</h1>
    </header>
  
    <!-- Matrix and Divider Section -->
    <div class="container">
      <!-- Left Side -->
      <div class="matrix-section">
        <div class="matrix-box red-border"></div>
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
  methods: {
    generateMatrix() {
      // TODO: Implement logic to generate a random sparse matrix
      console.log("Generating matrix...");
    },
    solveMatrix() {
      // TODO: Implement logic to solve the matrix
      console.log("Solving matrix...");
    }
  },
  head() {
    return {
      title: 'Sparse Matrix Solver'
    }
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
