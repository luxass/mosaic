function html(strings: TemplateStringsArray, ...values: any[]) {
  return String.raw(strings, ...values);
}

export default defineEventHandler((event) => {
  setResponseHeader(event, "Content-Type", "text/html");
  return html/* html */`<!doctype html>
  <html lang="en">
    <head>
      <meta charset="UTF-8" />
      <meta name="viewport" content="width=device-width, initial-scale=1.0" />
      <title>mosaic | luxass.dev</title>
      <script src="https://cdn.tailwindcss.com"></script>
      <style>
        :root {
          --background: 0 0% 100%;
          --foreground: 240 10% 3.9%;
          --card: 0 0% 100%;
          --card-foreground: 240 10% 3.9%;
          --popover: 0 0% 100%;
          --popover-foreground: 240 10% 3.9%;
          --primary: 240 5.9% 10%;
          --primary-foreground: 0 0% 98%;
          --secondary: 240 4.8% 95.9%;
          --secondary-foreground: 240 5.9% 10%;
          --muted: 240 4.8% 95.9%;
          --muted-foreground: 240 3.8% 45%;
          --accent: 240 4.8% 95.9%;
          --accent-foreground: 240 5.9% 10%;
          --destructive: 0 72% 51%;
          --destructive-foreground: 0 0% 98%;
          --border: 240 5.9% 90%;
          --input: 240 5.9% 90%;
          --ring: 240 5.9% 10%;
          --chart-1: 173 58% 39%;
          --chart-2: 12 76% 61%;
          --chart-3: 197 37% 24%;
          --chart-4: 43 74% 66%;
          --chart-5: 27 87% 67%;
          --radius: 0.5rem;
        }
      </style>
      <style>
        h1,
        h2,
        h3,
        h4,
        h5,
        h6 {
          font-family: "Inter", sans-serif;
          --font-sans-serif: "Inter";
        }
      </style>
      <style>
        body {
          font-family: "Inter", sans-serif;
          --font-sans-serif: "Inter";
        }
      </style>
      <script>
        tailwind.config = {
          theme: {
            extend: {
              colors: {
                border: "hsl(var(--border))",
                input: "hsl(var(--input))",
                ring: "hsl(var(--ring))",
                background: "hsl(var(--background))",
                foreground: "hsl(var(--foreground))",
                primary: {
                  DEFAULT: "hsl(var(--primary))",
                  foreground: "hsl(var(--primary-foreground))",
                },
                secondary: {
                  DEFAULT: "hsl(var(--secondary))",
                  foreground: "hsl(var(--secondary-foreground))",
                },
                destructive: {
                  DEFAULT: "hsl(var(--destructive))",
                  foreground: "hsl(var(--destructive-foreground))",
                },
                muted: {
                  DEFAULT: "hsl(var(--muted))",
                  foreground: "hsl(var(--muted-foreground))",
                },
                accent: {
                  DEFAULT: "hsl(var(--accent))",
                  foreground: "hsl(var(--accent-foreground))",
                },
                popover: {
                  DEFAULT: "hsl(var(--popover))",
                  foreground: "hsl(var(--popover-foreground))",
                },
                card: {
                  DEFAULT: "hsl(var(--card))",
                  foreground: "hsl(var(--card-foreground))",
                },
              },
              borderRadius: {
                xl: "calc(var(--radius) + 4px)",
                lg: "var(--radius)",
                md: "calc(var(--radius) - 2px)",
                sm: "calc(var(--radius) - 4px)",
              },
              keyframes: {
                "accordion-down": {
                  from: { height: 0 },
                  to: { height: "var(--radix-accordion-content-height)" },
                },
                "accordion-up": {
                  from: { height: "var(--radix-accordion-content-height)" },
                  to: { height: 0 },
                },
              },
              animation: {
                "accordion-down": "accordion-down 0.2s ease-out",
                "accordion-up": "accordion-up 0.2s ease-out",
              },
            },
          },
        };
      </script>
    </head>
    <body class="bg-gray-100">
      <div
        class="flex flex-col items-center justify-center min-h-dvh bg-background text-foreground"
      >
        <section class="container px-4 py-12 md:py-24 lg:py-32 text-center">
          <div class="max-w-3xl mx-auto space-y-6">
            <h1
              class="text-4xl font-bold tracking-tight sm:text-5xl md:text-6xl"
            >
              MOSAIC
            </h1>
            <p class="text-muted-foreground md:text-xl">
              An Api serving the list of projects and their details.
            </p>
            <a
              href="https://github.com/luxass/mosaic"
              class="inline-flex items-center justify-center rounded-md bg-primary px-6 py-3 text-sm font-medium text-primary-foreground shadow-sm transition-colors hover:bg-primary/90 focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-2"
            >
              View on GitHub
            </a>
          </div>
        </section>
      </div>
    </body>
  </html>`;
});
