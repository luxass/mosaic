export default defineEventHandler(async (event) => {
  const owner = getRouterParam(event, "owner");
  const repositoryName = getRouterParam(event, "repositoryName");

  if (!owner || !repositoryName) {
    return new Response("missing params", {
      status: 400,
    });
  }

  const config = await resolveConfig(owner, repositoryName);

  if (!config || config.type === "not_found") {
    return new Response("repository has no config defined", {
      status: 404,
    });
  }

  if (config.type === "error") {
    return new Response(
      "error resolving config due to config not being valid",
      {
        status: 500,
      },
    );
  }

  return {
    lastModified: new Date().toISOString(),
    content: config.content,
    external: config.external,
    path: config.path,
  };
});
