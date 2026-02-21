'use client';

import { useState } from 'react';
import { MainLayout } from '@/components/layout/MainLayout';
import { GeneratorPanel } from '@/components/features/GeneratorPanel';
import { GalleryPanel } from '@/components/features/GalleryPanel';
import { GPUMonitor } from '@/components/features/GPUMonitor';
import { SettingsPanel } from '@/components/features/SettingsPanel';

type TabId = 'generate' | 'gallery' | 'gpu' | 'settings';

export default function HomePage() {
  const [activeTab, setActiveTab] = useState<TabId>('generate');

  const renderContent = () => {
    switch (activeTab) {
      case 'generate':
        return <GeneratorPanel />;
      case 'gallery':
        return <GalleryPanel />;
      case 'gpu':
        return <GPUMonitor />;
      case 'settings':
        return <SettingsPanel />;
      default:
        return <GeneratorPanel />;
    }
  };

  return (
    <MainLayout activeTab={activeTab} onTabChange={setActiveTab}>
      {renderContent()}
    </MainLayout>
  );
}
