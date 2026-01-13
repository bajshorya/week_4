import Link from "next/link";
import React from "react";

const Navbar = () => {
  return (
    <div className="w-full h-[100px] border font-mono flex">
      <div className="w-[200px] h-full flex items-center justify-center text-4xl font-bold">
        <h1>Week_4</h1>
      </div>
      <div className="w-full flex items-center justify-evenly">
        <Link href="/csr" className="hover:cursor-pointer">
          CSR
        </Link>
        <Link href="/ssr" className="hover:cursor-pointer">
          SSR
        </Link>
        <Link href="/ssg" className="hover:cursor-pointer">
          SSG
        </Link>
        <Link href="/isr" className="hover:cursor-pointer">
          ISR
        </Link>
      </div>
    </div>
  );
};

export default Navbar;
