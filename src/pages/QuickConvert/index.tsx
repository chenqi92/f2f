import { useState } from "react";
import { FileDropzone } from "@/components/FileDropzone";
import { ArrowRight, Loader2 } from "lucide-react";
import { planTargets, createJob } from "@/lib/tauri";
import type { ConversionTarget } from "@/types/ipc";

export function QuickConvert() {
  const [files, setFiles] = useState<File[]>([]);
  const [targets, setTargets] = useState<ConversionTarget[]>([]);
  const [selectedTarget, setSelectedTarget] = useState<string>("");
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleFilesSelected = async (selectedFiles: File[]) => {
    setFiles(selectedFiles);
    setError(null);

    if (selectedFiles.length > 0) {
      try {
        setIsLoading(true);
        // 获取第一个文件的可转换目标
        const firstFile = selectedFiles[0];
        if (!firstFile) return;

        const availableTargets = await planTargets(firstFile.name);
        setTargets(availableTargets);

        // 自动选择第一个推荐的目标
        const recommended = availableTargets.find(t => t.recommended);
        if (recommended) {
          setSelectedTarget(recommended.format);
        } else if (availableTargets.length > 0) {
          const firstTarget = availableTargets[0];
          if (firstTarget) {
            setSelectedTarget(firstTarget.format);
          }
        }
      } catch (err) {
        setError(`获取转换目标失败: ${err}`);
        console.error("Failed to plan targets:", err);
      } finally {
        setIsLoading(false);
      }
    }
  };

  const handleStartConversion = async () => {
    if (!selectedTarget || files.length === 0) {
      setError("请选择文件和目标格式");
      return;
    }

    try {
      setIsLoading(true);
      setError(null);

      const jobId = await createJob(
        files.map(f => f.name),
        selectedTarget,
        undefined,
        { quality_priority: "balanced" }
      );

      console.log("Job created:", jobId);
      // TODO: 跳转到队列页面或显示进度
    } catch (err) {
      setError(`创建任务失败: ${err}`);
      console.error("Failed to create job:", err);
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="p-8 max-w-4xl mx-auto">
      <div className="mb-8">
        <h2 className="text-3xl font-bold mb-2">快速转换</h2>
        <p className="text-muted-foreground">
          上传文件，选择目标格式，一键转换
        </p>
      </div>

      <div className="space-y-6">
        {/* 文件上传区域 */}
        <section>
          <h3 className="text-sm font-medium mb-3">1. 选择文件</h3>
          <FileDropzone
            onFilesSelected={handleFilesSelected}
            multiple={false}
            maxSize={500}
          />
        </section>

        {/* 目标格式选择 */}
        {targets.length > 0 && (
          <section>
            <h3 className="text-sm font-medium mb-3">2. 选择目标格式</h3>
            <div className="grid grid-cols-2 md:grid-cols-3 gap-3">
              {targets.map((target) => (
                <button
                  key={target.format}
                  onClick={() => setSelectedTarget(target.format)}
                  disabled={!target.available}
                  className={`
                    p-4 rounded-lg border-2 text-left transition-all
                    ${
                      selectedTarget === target.format
                        ? "border-primary bg-primary/5"
                        : "border-border hover:border-primary/50"
                    }
                    ${
                      !target.available
                        ? "opacity-50 cursor-not-allowed"
                        : "cursor-pointer"
                    }
                  `}
                >
                  <div className="flex items-start justify-between">
                    <div>
                      <p className="font-medium">{target.format.toUpperCase()}</p>
                      <p className="text-xs text-muted-foreground mt-1">
                        {target.mime_type}
                      </p>
                    </div>
                    {target.recommended && (
                      <span className="text-xs bg-primary/10 text-primary px-2 py-1 rounded">
                        推荐
                      </span>
                    )}
                  </div>
                  {!target.available && target.missing_requirements.length > 0 && (
                    <p className="text-xs text-destructive mt-2">
                      需要: {target.missing_requirements.join(", ")}
                    </p>
                  )}
                </button>
              ))}
            </div>
          </section>
        )}

        {/* 错误提示 */}
        {error && (
          <div className="p-4 rounded-lg bg-destructive/10 border border-destructive/20">
            <p className="text-sm text-destructive">{error}</p>
          </div>
        )}

        {/* 开始转换按钮 */}
        {files.length > 0 && selectedTarget && (
          <section>
            <button
              onClick={handleStartConversion}
              disabled={isLoading || !selectedTarget}
              className="
                w-full p-4 rounded-lg bg-primary text-primary-foreground
                font-medium flex items-center justify-center gap-2
                hover:bg-primary/90 transition-colors
                disabled:opacity-50 disabled:cursor-not-allowed
              "
            >
              {isLoading ? (
                <>
                  <Loader2 className="h-5 w-5 animate-spin" />
                  处理中...
                </>
              ) : (
                <>
                  开始转换
                  <ArrowRight className="h-5 w-5" />
                </>
              )}
            </button>
          </section>
        )}
      </div>
    </div>
  );
}
