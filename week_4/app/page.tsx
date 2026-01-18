import Link from "next/link";

export default function Home() {
  const renderingMethods = [
    {
      title: "Client-Side Rendering",
      description: "Render content in the browser using JavaScript",
      link: "/csr",
      tag: "CSR",
    },
    {
      title: "Server-Side Rendering",
      description: "Render content on the server for each request",
      link: "/ssr",
      tag: "SSR",
    },
    {
      title: "Static Site Generation",
      description: "Pre-render pages at build time",
      link: "/ssg",
      tag: "SSG",
    },
    {
      title: "Incremental Static Regeneration",
      description: "Combine static generation with on-demand revalidation",
      link: "/isr",
      tag: "ISR",
    },
  ];

  return (
    <main className="min-h-screen bg-black">
      <div className="mx-auto max-w-4xl px-6 pt-32 pb-20">
        <div className="mb-24">
          <div className="mb-3 flex items-center gap-2">
            <div className="h-px w-8 bg-neutral-800"></div>
            <span className="text-xs font-mono text-neutral-600">NEXT.JS</span>
          </div>
          <h1 className="mb-5 text-6xl font-extralight tracking-tight text-white">
            Rendering Methods
          </h1>
          <p className="text-base text-neutral-500 max-w-lg">
            Explore different rendering strategies in Next.js and understand how
            each approach impacts performance and user experience
          </p>
        </div>

        <div className="space-y-2">
          {renderingMethods.map((method, index) => (
            <Link key={index} href={method.link} className="group block">
              <div className="relative flex items-center justify-between border border-neutral-900/50 bg-neutral-950/20 p-8 transition-all duration-300 hover:border-neutral-800 hover:bg-neutral-950/40">
                <div className="absolute left-0 top-0 h-full w-0.5 bg-white opacity-0 transition-opacity duration-300 group-hover:opacity-100"></div>
                <div className="flex-1">
                  <div className="mb-2 flex items-center gap-3">
                    <span className="text-[10px] font-mono tracking-wider text-neutral-700">
                      {method.tag}
                    </span>
                  </div>
                  <h3 className="mb-2 text-xl font-light text-white transition-colors group-hover:text-neutral-200">
                    {method.title}
                  </h3>
                  <p className="text-sm leading-relaxed text-neutral-600 transition-colors group-hover:text-neutral-500">
                    {method.description}
                  </p>
                </div>
                <div className="ml-8 text-neutral-800 transition-all duration-300 group-hover:translate-x-1 group-hover:text-neutral-500">
                  <svg
                    width="20"
                    height="20"
                    viewBox="0 0 20 20"
                    fill="none"
                    xmlns="http://www.w3.org/2000/svg"
                  >
                    <path
                      d="M7.5 15L12.5 10L7.5 5"
                      stroke="currentColor"
                      strokeWidth="1.5"
                      strokeLinecap="round"
                      strokeLinejoin="round"
                    />
                  </svg>
                </div>
              </div>
            </Link>
          ))}
        </div>
      </div>
    </main>
  );
}
