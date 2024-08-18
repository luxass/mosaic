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

      let events = data
        .filter((event) => event.type === "PushEvent")
        .flatMap((event) => {
          const payload: any = event.payload || {};
          return (payload.commits || []).map((commit: any) => {
            const title = (commit?.message || "").split("\n")[0];
            const version
              = title.match(/v?(\d+\.\d+\.\d+(?:-[\w.]+)?)(?:\s|$)/)?.[1] || "";
            return {
              id: event.id,
              type: event.type!,
              repo: event.repo.name,
              title,
              sha: commit?.sha || "",
              commit: `https://github.com/${event.repo.name}/commit/${commit?.sha}`,
              created_at: event.created_at!,
              version,
            };
          });
        })
        .filter((event) => event.title.includes("release") && event.version);

      events.sort(
        (a, b) =>
          new Date(a.created_at).getTime() - new Date(b.created_at).getTime(),
      );

      events = events.filter((info, index) => {
        const next = events[index + 1];
        if (next && info.repo === next.repo) {
          return false;
        }
        return true;
      });

      events.reverse();

      return events;
    },
    {
      maxAge: 60 * 5, // 5 minutes
      swr: true,
    },
  );
});
