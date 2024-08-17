import { Octokit } from "@octokit/rest";

export default defineLazyEventHandler(() => {
  const config = useRuntimeConfig();
  const octokit = new Octokit({
    auth: config.github.token,
  });

  return defineCachedEventHandler(
    async () => {
      const { data } = await octokit.request("GET /users/{username}/events", {
        username: config.github.username,
        per_page: 100,
        page: 1,
      });

      const events = data.flatMap((event) => {
        return {
          id: event.id,
          type: event.type,
          created_at: event.created_at,
          repo: event.repo.name,
        };
      });

      return events;
    },
    {
      maxAge: 60 * 5, // 5 minutes
      swr: true,
    },
  );
});
