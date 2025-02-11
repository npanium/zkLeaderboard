import type { Metadata } from "next";
import "./globals.css";
import "./arcade_machine.css";
import { poppins } from "@/lib/fonts";

export const metadata: Metadata = {
  title: "zkLeaderboard",
  description: "Demo of zkLeaderboard and betting game tools",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body className={`dark ${poppins.className}`}>{children}</body>
    </html>
  );
}
