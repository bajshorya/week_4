export const revalidate = 10;

export default async function ISRPage() {
  const data = await fetch("http://localhost:3000/api/data", {
    next: { revalidate: 10 },
  }).then((res) => res.json());

  return (
    <main className="min-h-screen bg-black pt-16">
      <div className="mx-auto max-w-4xl px-6 py-24">
        <div className="mb-16">
          <div className="mb-3 flex items-center gap-2">
            <div className="h-px w-8 bg-neutral-800"></div>
            <span className="text-[10px] font-mono tracking-wider text-neutral-700">
              ISR
            </span>
          </div>
          <h1 className="mb-4 text-5xl font-extralight tracking-tight text-white">
            Incremental Static Regeneration
          </h1>
          <p className="text-neutral-500 max-w-2xl leading-relaxed">
            Static generation with on-demand revalidation every 10 seconds.
            Combines the speed of static sites with the freshness of server
            rendering.
          </p>
        </div>

        <div className="mb-16 grid grid-cols-3 gap-3">
          <div className="border border-neutral-900/50 bg-neutral-950/20 p-6">
            <div className="mb-3 text-[10px] font-mono tracking-wider text-neutral-700">
              METHOD
            </div>
            <p className="text-sm font-light text-neutral-400">Revalidation</p>
          </div>
          <div className="border border-neutral-900/50 bg-neutral-950/20 p-6">
            <div className="mb-3 text-[10px] font-mono tracking-wider text-neutral-700">
              USE CASE
            </div>
            <p className="text-sm font-light text-neutral-400">Fresh Content</p>
          </div>
          <div className="border border-neutral-900/50 bg-neutral-950/20 p-6">
            <div className="mb-3 text-[10px] font-mono tracking-wider text-neutral-700">
              TIMING
            </div>
            <p className="text-sm font-light text-neutral-400">10s Interval</p>
          </div>
        </div>

        <div>
          <div className="mb-6 flex items-center gap-2">
            <div className="h-px w-8 bg-neutral-800"></div>
            <h2 className="text-[10px] font-mono tracking-wider text-neutral-700">
              FETCHED DATA
            </h2>
          </div>
          <div className="border border-neutral-900/50 bg-neutral-950/20 p-8">
            <pre className="overflow-auto text-xs leading-relaxed text-neutral-500">
              {JSON.stringify(data, null, 2)}
            </pre>
          </div>
        </div>
      </div>
    </main>
  );
}
