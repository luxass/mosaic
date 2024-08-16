import type { z } from "zod";
import type { Language, Repository } from "github-schema";
import type {
  DEPRECATED_SCHEMA,
  NPM_SCHEMA,
  PROJECT_SCHEMA,
  WEBSITE_SCHEMA,
} from "./utils/json-schema";

type SafeOmit<T, K extends keyof T> = Omit<T, K>;

export type ResolvedProject = {
  name: string;
  website?: SafeOmit<z.infer<typeof WEBSITE_SCHEMA>, "enabled">;
  npm?: SafeOmit<z.infer<typeof NPM_SCHEMA>, "enabled" | "downloads"> & {
    url?: string;
    downloads?: number;
  };
  deprecated?: z.infer<typeof DEPRECATED_SCHEMA>;
  readme?: string;
} & SafeOmit<z.infer<typeof PROJECT_SCHEMA>, "version" | "stars"> & {
  version?: string;
  stars?: number;
};

export type Project = ResolvedProject &
  Pick<Repository, "nameWithOwner" | "pushedAt" | "url"> & {
    defaultBranch?: string;
    isContributor: boolean;
    language?: Pick<Language, "name" | "color">;
  };

export interface GitTree {
  /** @description The file referenced in the tree. */
  path?: string;
  /**
   * @description The file mode; one of `100644` for file (blob), `100755` for executable (blob), `040000` for subdirectory (tree), `160000` for submodule (commit), or `120000` for a blob that specifies the path of a symlink.
   * @enum {string}
   */
  mode?: "100644" | "100755" | "040000" | "160000" | "120000";
  /**
   * @description Either `blob`, `tree`, or `commit`.
   * @enum {string}
   */
  type?: "blob" | "tree" | "commit";
  /**
   * @description The SHA1 checksum ID of the object in the tree. Also called `tree.sha`. If the value is `null` then the file will be deleted.
   *
   * **Note:** Use either `tree.sha` or `content` to specify the contents of the entry. Using both `tree.sha` and `content` will return an error.
   */
  sha?: string | null;
  /**
   * @description The content you want this file to have. GitHub will write this blob out and use that SHA for this entry. Use either this, or `tree.sha`.
   *
   * **Note:** Use either `tree.sha` or `content` to specify the contents of the entry. Using both `tree.sha` and `content` will return an error.
   */
  content?: string;
};
