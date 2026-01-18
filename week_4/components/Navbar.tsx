"use client";
import Link from "next/link";
import React from "react";
import { usePathname } from "next/navigation";

const Navbar = () => {
  const pathname = usePathname();
  const isActive = (href: string) => pathname === href;

  const navItems = [
    { href: "/csr", label: "CSR" },
    { href: "/ssr", label: "SSR" },
    { href: "/ssg", label: "SSG" },
    { href: "/isr", label: "ISR" },
  ];

  return (
    <nav className="w-full border-b border-neutral-900/50 bg-black/70 backdrop-blur-xl">
      <div className="mx-auto max-w-5xl px-6">
        <div className="flex h-16 items-center justify-between">
          <Link href="/" className="group flex items-center gap-2">
            <span className="text-sm font-light tracking-wide text-neutral-500 transition-colors group-hover:text-white">
              rendering methods
            </span>
          </Link>

          <div className="flex items-center gap-1">
            {navItems.map((item) => (
              <Link
                key={item.href}
                href={item.href}
                className={`px-3.5 py-2 text-xs font-mono tracking-wider transition-colors ${
                  isActive(item.href)
                    ? "text-white"
                    : "text-neutral-600 hover:text-neutral-400"
                }`}
              >
                {item.label}
              </Link>
            ))}
          </div>
        </div>
      </div>
    </nav>
  );
};

export default Navbar;
