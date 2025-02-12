import type { Metadata } from "next";
import "./styles/globals.css";
import { poppins } from "@/lib/fonts";
import { Providers } from "./providers";

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
      <body className={`dark ${poppins.className}`}>
        <Providers>{children}</Providers>
      </body>
    </html>
  );
}
