// https://nitro.unjs.io/config
export default defineNitroConfig({
  renderer: "./renderer",
  runtimeConfig: {
    github: {
      token: "",
      username: "luxass",
    },
    siteUrl: process.env.DEPLOY_URL ? process.env.URL : "http://localhost:3000",
  },
  errorHandler: "~/error-handler",
});
