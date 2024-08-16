// https://nitro.unjs.io/config
export default defineNitroConfig({
  srcDir: "server",
  preset: "netlify-edge",
  runtimeConfig: {
    github: {
      token: "",
      username: "luxass",
    },
  },
});
