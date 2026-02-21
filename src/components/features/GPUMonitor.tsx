'use client';

import { useState, useEffect } from 'react';

interface GPUStatus {
  id: number;
  name: string;
  totalMemoryMb: number;
  usedMemoryMb: number;
  freeMemoryMb: number;
  memoryUtilization: number;
  gpuUtilization: number;
  temperature: number | null;
  activeTasks: number;
  queueLength: number;
  available: boolean;
}

const mockGPUStatus: GPUStatus[] = [
  {
    id: 0,
    name: 'NVIDIA GeForce RTX 4090',
    totalMemoryMb: 24564,
    usedMemoryMb: 8192,
    freeMemoryMb: 16372,
    memoryUtilization: 0.33,
    gpuUtilization: 0.45,
    temperature: 65,
    activeTasks: 2,
    queueLength: 3,
    available: true,
  },
  {
    id: 1,
    name: 'NVIDIA GeForce RTX 4090',
    totalMemoryMb: 24564,
    usedMemoryMb: 4096,
    freeMemoryMb: 20468,
    memoryUtilization: 0.17,
    gpuUtilization: 0.25,
    temperature: 58,
    activeTasks: 1,
    queueLength: 1,
    available: true,
  },
];

export function GPUMonitor() {
  const [gpuStatus, setGPUStatus] = useState<GPUStatus[]>(mockGPUStatus);
  const [isRefreshing, setIsRefreshing] = useState(false);

  const refreshStatus = async () => {
    setIsRefreshing(true);
    await new Promise((resolve) => setTimeout(resolve, 500));
    setGPUStatus((prev) =>
      prev.map((gpu) => ({
        ...gpu,
        memoryUtilization: Math.random() * 0.5 + 0.2,
        gpuUtilization: Math.random() * 0.6 + 0.1,
        temperature: Math.floor(Math.random() * 20 + 55),
      }))
    );
    setIsRefreshing(false);
  };

  useEffect(() => {
    const interval = setInterval(refreshStatus, 5000);
    return () => clearInterval(interval);
  }, []);

  const formatMemory = (mb: number) => {
    if (mb >= 1024) {
      return `${(mb / 1024).toFixed(1)} GB`;
    }
    return `${mb} MB`;
  };

  const totalVRAM = gpuStatus.reduce((sum, gpu) => sum + gpu.totalMemoryMb, 0);
  const usedVRAM = gpuStatus.reduce((sum, gpu) => sum + gpu.usedMemoryMb, 0);
  const totalUtilization = gpuStatus.reduce((sum, gpu) => sum + gpu.gpuUtilization, 0) / gpuStatus.length;

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h2 className="text-lg font-semibold text-gray-900 dark:text-gray-100">GPU 监控</h2>
        <button
          onClick={refreshStatus}
          disabled={isRefreshing}
          className="btn-secondary text-sm"
        >
          {isRefreshing ? '刷新中...' : '刷新'}
        </button>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
        <div className="card p-4">
          <p className="text-sm text-gray-500 dark:text-gray-400">总显存</p>
          <p className="text-2xl font-bold text-gray-900 dark:text-gray-100">{formatMemory(totalVRAM)}</p>
        </div>
        <div className="card p-4">
          <p className="text-sm text-gray-500 dark:text-gray-400">已用显存</p>
          <p className="text-2xl font-bold text-primary-600">{formatMemory(usedVRAM)}</p>
        </div>
        <div className="card p-4">
          <p className="text-sm text-gray-500 dark:text-gray-400">平均利用率</p>
          <p className="text-2xl font-bold text-secondary-blue-500">
            {(totalUtilization * 100).toFixed(1)}%
          </p>
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
        {gpuStatus.map((gpu) => (
          <div key={gpu.id} className="card p-4">
            <div className="flex items-center justify-between mb-4">
              <div>
                <h3 className="font-semibold text-gray-900 dark:text-gray-100">GPU {gpu.id}</h3>
                <p className="text-sm text-gray-500 dark:text-gray-400">{gpu.name}</p>
              </div>
              <span
                className={`px-2 py-1 rounded-full text-xs font-medium ${
                  gpu.available
                    ? 'bg-green-100 text-green-700 dark:bg-green-900/30 dark:text-green-400'
                    : 'bg-red-100 text-red-700 dark:bg-red-900/30 dark:text-red-400'
                }`}
              >
                {gpu.available ? '可用' : '繁忙'}
              </span>
            </div>

            <div className="space-y-3">
              <div>
                <div className="flex justify-between text-sm mb-1">
                  <span className="text-gray-600 dark:text-gray-400">显存使用</span>
                  <span className="text-gray-900 dark:text-gray-100">
                    {formatMemory(gpu.usedMemoryMb)} / {formatMemory(gpu.totalMemoryMb)}
                  </span>
                </div>
                <div className="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2">
                  <div
                    className="bg-primary-600 h-2 rounded-full"
                    style={{ width: `${gpu.memoryUtilization * 100}%` }}
                  />
                </div>
              </div>

              <div>
                <div className="flex justify-between text-sm mb-1">
                  <span className="text-gray-600 dark:text-gray-400">GPU 利用率</span>
                  <span className="text-gray-900 dark:text-gray-100">
                    {(gpu.gpuUtilization * 100).toFixed(1)}%
                  </span>
                </div>
                <div className="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2">
                  <div
                    className="bg-secondary-blue-500 h-2 rounded-full"
                    style={{ width: `${gpu.gpuUtilization * 100}%` }}
                  />
                </div>
              </div>

              <div className="grid grid-cols-3 gap-2 text-sm">
                <div className="text-center p-2 bg-gray-50 dark:bg-gray-700/50 rounded">
                  <p className="text-gray-500 dark:text-gray-400">温度</p>
                  <p className="font-semibold text-gray-900 dark:text-gray-100">
                    {gpu.temperature ? `${gpu.temperature}°C` : 'N/A'}
                  </p>
                </div>
                <div className="text-center p-2 bg-gray-50 dark:bg-gray-700/50 rounded">
                  <p className="text-gray-500 dark:text-gray-400">活跃任务</p>
                  <p className="font-semibold text-gray-900 dark:text-gray-100">{gpu.activeTasks}</p>
                </div>
                <div className="text-center p-2 bg-gray-50 dark:bg-gray-700/50 rounded">
                  <p className="text-gray-500 dark:text-gray-400">队列</p>
                  <p className="font-semibold text-gray-900 dark:text-gray-100">{gpu.queueLength}</p>
                </div>
              </div>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
