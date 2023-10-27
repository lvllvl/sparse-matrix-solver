<template>
  <div class="container">
    <!-- Left Side -->
    <div class="matrix-section">
      <div class="matrix-box red-border"></div>
      <button @click="generateMatrix">Generate Sparse Matrix</button>
    </div>

    <!-- Right Side -->
    <div class="matrix-section">
      <div class="matrix-box blue-border"></div>
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
  }
};
</script>


<style scoped>
.container {
  display: flex;
  justify-content: space-between;
  height: 100vh;
  align-items: center;
  padding: 0 10%;
}

.matrix-section {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
}

.matrix-box {
  width: 80%;
  height: 33vh;
  margin-bottom: 20px;
}

.red-border {
  border: 2px solid red;
}

.blue-border {
  border: 2px solid blue;
}

button {
  padding: 10px 20px;
  font-size: 16px;
}
</style>
