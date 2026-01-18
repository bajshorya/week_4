"use client";
import React, { useEffect, useState } from "react";

export default function CSR() {
  const [data, setData] = useState(null);
  const [fetchTime, setFetchTime] = useState<number | null>(null);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    const start = Date.now();
    fetch("/api/data")
      .then((res) => res.json())
      .then((result) => {
        setData(result);
        setFetchTime(Date.now() - start);
        setIsLoading(false);
      })
      .catch(() => setIsLoading(false));
  }, []);

  return (
    <main className="min-h-screen bg-black pt-16">
      <div className="mx-auto max-w-4xl px-6 py-24">
        <div className="mb-16">
          <div className="mb-3 flex items-center gap-2">
            <div className="h-px w-8 bg-neutral-800"></div>
            <span className="text-[10px] font-mono tracking-wider text-neutral-700">
              CSR
            </span>
          </div>
          <h1 className="mb-4 text-5xl font-extralight tracking-tight text-white">
            Client-Side Rendering
          </h1>
          <p className="text-neutral-500 max-w-2xl leading-relaxed">
            Content is rendered in the browser using JavaScript. This approach
            provides high interactivity but requires the client to fetch and
            process data.
          </p>
        </div>

        <div className="mb-16 grid grid-cols-3 gap-3">
          <div className="border border-neutral-900/50 bg-neutral-950/20 p-6">
            <div className="mb-3 text-[10px] font-mono tracking-wider text-neutral-700">
              METHOD
            </div>
            <p className="text-sm font-light text-neutral-400">
              Browser Render
            </p>
          </div>
          <div className="border border-neutral-900/50 bg-neutral-950/20 p-6">
            <div className="mb-3 text-[10px] font-mono tracking-wider text-neutral-700">
              USE CASE
            </div>
            <p className="text-sm font-light text-neutral-400">
              Interactive SPAs
            </p>
          </div>
          <div className="border border-neutral-900/50 bg-neutral-950/20 p-6">
            <div className="mb-3 text-[10px] font-mono tracking-wider text-neutral-700">
              TIMING
            </div>
            <p className="text-sm font-light text-neutral-400">Runtime</p>
          </div>
        </div>

        <div>
          <div className="mb-6 flex items-center gap-2">
            <div className="h-px w-8 bg-neutral-800"></div>
            <h2 className="text-[10px] font-mono tracking-wider text-neutral-700">
              FETCHED DATA
            </h2>
          </div>
          {isLoading ? (
            <div className="flex h-80 items-center justify-center border border-neutral-900/50 bg-neutral-950/20">
              <div className="flex flex-col items-center gap-4">
                <div className="flex gap-1.5">
                  <div
                    className="h-1.5 w-1.5 animate-pulse rounded-full bg-neutral-800"
                    style={{ animationDelay: "0s" }}
                  />
                  <div
                    className="h-1.5 w-1.5 animate-pulse rounded-full bg-neutral-800"
                    style={{ animationDelay: "0.2s" }}
                  />
                  <div
                    className="h-1.5 w-1.5 animate-pulse rounded-full bg-neutral-800"
                    style={{ animationDelay: "0.4s" }}
                  />
                </div>
                <span className="text-xs font-mono text-neutral-700">
                  Fetching data
                </span>
              </div>
            </div>
          ) : (
            <div className="space-y-3">
              <div className="border border-neutral-900/50 bg-neutral-950/20 p-8">
                <pre className="overflow-auto text-xs leading-relaxed text-neutral-500">
                  {JSON.stringify(data, null, 2)}
                </pre>
              </div>
              {fetchTime !== null && (
                <div className="border border-neutral-900/50 bg-neutral-950/20 p-5">
                  <span className="text-xs font-mono text-neutral-600">
                    Fetch completed in{" "}
                    <span className="text-neutral-500">{fetchTime}ms</span>
                  </span>
                </div>
              )}
            </div>
          )}
        </div>
      </div>
    </main>
  );
}
