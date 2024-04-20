"use client";

import { FC } from "react";
import Link from "next/link";
import { usePathname } from "next/navigation";
import { classNames } from "@/lib/utils";

interface NavbarItem {
  text: string;
  href: string;
}

interface NavbarProps {
  items: NavbarItem[];
}

export const Navbar: FC<NavbarProps> = ({ items }) => {
  const pathname = usePathname();

  return (
    <div className="space-x-5 flex flex-row p-5">
      {items.map((item, index) => (
        <span key={index}>
          <Link href={item.href}>
            <button
              key={index}
              className={classNames(
                "py-1 px-3 rounded ",
                pathname == item.href ? "bg-gray-600 border" : "bg-gray-500"
              )}
            >
              <h1>{item.text}</h1>
            </button>
          </Link>
        </span>
      ))}
    </div>
  );
};
