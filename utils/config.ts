import type { z } from "zod";
import { parse as parseToml } from "smol-toml";
import { zodErrorMap } from "zod-error-utils";

type ResolvedConfig = z.infer<typeof MOSAIC_SCHEMA>;

type ConfigResult =
  | {
    type: "resolved";
    content: ResolvedConfig;
    external: boolean;
    path: string;
  }
  | { type: "error"; message: string; details?: unknown }
  | { type: "not_found" };

export interface ResolveConfigOptions {
  owner: string;
  name: string;
}

export async function resolveConfig(
  owner: string,
  repository: string,
): Promise<ConfigResult> {
  if (!owner || !repository) {
    return { type: "not_found" };
  }

  try {
    let external = false;

    let url = new URL(
      `https://api.github.com/repos/${owner}/${repository}/contents/.github/mosaic.toml`,
    );

    if (owner !== "luxass") {
      external = true;

      // when the owner is not luxass, resolve the repository externally
      // every external repository that should be resolved, requires
      // a `mosaic.toml` file in luxass/luxass repository
      // the path for these files should be .github/mosaic/<external-owner>/<external-repository>.toml
      // for example: .github/mosaic/vercel/next.js.toml
      if (repository.endsWith(".toml")) {
        repository = repository.slice(0, -5);
      }

      url = new URL(
        `https://api.github.com/repos/luxass/luxass/contents/.github/mosaic/${owner.toLowerCase()}/${repository.toLowerCase()}.toml`,
      );
    }

    const runtimeConfig = useRuntimeConfig();

    const result = await fetch(url, {
      headers: {
        "Authorization": `Bearer ${runtimeConfig.github.token}`,
        "Content-Type": "application/vnd.github+json",
        "X-GitHub-Api-Version": "2022-11-28",
      },
    }).then((res) => res.json());

    if (
      !result
      || typeof result !== "object"
      || !("content" in result)
      || typeof result.content !== "string"
    ) {
      return {
        type: "error",
        message: "error resolving config due to config not being found",
      };
    }

    const content = parseToml(base64ToString(result.content));

    const parsed = await MOSAIC_SCHEMA.safeParseAsync(content, {
      errorMap: zodErrorMap,
    });

    if (!parsed.success) {
      console.error(parsed.error);
      return {
        type: "error",
        message: "error resolving config due to config not being valid",
        details: parsed.error,
      };
    }

    const config = parsed.data;
    if (config.workspace?.enabled && config.workspace.overrides != null) {
      for (const [key] of Object.entries(config.workspace.overrides)) {
        const projectOverride = config.workspace.overrides[key];
        if (projectOverride == null) {
          throw new Error("project not found, how did this happen?");
        }

        projectOverride.project.name = key;
      }
    }

    return {
      type: "resolved",
      content: config,
      external,
      path: `https://github.com/${owner}/${repository}/blob/main/.github/mosaic.toml`,
    };
  } catch (err) {
    console.error(err);

    return {
      type: "error",
      message: "error resolving config due to an internal error",
      details: err,
    };
  }
}
