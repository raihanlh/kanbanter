import { Inter } from "next/font/google";
import "./globals.css";
import React from "react";
import { Navbar } from "@/components/navigation/Navbar";

const inter = Inter({ subsets: ["latin"] });

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className={inter.className}>
        <Navbar
          items={[
            {
              text: "Board",
              href: "/",
            },
            {
              text: "Archive",
              href: "/archive",
            },
          ]}
        />
        {children}
      </body>
    </html>
  );
}
