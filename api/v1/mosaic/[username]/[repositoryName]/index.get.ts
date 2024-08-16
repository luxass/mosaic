export default defineEventHandler(async (event) => {
  const username = getRouterParam(event, "username");
  const repositoryName = getRouterParam(event, "repositoryName");

  if (!username || !repositoryName) {
    return new Response("missing params", {
      status: 400,
    });
  }

  return {};
});
