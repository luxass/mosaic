import { graphql } from "@octokit/graphql";
import type { RepositoryNode } from "github-schema";
import { gql } from "github-schema";
import { Language } from "@luxass/github-languages";

export type RepositoryType = "fork" | "private" | "archived" | "public";

export interface RepositoryTypeOptions {
  owner: string;
  name: string;
}

export async function getRepositoryType(
  owner: string,
  repository: string,
): Promise<RepositoryType | undefined> {
  if (!owner || !repository) {
    return undefined;
  }

  const runtimeConfig = useRuntimeConfig();

  const res = await fetch(
    `https://api.github.com/repos/${owner}/${repository}`,
    {
      headers: {
        "Authorization": `Bearer ${runtimeConfig.github.token}`,
        "Content-Type": "application/vnd.github+json",
        "X-GitHub-Api-Version": "2022-11-28",
      },
    },
  );

  if (!res.ok) {
    return;
  }

  const data = await res.json();

  if (!data || typeof data !== "object") {
    console.error("Invalid response from GitHub API");
    return;
  }

  if ("fork" in data && data.fork === true) {
    return "fork";
  }

  if ("archived" in data && data.archived === true) {
    return "archived";
  }

  if ("private" in data && data.private === false) {
    return "public";
  }

  // always return private, if we can't determine the type
  return "private";
}

const REPOSITORY_QUERY = gql`
  #graphql
  query getRepository($owner: String!, $name: String!) {
    repository(owner: $owner, name: $name) {
      name
      homepageUrl
      isFork
      isPrivate
      nameWithOwner
      description
      pushedAt
      url
      defaultBranchRef {
        name
      }
      stargazerCount
      languages(first: 1, orderBy: { field: SIZE, direction: DESC }) {
        nodes {
          name
          color
        }
      }
    }
  }
`;

export async function getRepository(
  owner: string,
  name: string,
): Promise<RepositoryNode["repository"] | undefined> {
  if (!owner || !name) {
    return undefined;
  }

  const runtimeConfig = useRuntimeConfig();

  const { repository } = await graphql<RepositoryNode>(REPOSITORY_QUERY, {
    headers: {
      "Authorization": `Bearer ${runtimeConfig.github.token}`,
      "Content-Type": "application/vnd.github+json",
      "X-GitHub-Api-Version": "2022-11-28",
    },
    name,
    owner,
  });

  // to prevent returning null from the query
  if (!repository) {
    return undefined;
  }

  return repository;
}

export async function* getExternalRepositories(
  path: string = ".github/mosaic",
): AsyncGenerator<string> {
  try {
    const runtimeConfig = useRuntimeConfig();

    const data = await fetch(
      `https://api.github.com/repos/luxass/luxass/contents/${path}`,
      {
        headers: {
          "Authorization": `Bearer ${runtimeConfig.github.token}`,
          "Content-Type": "application/vnd.github+json",
          "X-GitHub-Api-Version": "2022-11-28",
        },
      },
    ).then((res) => res.json());
    if (Array.isArray(data)) {
      for (const item of data) {
        if (item.type === "file") {
          yield item.path;
        } else if (item.type === "dir") {
          yield * getExternalRepositories(item.path);
        }
      }
    } else {
      throw new TypeError("invalid response from github");
    }
  } catch (error: any) {
    console.error("Error fetching files from GitHub:", error.message);
    throw error;
  }
}

export async function getRepositoryLanguages(
  owner: string,
  repositoryName: string,
): Promise<Record<string, number> | undefined> {
  if (!owner || !repositoryName) {
    return undefined;
  }

  const runtimeConfig = useRuntimeConfig();
  try {
    const data = await fetch(
      `https://api.github.com/repos/${owner}/${repositoryName}/languages`,
      {
        headers: {
          "Authorization": `Bearer ${runtimeConfig.github.token}`,
          "Content-Type": "application/vnd.github+json",
          "X-GitHub-Api-Version": "2022-11-28",
        },
      },
    ).then((res) => res.json());

    if (!data || typeof data !== "object" || "message" in data) {
      return undefined;
    }

    return data;
  } catch (err) {
    console.error("Error fetching languages from github:", err);
    return undefined;
  }
}
