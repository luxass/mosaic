import { Kind, Type, TypeRegistry } from "@sinclair/typebox";
import addFormats from "ajv-formats";
import Ajv from "ajv";
import type { SchemaOptions, Static, TSchema, TUnion } from "@sinclair/typebox";
import { Value } from "@sinclair/typebox/value";

const ajv = addFormats(new Ajv({}), [
  "date-time",
  "time",
  "date",
  "email",
  "hostname",
  "ipv4",
  "ipv6",
  "uri",
  "uri-reference",
  "uuid",
  "uri-template",
  "json-pointer",
  "relative-json-pointer",
  "regex",
]);

TypeRegistry.Set(
  "ExtendedOneOf",
  (schema: any, value) =>
    schema.oneOf.reduce(
      (acc: number, schema: any) => acc + (Value.Check(schema, value) ? 1 : 0),
      0,
    ) === 1,
);

function OneOf<T extends TSchema[]>(
  oneOf: [...T],
  options: SchemaOptions = {},
) {
  return Type.Unsafe<Static<TUnion<T>>>({
    ...options,
    [Kind]: "ExtendedOneOf",
    oneOf,
  });
}

export const PROJECTRC_TYPEBOX_SCHEMA = Type.Object(
  {
    readme: Type.Optional(
      OneOf(
        [
          Type.Boolean({
            description: "Will use the root readme.md as the repository readme",
            default: false,
          }),
          Type.String({
            description: "Will use the given file as the repository readme",
            default: "README.md",
          }),
        ],
        {
          default: false,
          description:
            "If defined will show the repository readme at luxass.dev/projects/repo-name",
        },
      ),
    ),
    npm: Type.Optional(
      OneOf(
        [
          Type.Boolean({
            description:
              "Will find the first package.json in the repo and use the `name` as the npm package name",
            default: false,
          }),
          Type.String({
            description:
              "Will use the given package.json to find the npm package name",
            default: "package.json",
          }),
        ],
        {
          description:
            "Does the repo have a npm package and should it be visible on luxass.dev?",
          default: false,
        },
      ),
    ),
    ignore: Type.Optional(
      Type.Boolean({
        description: "Ignore this repository from being used",
        default: false,
      }),
    ),
    website: Type.Optional(
      Type.String({
        description: "The url to the website",
        format: "uri",
      }),
    ),
  },
  {
    $schema: "http://json-schema.org/draft-07/schema",
    description:
      "Project configuration file for luxass.dev. See more here https://projectrc.luxass.dev",
  },
);

export const PROJECTRC_VALIDATE = ajv.compile(PROJECTRC_TYPEBOX_SCHEMA);
