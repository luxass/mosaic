export interface READMEOptions {
  owner: string;
  repository: string;
  readmePath?: boolean | string;
}

export type READMEResult =
  | { type: "not_found" }
  | { type: "resolved"; content: string; path: string }
  | { type: "error"; message: string; details?: unknown };

export async function getREADME(
  options: READMEOptions,
): Promise<READMEResult> {
  if (!options.owner || !options.repository) {
    return { type: "error", message: "missing owner or repository" };
  }

  const { owner, repository } = options;

  let { readmePath } = options;

  const readmeUrl = new URL(
    `https://api.github.com/repos/${owner}/${repository}`,
  );

  if (typeof readmePath === "string" && readmePath !== "") {
    if (readmePath.startsWith("/")) {
      readmePath = readmePath.slice(1);
    }

    if (!readmePath.endsWith("README.md")) {
      readmePath += "/README.md";
    }

    readmeUrl.pathname += `/contents/${readmePath}`;
  } else {
    readmeUrl.pathname += "/readme";
  }

  const runtimeConfig = useRuntimeConfig();

  try {
    const result = await fetch(readmeUrl.toString(), {
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
      return { type: "not_found" };
    }

    return {
      type: "resolved",
      content: base64ToString(result.content),
      path: readmeUrl.toString(),
    };
  } catch (err) {
    return {
      type: "error",
      message: "error fetching readme",
      details: err,
    };
  }
}
