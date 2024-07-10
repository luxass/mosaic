import { z } from "zod";

const PROJECT_SCHEMA = z.object({
  name: z.string({
    description: "the name of the project.",
  }),
  priority: z.number({
    description: "the priority of the project. the higher the number, the higher position the project will have on `luxass.dev`.",
  }).default(10),

  version: z.boolean({
    description: "infer the version of the project from the repository.",
  }).default(false),

  ignore: z.boolean({
    description: "ignore the project from being displayed on `luxass.dev`.",
  }).default(false),
});

const NPM_SCHEMA = z.object({
  enabled: z.boolean({
    description: "npm package information.",
  }),

  name: z.string({
    description: "the name of the npm package. by default the `name` will be auto-inferred from `package.json`",
  }).optional(),

  downloads: z.boolean({
    description: "include the npm package downloads",
  }).default(false),
});

const README_SCHEMA = z.object({
  enabled: z.boolean({
    description: "include the readme file of the repository.",
  }),
  path: z.string({
    description: "the path to the readme file. by default the `path` will be auto-inferred from the repository.",
  }).optional(),
});

const WEBSITE_SCHEMA = z.object({
  enabled: z.boolean({
    description: "include the website information.",
  }),
  url: z.string({
    description: "the url of the website.",
  }).optional(),
  title: z.string({
    description: "the title of the website.",
  }).optional(),
  description: z.string({
    description: "the description of the website.",
  }).optional(),
  keywords: z.array(z.string()).optional(),
});

const WORKSPACE_SCHEMA = z.object({
  enabled: z.boolean({
    description: "include the workspace information.",
  }),
});

export const MOSAIC_SCHEMA = z.object({
  project: PROJECT_SCHEMA,
  npm: NPM_SCHEMA.optional(),
  readme: README_SCHEMA.optional(),
  website: WEBSITE_SCHEMA.optional(),

  workspace: WORKSPACE_SCHEMA.optional(),
});
