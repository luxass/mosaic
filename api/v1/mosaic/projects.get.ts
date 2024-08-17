import { Octokit } from "@octokit/rest";

export default defineLazyEventHandler(async () => {
  const runtimeConfig = useRuntimeConfig();
  const octokit = new Octokit({
    auth: runtimeConfig.github.token,
  });

  return defineCachedEventHandler(async () => {
    const allRepositories = await Promise.all([
      octokit.request("GET /users/{username}/repos", {
        username: runtimeConfig.github.username,
        per_page: 10,
        page: 1,
      }),
      octokit.request("GET /users/{username}/repos", {
        username: runtimeConfig.github.username,
        per_page: 10,
        page: 2,
      }),
      octokit.request("GET /users/{username}/repos", {
        username: runtimeConfig.github.username,
        per_page: 10,
        page: 3,
      }),
      octokit.request("GET /users/{username}/repos", {
        username: runtimeConfig.github.username,
        per_page: 10,
        page: 4,
      }),
      octokit.request("GET /users/{username}/repos", {
        username: runtimeConfig.github.username,
        per_page: 10,
        page: 5,
      }),
      octokit.request("GET /users/{username}/repos", {
        username: runtimeConfig.github.username,
        per_page: 10,
        page: 6,
      }),
      octokit.request("GET /users/{username}/repos", {
        username: runtimeConfig.github.username,
        per_page: 10,
        page: 7,
      }),
      octokit.request("GET /users/{username}/repos", {
        username: runtimeConfig.github.username,
        per_page: 10,
        page: 8,
      }),
      octokit.request("GET /users/{username}/repos", {
        username: runtimeConfig.github.username,
        per_page: 10,
        page: 9,
      }),
      octokit.request("GET /users/{username}/repos", {
        username: runtimeConfig.github.username,
        per_page: 10,
        page: 10,
      }),
    ]);

    const data = allRepositories.flatMap((res) => res.data);

    const projects: string[] = [];

    const ignoreFile = await fetch(
      "https://raw.githubusercontent.com/luxass/luxass/main/.github/mosaic/.mosaicignore",
    ).then((res) => res.text());
    const ignore = ignoreFile
      .split("\n")
      .map((line) => line.trim())
      .filter((line) => line && !line.startsWith("#"));

    const repositories = data.filter((repo) => {
      return (
        !!repo
        && !repo.fork
        && !repo.private
        && !repo.archived
        && !ignore.includes(repo.full_name)
        && !ignore.includes(repo.name)
      );
    });

    for await (const file of getExternalRepositories()) {
      if (file.endsWith("README.md") || file.endsWith(".mosaicignore")) continue;

      projects.push(file.replace(".github/mosaic/", "").replace(".toml", ""));
    }

    for (const repo of repositories) {
      projects.push(repo.full_name);
    }

    return projects;
  }, {
    maxAge: 60 * 60, // 1 hour
    swr: true,
    shouldBypassCache() {
      return import.meta.dev;
    },
  });
});
