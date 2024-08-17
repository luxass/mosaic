import { z } from "zod";
import type { ResolvedProject } from "~/types";

export default defineLazyEventHandler(async () => {
  const runtimeConfig = useRuntimeConfig();
  return defineCachedEventHandler(async (event) => {
    const username = getRouterParam(event, "username");
    const repositoryName = getRouterParam(event, "repositoryName");

    if (!username || !repositoryName) {
      throw createError({
        status: 400,
        message: "missing params",
      });
    }

    const resolvedMosaicConfig = await resolveMosaicConfig(username, repositoryName);

    if (!resolvedMosaicConfig || resolvedMosaicConfig.type === "not_found") {
      throw createError({
        status: 404,
        message: "repository has no config defined",
      });
    }

    if (resolvedMosaicConfig.type === "error") {
      throw createError({
        status: 400,
        message: `could not resolve config for ${username}/${repositoryName} due to config not being valid`,
        data: resolvedMosaicConfig.details,
      });
    }

    const repository = await getRepository(username, repositoryName);

    if (!repository) {
      throw createError({
        status: 404,
        message: "repository not found",
      });
    }

    const config = resolvedMosaicConfig.content;

    if (config.project.ignore) {
      throw createError({
        status: 404,
        message: "repository is ignored",
      });
    }

    const projects: ResolvedProject[] = [];

    if (config.workspace && config.workspace.enabled) {
    // TODO
      throw new Error("workspace is not implemented yet");
    } else {
      const project: ResolvedProject = {
        name: repository.name,
        ignore: config.project.ignore || false,
        priority: config.project.priority || 0,
        readme: config.readme
          ? `${runtimeConfig.siteUrl}/api/v1/mosaic/${username}/${repositoryName}/readme${typeof config.readme === "string" ? `/${config.readme}` : ""
          }`
          : undefined,
        deprecated: config.deprecated,
        stars: config.project.stars ? repository.stargazerCount : undefined,
        description: config.project.description || repository.description || undefined,
      };
      if (config.website?.enabled) {
        let website;

        if (typeof config.website.url === "string") {
          website = config.website.url;
        } else {
          website = repository.homepageUrl || null;
        }

        project.website = {
          url: website,
          title: config.website.title || repository.name,
          description: config.website.description || project.description || undefined,
          keywords: config.website.keywords || undefined,
        };
      }

      if (config.npm?.enabled) {
        if (config.npm.name) {
          project.npm = {
            name: config.npm.name,
            url: `https://www.npmjs.com/package/${config.npm.name}`,
          };
        } else {
          const pkg = await getPackage(username, repositoryName);

          if (!pkg.name) {
            throw new Error("no name found in package.json");
          }

          project.npm = {
            name: pkg.name,
            url: `https://www.npmjs.com/package/${pkg.name}`,
          };

          if (config.npm.downloads && project.npm?.name) {
            const result = await fetch(`https://api.npmjs.org/downloads/point/last-month/${project.npm.name}`).then(
              (res) => res.json(),
            );

            if (
              !result
              || typeof result !== "object"
              || !("downloads" in result)
              || typeof result.downloads !== "number"
            ) {
              console.warn(
                `npm downloads is enabled, but no \`downloads\` field was found in the npm API response.\nPlease try again later.`,
              );
            }

            project.npm.downloads = result.downloads;
          }
        }
      }

      if (config.project.version) {
        const latestReleaseResponse = await fetch(
          `https://api.github.com/repos/${username}/${repositoryName}/releases/latest`,
        );
        const pkg = await getPackage(username, repositoryName);

        if (!latestReleaseResponse.ok && !pkg.version) {
          throw new Error(`could not find latest release on github and no version was found in package.json in ${repository.name}`);
        }

        if (!latestReleaseResponse.ok && pkg.version) {
          console.warn("no latest release found on github");
          const npmResult = await fetch(`https://registry.npmjs.org/${pkg.name}`).then((res) => res.json());

          if (!npmResult || typeof npmResult !== "object") {
            throw new Error("version is enabled, but no npm API response was found.\nPlease try again later.");
          }

          const test = z.object({
            "dist-tags": z.object({
              latest: z.string(),
            }),
          });

          const npm = await test.parseAsync(npmResult);

          const latestVersion = npm["dist-tags"].latest;

          project.version = latestVersion || pkg.version;
        } else {
          const result = await latestReleaseResponse.json();

          if (!result || typeof result !== "object" || !("tag_name" in result) || typeof result.tag_name !== "string") {
            throw new Error(
              "version is enabled, but no `tag_name` field was found in the GitHub API response.\nPlease try again later.",
            );
          }

          project.version = result.tag_name;
        }
      }

      projects.push(project);
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
