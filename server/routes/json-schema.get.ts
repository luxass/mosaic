import { z } from "zod";
import { zodToJsonSchema } from "zod-to-json-schema";

export default defineCachedEventHandler(async (event) => {
  const jsonSchema = zodToJsonSchema(
    MOSAIC_SCHEMA.merge(
      z.object({
        $schema: z
          .string({
            description: "Ignore this, it's just for editors.",
          })
          .default("https://mosaic.luxass.dev/json-schema"),
      }),
    ),
  );

  setResponseHeaders(event, {
    "Content-Type": "application/json",
    "Access-Control-Allow-Origin": "*",
    "Cache-Control": "public, s-maxage=3600, must-revalidate",
    "Content-Disposition": "inline",
  });

  return jsonSchema;
}, {
  maxAge: 60 * 60, // 1 hour
  swr: true,
});
