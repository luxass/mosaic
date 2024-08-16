export default defineEventHandler(async (event) => {
  const owner = getRouterParam(event, "owner");
  const repositoryName = getRouterParam(event, "repositoryName");

  if (!owner || !repositoryName) {
    return new Response("missing params", {
      status: 400,
    });
  }

  return {};
});
