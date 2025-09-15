<script lang="ts">
import wsl_logo from "$lib/Logo_WSL.svg";
</script>

<header class="mb-4">
  <h1 class="display-3 mb-0">Liebende Totholzapp</h1>
  <span>
    <p class="mb-0 me-2">Made by and for:</p>
    <a href="https://wsl.ch/" target="_blank">
      <img
        class="img-fluid mw-100 col justify-content-end"
        src={wsl_logo}
        alt="WSL Logo"
        style="/*height: 3rem;*/"
      />
    </a>
  </span>
</header>

<style>
header {
  width: 100%;
  display: flex;
  flex-direction: row;
  flex-wrap: wrap;
  justify-content: space-between;
  align-items: center;

  span {
    height: 100%;
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: center;
  }
}

a {
  img {
    max-height: 3rem;
  }
}
</style>
