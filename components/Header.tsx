import Link from "next/link";

export function Header() {
  return (
    <header>
      <nav className="flex flex-wrap items-center justify-between">
        <Link href="/" className="flex items-center gap-2">
          <span className="i-twemoji-clipboard text-2xl"></span>
          <h1>projectrc.luxass.dev</h1>
        </Link>

        <div className="flex items-center justify-between gap-2">
          <Link href="https://github.com/luxass/projectrc.luxass.dev" target="_blank" rel="noopener noreferrer" title="GitHub" aria-label="GitHub">
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 16 16">
              <path
                fill="currentColor"
                d="M8 0c4.42 0 8 3.58 8 8a8.013 8.013 0 0 1-5.45 7.59c-.4.08-.55-.17-.55-.38c0-.27.01-1.13.01-2.2c0-.75-.25-1.23-.54-1.48c1.78-.2 3.65-.88 3.65-3.95c0-.88-.31-1.59-.82-2.15c.08-.2.36-1.02-.08-2.12c0 0-.67-.22-2.2.82c-.64-.18-1.32-.27-2-.27c-.68 0-1.36.09-2 .27c-1.53-1.03-2.2-.82-2.2-.82c-.44 1.1-.16 1.92-.08 2.12c-.51.56-.82 1.28-.82 2.15c0 3.06 1.86 3.75 3.64 3.95c-.23.2-.44.55-.51 1.07c-.46.21-1.61.55-2.33-.66c-.15-.24-.6-.83-1.23-.82c-.67.01-.27.38.01.53c.34.19.73.9.82 1.13c.16.45.68 1.31 2.69.94c0 .67.01 1.3.01 1.49c0 .21-.15.45-.55.38A7.995 7.995 0 0 1 0 8c0-4.42 3.58-8 8-8Z"
              >
              </path>
            </svg>
          </Link>

          <button id="theme-toggle" className="op-50 hover:op-75 ml-1 flex items-center justify-center text-lg" title="Toggle Theme" aria-label="Toggle Theme">
            <span className="i-carbon-sun block dark:i-carbon-moon"></span>
          </button>
        </div>
      </nav>
    </header>

  );
}
