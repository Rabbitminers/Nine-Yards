<template lang="pug">
form.projects-section.login-form
  icons-jumping.login-form-graphic(
    :shadowed="true"
  )
  section.login-form-inputs.columns
    header.projects-section-header
        p Welcome!
  
    label.input-bar.rows
      icons-user.input-icon
      input(
        id="username"
        type="text"
        placeholder="Username"
      )
    label.input-bar.rows
      icons-email.input-icon
      input(
        id="email"
        type="text"
        placeholder="Email"
      )
    label.input-bar.rows
      icons-password.input-icon
      input(
        id="password"
        type="password"
        placeholder="Password"
      )
    label.input-bar.rows
      input.login-form-submit(
        type="submit" 
        value="Sign Up!"
      )
    router-link(to="/signin")
      a Already have an account?
</template>

<script lang="ts">
definePageMeta({
  layout: "default",
});

export default {
  name: "signup",
}
</script>

<style lang="scss">
$theme: var(--colour-green);

.login-form {
  display: flex;
  flex-direction: row;
  gap: 1em;

  &-inputs {
    width: 100%;
    margin: auto;
    align-items: center;
    gap: inherit;
  }

  span {
    font-size: large;
  }

  a {
    text-decoration: none;
    color: var(--color-secondary);
  }
}

.sub-card {
  background-color: $theme;
  padding: 1.5em;
  border-radius: 32px;
}

.input-bar {
  @extend .sub-card;
  gap: 0.5em;

  > input {
    background: transparent;
    border: none;
    width: 100%;
    color: var(--color-secondary);

    &:focus {
      outline-color: none;
    }
  }
}

@media screen and (max-width: 540px) {
  .login-form {
    flex-direction: column;
  }
}


.fern-wool {
  fill: $theme;

  &-shadow {
    fill: #fff;
    opacity: 0.53;
  }
}
</style>