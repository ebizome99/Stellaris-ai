import type { Metadata } from 'next';
import './styles/globals.css';

export const metadata: Metadata = {
  title: 'Stellaris AI - 企业级AI图片生成系统',
  description: '支持本地GPU和云端混合算力的企业级AI图片生成系统',
  authors: [{ name: 'Stellaris AI Team' }],
  keywords: ['AI', '图片生成', 'Stable Diffusion', 'GPU', '企业级'],
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="zh-CN" suppressHydrationWarning>
      <body className="min-h-screen bg-gray-50 dark:bg-gray-900">
        {children}
      </body>
    </html>
  );
}
