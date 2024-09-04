import type { Metadata } from "next";
import "./style.css";

export const metadata: Metadata = {
    icons: "./logo.ico",
    title: "节点网络",
};

export default function RootLayout({
    children,
}: Readonly<{
    children: React.ReactNode;
}>) {
    return (
        <html lang="en">
            <body>{children}</body>
        </html>
    );
}
