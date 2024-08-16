import { Octokit } from "@octokit/rest";

export default defineLazyEventHandler(async () => {
  const runtimeConfig = useRuntimeConfig();
  const octokit = new Octokit({
    auth: runtimeConfig.github.token,
  });

  return defineCachedEventHandler(async () => {
    const { data } = await octokit.request("GET /users/{username}/repos", {
      username: runtimeConfig.github.username,
      per_page: 100,
      page: 1,
    });

    const projects: {
      name: string;
      nameWithOwner: string;
    }[] = [];

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

      const [owner, name] = file.replace(".github/mosaic/", "").split("/");

      // const { repository } = await graphql<{
      //   repository: Repository;
      // }>(REPOSITORY_QUERY, {
      //   owner,
      //   name: name.replace(".toml", ""),
      //   headers: {
      //     "Authorization": `Bearer ${runtimeConfig.github.token}`,
      //     "Content-Type": "application/json",
      //   },
      // });

      projects.push({
        name: name.replace(".toml", ""),
        nameWithOwner: `${owner}/${name.replace(".toml", "")}`,
      });
    }

    for (const repo of repositories) {
      projects.push({
        name: repo.name,
        nameWithOwner: repo.full_name,
      });
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
