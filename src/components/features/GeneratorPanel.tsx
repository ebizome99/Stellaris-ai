'use client';

import { useState } from 'react';
import { ArrowSync20Regular, Image20Regular } from '@fluentui/react-icons';

interface GenerationParams {
  prompt: string;
  negativePrompt: string;
  width: number;
  height: number;
  steps: number;
  cfgScale: number;
  seed: number | null;
  batchSize: number;
  model: string;
}

const defaultParams: GenerationParams = {
  prompt: '',
  negativePrompt: '',
  width: 1024,
  height: 1024,
  steps: 20,
  cfgScale: 7,
  seed: null,
  batchSize: 1,
  model: 'sdxl',
};

const presets = [
  { label: '1:1', width: 1024, height: 1024 },
  { label: '3:4', width: 896, height: 1152 },
  { label: '9:16', width: 768, height: 1344 },
  { label: '16:9', width: 1344, height: 768 },
];

export function GeneratorPanel() {
  const [params, setParams] = useState<GenerationParams>(defaultParams);
  const [isGenerating, setIsGenerating] = useState(false);
  const [progress, setProgress] = useState(0);

  const handleGenerate = async () => {
    if (!params.prompt.trim()) return;
    
    setIsGenerating(true);
    setProgress(0);

    for (let i = 0; i <= 100; i += 10) {
      await new Promise((resolve) => setTimeout(resolve, 200));
      setProgress(i);
    }

    setIsGenerating(false);
    setProgress(0);
  };

  const handleResolutionPreset = (width: number, height: number) => {
    setParams((prev) => ({ ...prev, width, height }));
  };

  return (
    <div className="grid grid-cols-1 lg:grid-cols-2 gap-6 h-full">
      <div className="card p-6 space-y-6 overflow-auto">
        <div>
          <h2 className="text-lg font-semibold text-gray-900 dark:text-gray-100 mb-4">生成参数</h2>
          
          <div className="space-y-4">
            <div>
              <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                提示词
              </label>
              <textarea
                value={params.prompt}
                onChange={(e) => setParams((p) => ({ ...p, prompt: e.target.value }))}
                className="input-field min-h-[100px] resize-none"
                placeholder="描述你想要生成的图像..."
              />
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                负面提示词
              </label>
              <textarea
                value={params.negativePrompt}
                onChange={(e) => setParams((p) => ({ ...p, negativePrompt: e.target.value }))}
                className="input-field min-h-[60px] resize-none"
                placeholder="不想出现的元素..."
              />
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                分辨率
              </label>
              <div className="flex gap-2 mb-2">
                {presets.map((preset) => (
                  <button
                    key={preset.label}
                    onClick={() => handleResolutionPreset(preset.width, preset.height)}
                    className={`px-3 py-1 text-sm rounded-md transition-colors ${
                      params.width === preset.width && params.height === preset.height
                        ? 'bg-primary-600 text-white'
                        : 'bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-600'
                    }`}
                  >
                    {preset.label}
                  </button>
                ))}
              </div>
              <div className="flex gap-2">
                <input
                  type="number"
                  value={params.width}
                  onChange={(e) => setParams((p) => ({ ...p, width: parseInt(e.target.value) || 512 }))}
                  className="input-field"
                  placeholder="宽度"
                />
                <span className="flex items-center text-gray-500">×</span>
                <input
                  type="number"
                  value={params.height}
                  onChange={(e) => setParams((p) => ({ ...p, height: parseInt(e.target.value) || 512 }))}
                  className="input-field"
                  placeholder="高度"
                />
              </div>
            </div>

            <div className="grid grid-cols-2 gap-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                  步数: {params.steps}
                </label>
                <input
                  type="range"
                  min="1"
                  max="50"
                  value={params.steps}
                  onChange={(e) => setParams((p) => ({ ...p, steps: parseInt(e.target.value) }))}
                  className="w-full"
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                  CFG Scale: {params.cfgScale}
                </label>
                <input
                  type="range"
                  min="1"
                  max="20"
                  step="0.5"
                  value={params.cfgScale}
                  onChange={(e) => setParams((p) => ({ ...p, cfgScale: parseFloat(e.target.value) }))}
                  className="w-full"
                />
              </div>
            </div>

            <div className="grid grid-cols-2 gap-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                  种子 (留空随机)
                </label>
                <input
                  type="number"
                  value={params.seed ?? ''}
                  onChange={(e) =>
                    setParams((p) => ({
                      ...p,
                      seed: e.target.value ? parseInt(e.target.value) : null,
                    }))
                  }
                  className="input-field"
                  placeholder="随机"
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                  批量大小
                </label>
                <select
                  value={params.batchSize}
                  onChange={(e) => setParams((p) => ({ ...p, batchSize: parseInt(e.target.value) }))}
                  className="input-field"
                >
                  {[1, 2, 3, 4].map((n) => (
                    <option key={n} value={n}>
                      {n}
                    </option>
                  ))}
                </select>
              </div>
            </div>
          </div>
        </div>

        <button
          onClick={handleGenerate}
          disabled={isGenerating || !params.prompt.trim()}
          className="btn-primary w-full flex items-center justify-center gap-2 disabled:opacity-50 disabled:cursor-not-allowed"
        >
          {isGenerating ? (
            <>
              <ArrowSync20Regular className="w-5 h-5 animate-spin" />
              生成中... {progress}%
            </>
          ) : (
            <>
              <Image20Regular className="w-5 h-5" />
              开始生成
            </>
          )}
        </button>

        {isGenerating && (
          <div className="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2">
            <div
              className="bg-primary-600 h-2 rounded-full transition-all duration-300"
              style={{ width: `${progress}%` }}
            />
          </div>
        )}
      </div>

      <div className="card p-6">
        <h2 className="text-lg font-semibold text-gray-900 dark:text-gray-100 mb-4">预览</h2>
        <div className="aspect-square bg-gray-100 dark:bg-gray-700 rounded-lg flex items-center justify-center">
          <div className="text-center text-gray-500 dark:text-gray-400">
            <Image20Regular className="w-16 h-16 mx-auto mb-2 opacity-50" />
            <p>生成的图像将显示在这里</p>
          </div>
        </div>
      </div>
    </div>
  );
}
